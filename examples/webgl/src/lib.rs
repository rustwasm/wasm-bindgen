extern crate js_sys;
extern crate wasm_bindgen;
extern crate web_sys;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader};

#[wasm_bindgen]
pub fn draw() {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("webgl")
        .unwrap()
        .unwrap()
        .dyn_into::<WebGlRenderingContext>()
        .unwrap();

    let vert_shader = compile_shader(
        &context,
        WebGlRenderingContext::VERTEX_SHADER,
        r#"
        attribute vec4 position;
        void main() {
            gl_Position = position;
        }
    "#,
    ).unwrap();
    let frag_shader = compile_shader(
        &context,
        WebGlRenderingContext::FRAGMENT_SHADER,
        r#"
        void main() {
            gl_FragColor = vec4(1.0, 1.0, 1.0, 1.0);
        }
    "#,
    ).unwrap();
    let program = link_program(&context, [vert_shader, frag_shader].iter()).unwrap();
    context.use_program(Some(&program));

    let vertices = [-0.7, -0.7, 0.0, 0.7, -0.7, 0.0, 0.0, 0.7, 0.0];
    let vert_array = js_sys::Float32Array::new(&wasm_bindgen::JsValue::from(vertices.len() as u32));
    for (i, f) in vertices.iter().enumerate() {
        vert_array.fill(*f, i as u32, (i + 1) as u32);
    }

    let buffer = context.create_buffer().unwrap();
    context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));
    context.buffer_data_with_opt_array_buffer(
        WebGlRenderingContext::ARRAY_BUFFER,
        Some(&vert_array.buffer()),
        WebGlRenderingContext::STATIC_DRAW,
    );
    context.vertex_attrib_pointer_with_i32(0, 3, WebGlRenderingContext::FLOAT, false, 0, 0);
    context.enable_vertex_attrib_array(0);

    context.clear_color(0.0, 0.0, 0.0, 1.0);
    context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

    context.draw_arrays(
        WebGlRenderingContext::TRIANGLES,
        0,
        (vertices.len() / 3) as i32,
    );
}

pub fn compile_shader(
    context: &WebGlRenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| "Unknown error creating shader".into()))
    }
}

pub fn link_program<'a, T: IntoIterator<Item = &'a WebGlShader>>(
    context: &WebGlRenderingContext,
    shaders: T,
) -> Result<WebGlProgram, String> {
    let program = context
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    for shader in shaders {
        context.attach_shader(&program, shader)
    }
    context.link_program(&program);

    if context
        .get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(context
            .get_program_info_log(&program)
            .unwrap_or_else(|| "Unknown error creating program object".into()))
    }
}
