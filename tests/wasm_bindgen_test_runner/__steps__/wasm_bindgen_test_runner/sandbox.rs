use rand::Rng;
use std::fs;
use std::path::PathBuf;

pub struct Sandbox {
    assembly: Option<PathBuf>,
    original: PathBuf,
    root: Option<PathBuf>,
}

impl Sandbox {
    pub fn new(original: PathBuf) -> Self {
        if !&original.exists() {
            panic!("Couldn't find generated assembly: {}", &original.display());
        }
        Self {
            assembly: None,
            original,
            root: None,
        }
    }

    fn init(&mut self) {
        let file_name = self.original.file_name().and_then(|s| s.to_str()).unwrap();

        let mut rng = rand::thread_rng();

        let root = self
            .original
            .parent() // chop off file name
            .map(|p| p.join(format!("sandbox-{}", rng.gen_range(100000..999999))))
            .unwrap();

        drop(fs::remove_dir_all(&root));

        let target = root.join("debug").join("deps");

        fs::create_dir_all(&target).unwrap();

        let assembly = target.join(file_name);

        fs::copy(&self.original, &assembly).unwrap();

        self.assembly = Some(assembly);
        self.root = Some(root);
    }

    pub fn assembly(&mut self) -> &PathBuf {
        if self.assembly.is_none() {
            self.init();
        }
        self.assembly.as_ref().unwrap()
    }

    pub fn original(&self) -> &PathBuf {
        &self.original
    }

    pub fn root(&self) -> &PathBuf {
        self.root.as_ref().unwrap()
    }
}

impl Drop for Sandbox {
    fn drop(&mut self) {
        if let Some(root) = &self.root {
            drop(fs::remove_dir_all(root));
        }
    }
}
