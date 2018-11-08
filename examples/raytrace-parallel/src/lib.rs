extern crate futures;
extern crate js_sys;
extern crate raytracer;
extern crate wasm_bindgen;
extern crate web_sys;

use std::cell::RefCell;
use std::cmp;
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, AtomicBool, Ordering::SeqCst};
use std::sync::atomic::ATOMIC_USIZE_INIT;
use std::sync::{Arc, Mutex, MutexGuard};

use futures::Future;
use futures::sync::oneshot;
use js_sys::{Promise, Error, WebAssembly, Uint8ClampedArray, Array};
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, Worker, Event, ErrorEvent};
use web_sys::{DedicatedWorkerGlobalScope, MessageEvent};

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct Scene {
    inner: raytracer::scene::Scene,
}

static NEXT_ID: AtomicUsize = ATOMIC_USIZE_INIT;

#[wasm_bindgen]
impl Scene {
    #[wasm_bindgen(constructor)]
    pub fn new(object: &JsValue) -> Result<Scene, JsValue> {
        console_error_panic_hook::set_once();
        Ok(Scene {
            inner: object.into_serde()
                .map_err(|e| JsValue::from(e.to_string()))?,
        })
    }

    pub fn render(
        self,
        concurrency: usize,
        pool: WorkerPool,
        ctx: CanvasRenderingContext2d,
    ) -> Result<RenderingScene, JsValue> {
        let (tx, rx) = oneshot::channel();
        let rx = rx.then(|_| Ok(JsValue::undefined()));

        let data = Rc::new(RefCell::new(None::<Render>));

        let pixels = (self.inner.width * self.inner.height) as usize;
        let mut r = Render {
            tx: Some(tx),
            callback: None,
            shared: Arc::new(Shared {
                id: NEXT_ID.fetch_add(1, SeqCst),
                need_update: AtomicBool::new(false),
                scene: self.inner,
                next_pixel: AtomicUsize::new(0),
                remaining: AtomicUsize::new(concurrency),
                rgb_data: Mutex::new(vec![0; 4 * pixels]),
            }),
            ctx,
        };

        let data2 = data.clone();
        let callback = Closure::wrap(Box::new(move |msg: Event| -> Result<(), JsValue> {
            let mut slot = data2.borrow_mut();
            if let Some(mut data) = slot.take() {
                match data.event(&msg) {
                    Ok(true) => {}
                    Ok(false) => *slot = Some(data),
                    Err(e) => {
                        *slot = Some(data);
                        return Err(e)
                    }
                }
            }
            Ok(())
        }) as Box<FnMut(_) -> Result<(), JsValue>>);

        for worker in &pool.workers[..concurrency] {
            let ptr_to_send = Arc::into_raw(r.shared.clone()) as u32;
            let ptr_to_send = JsValue::from(ptr_to_send);
            worker.post_message(&ptr_to_send)?;
            worker.set_onmessage(Some(callback.as_ref().unchecked_ref()));
            worker.set_onerror(Some(callback.as_ref().unchecked_ref()));
        }

        r.callback = Some(callback);
        *data.borrow_mut() = Some(r);

        Ok(RenderingScene {
            inner: data,
            promise: wasm_bindgen_futures::future_to_promise(rx),
            pool,
        })
    }
}

#[wasm_bindgen]
pub struct WorkerPool {
    workers: Vec<Worker>,
    callback: Closure<FnMut(Event)>,
}

#[wasm_bindgen]
impl WorkerPool {
    #[wasm_bindgen(constructor)]
    pub fn new(max: u32) -> Result<WorkerPool, JsValue> {
        let callback = Closure::wrap(Box::new(|event: Event| {
            console_log!("unhandled event: {}", event.type_());
        }) as Box<FnMut(Event)>);
        let mut workers = Vec::new();
        for _ in 0..max {
            // TODO: what do do about `./worker.js`:
            //
            // * the path is only known by the bundler. How can we, as a
            //   library, know what's going on?
            // * How do we not fetch a script N times? It internally then
            //   causes another script to get fetched N times...
            let worker = Worker::new("./worker.js")?;
            let array = js_sys::Array::new();
            array.push(&wasm_bindgen::module());

            // TODO: memory allocation error handling here is hard:
            //
            // * How to we make sure that our strong ref made it to a client
            //   thread?
            // * Need to handle the `?` on `post_message` as well.
            array.push(&wasm_bindgen::memory());
            worker.post_message(&array)?;
            worker.set_onmessage(Some(callback.as_ref().unchecked_ref()));
            worker.set_onerror(Some(callback.as_ref().unchecked_ref()));
            workers.push(worker);
        }

        Ok(WorkerPool {
            workers,
            callback,
        })
    }
}

impl Drop for WorkerPool {
    fn drop(&mut self) {
        for worker in self.workers.iter() {
            worker.terminate();
        }
    }
}

#[wasm_bindgen]
pub struct RenderingScene {
    inner: Rc<RefCell<Option<Render>>>,
    promise: Promise,
    pool: WorkerPool,
}

#[wasm_bindgen]
impl RenderingScene {
    pub fn promise(&self) -> Promise {
        self.promise.clone()
    }

    #[wasm_bindgen(js_name = requestUpdate)]
    pub fn request_update(&self) {
        if let Some(render) = self.inner.borrow().as_ref() {
            render.shared.need_update.store(true, SeqCst);
        }
    }

    pub fn cancel(self) -> WorkerPool {
        if let Some(render) = self.inner.borrow_mut().take() {
            // drain the rest of the pixels to cause all workers to cancel ASAP.
            let pixels = render.shared.scene.width * render.shared.scene.height;
            render.shared.next_pixel.fetch_add(pixels as usize, SeqCst);
        }
        for worker in self.pool.workers.iter() {
            worker.set_onmessage(Some(&self.pool.callback.as_ref().unchecked_ref()));
            worker.set_onerror(Some(&self.pool.callback.as_ref().unchecked_ref()));
        }
        self.pool
    }
}

struct Render {
    callback: Option<Closure<FnMut(Event) -> Result<(), JsValue>>>,
    tx: Option<oneshot::Sender<()>>,
    shared: Arc<Shared>,
    ctx: CanvasRenderingContext2d,
}

struct Shared {
    id: usize,
    need_update: AtomicBool,
    scene: raytracer::scene::Scene,
    next_pixel: AtomicUsize,
    remaining: AtomicUsize,
    rgb_data: Mutex<Vec<u8>>,
}

#[wasm_bindgen]
extern {
    type ImageData;

    #[wasm_bindgen(constructor, catch)]
    fn new(data: &Uint8ClampedArray, width: f64, height: f64) -> Result<ImageData, JsValue>;
}

impl Render {
    fn event(&mut self, event: &Event) -> Result<bool, JsValue> {
        if let Some(error) = event.dyn_ref::<ErrorEvent>() {
            let msg = format!("error in worker: {}", error.message());
            return Err(Error::new(&msg).into());
        }

        if let Some(msg) = event.dyn_ref::<MessageEvent>() {
            let data = msg.data();
            if let Some(data) = data.dyn_ref::<Array>() {
                let id = data.pop();
                let done = data.pop();
                let image = data.pop();
                if let Some(id) = id.as_f64() {
                    if id == self.shared.id as f64 {
                        self.ctx.put_image_data(image.unchecked_ref(), 0.0, 0.0)?;
                        return Ok(done.as_bool() == Some(true))
                    }
                }
            }
            console_log!("unhandled message: {:?}", data);
            return Ok(false)
        }

        console_log!("unhandled event: {}", event.type_());

        Ok(false)
    }
}

#[wasm_bindgen]
pub fn child_entry_point(ptr: u32) -> Result<(), JsValue> {
    let ptr = unsafe {
        Arc::from_raw(ptr as *const Shared)
    };
    assert_send(&ptr);

    let global = js_sys::global()
        .unchecked_into::<DedicatedWorkerGlobalScope>();
    ptr.work(&global)?;

    return Ok(());

    fn assert_send<T: Send + 'static>(_: &T) {}
}

impl Shared {
    fn work(&self, global: &DedicatedWorkerGlobalScope) -> Result<(), JsValue> {
        // Once we're done raytracing a pixel we need to actually write its rgb
        // value into the shared memory buffer for our image. This, however,
        // requires synchronization with other threads (as currently
        // implemented). To help amortize the cost of synchronization each
        // thread processes a chunk of pixels at a time, and this number is how
        // many pixes will be rendered synchronously before committing them back
        // to memory.
        const BLOCK: usize = 1024;

        let width = self.scene.width as usize;
        let height = self.scene.height as usize;
        let end = width * height;

        // Thread-local storage for our RGB data, commited back in one batch to
        // the main image memory.
        let mut local_rgb = [0; BLOCK * 4];

        loop {
            // First up, grab a block of pixels to render using an atomic add.
            // If we're beyond the end then we're done!
            let start = self.next_pixel.fetch_add(BLOCK, SeqCst);
            if start >= end {
                break
            }

            // Raytrace all our pixels synchronously, writing all the results
            // into our local memory buffer.
            let len = cmp::min(end, start + BLOCK) - start;
            for (i, dst) in local_rgb.chunks_mut(4).enumerate().take(len) {
                let x = (start + i) % width;
                let y = (start + i) / width;
                let ray = raytracer::Ray::create_prime(x as u32, y as u32, &self.scene);
                let result = raytracer::cast_ray(&self.scene, &ray, 0).to_rgba();
                dst[0] = result.data[0];
                dst[1] = result.data[1];
                dst[2] = result.data[2];
                dst[3] = result.data[3];
            }

            // Ok, time to synchronize and commit this data back into the main
            // image buffer for other threads and the main thread to see.
            let mut data = self.rgb_data.lock().unwrap();
            data[start * 4..(start + len) * 4]
                .copy_from_slice(&mut local_rgb[..len * 4]);

            // As a "nifty feature" we try to have a live progressive rendering.
            // That means that we need to periodically send an `ImageData` to
            // the main thread. Do so whenever the main thread requests it.
            if self.need_update.swap(false, SeqCst) {
                self.update_image(false, data, global)?;
            }
        }


        // If we're the last thread out, be sure to update the main thread's
        // image as this is the last chance we'll get!
        if self.remaining.fetch_sub(1, SeqCst) == 1 {
            let data = self.rgb_data.lock().unwrap();
            self.update_image(true, data, global)?;
        }

        Ok(())
    }

    fn update_image(
        &self,
        done: bool,
        data: MutexGuard<Vec<u8>>,
        global: &DedicatedWorkerGlobalScope,
    ) -> Result<(), JsValue> {
        // This is pretty icky. We can't create an `ImageData` backed by
        // `SharedArrayBuffer`, so we need to copy the memory into a local
        // JS array using `slice`. This means we can't use
        // `web_sys::ImageData` right now but rather we have to use our own
        // binding.
        let mem = wasm_bindgen::memory()
            .unchecked_into::<WebAssembly::Memory>();
        let mem = Uint8ClampedArray::new(&mem.buffer())
            .slice(
                data.as_ptr() as u32,
                data.as_ptr() as u32 + data.len() as u32,
            );
        drop(data); // unlock the lock, we've copied the data now
        let data = ImageData::new(
            &mem,
            self.scene.width as f64,
            self.scene.height as f64,
        )?;
        let arr = Array::new();
        arr.push(&data);
        arr.push(&JsValue::from(done));
        arr.push(&JsValue::from(self.id as f64));
        global.post_message(&arr)?;
        Ok(())
    }
}
