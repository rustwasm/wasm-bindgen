use rand::Rng;
use std::fs;
use std::path::PathBuf;

pub struct Sandbox {
    assembly: PathBuf,
    original: PathBuf,
    root: PathBuf,
}

impl Sandbox {
    pub fn new(original: PathBuf) -> Self {
        let file_name = original.file_name().and_then(|s| s.to_str()).unwrap();

        let mut rng = rand::thread_rng();

        let root = original
            .parent() // chop off file name
            .and_then(|p| p.parent()) // chop off `deps`
            .and_then(|p| p.parent()) // chop off `debug`
            .and_then(|p| p.parent()) // chop off `wasm32-unknown-unknown`
            .map(|p| p.join("wasm-bindgen-test-runner-tests"))
            .map(|p| {
                p.join(format!(
                    "sandbox-{}-{}",
                    file_name,
                    rng.gen_range(1000..9999)
                ))
            })
            .unwrap();

        drop(fs::remove_dir_all(&root));

        let target = root.join("debug").join("deps");

        fs::create_dir_all(&target).unwrap();

        let assembly = target.join(file_name);

        fs::copy(&original, &assembly).unwrap();

        Self {
            assembly,
            original,
            root,
        }
    }

    pub fn assembly(&self) -> &PathBuf {
        &self.assembly
    }

    pub fn original(&self) -> &PathBuf {
        &self.original
    }

    pub fn root(&self) -> &PathBuf {
        &self.root
    }
}

impl Drop for Sandbox {
    fn drop(&mut self) {
        drop(fs::remove_dir_all(&self.root));
    }
}
