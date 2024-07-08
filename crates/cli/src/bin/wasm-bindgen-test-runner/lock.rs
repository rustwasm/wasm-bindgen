use anyhow::Result;
use std::fs::{hard_link, File, OpenOptions};
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::{id, Command};
use std::time::Duration;
use std::{env, thread};

pub struct Lock {
    file: PathBuf,
    lock: PathBuf,
}

impl Lock {
    pub fn try_new(name: &str) -> Result<Self> {
        let mut lock = Self {
            file: Self::create_file(name)?,
            lock: Self::create_lock(name)?,
        };
        lock.aquire()?;
        Ok(lock)
    }

    fn aquire(&mut self) -> Result<()> {
        while !self.try_aquire()? {
            thread::sleep(Duration::from_millis(100));
        }
        Ok(())
    }

    fn create_file(name: &str) -> Result<PathBuf> {
        let id = id();

        let file = env::temp_dir().join(format!("{}.{}", name, id));

        let mut file_handle = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&file)?;

        file_handle.write_all(id.to_string().as_bytes())?;
        file_handle.sync_all()?;

        Ok(file)
    }

    fn create_lock(name: &str) -> Result<PathBuf> {
        Ok(env::temp_dir().join(format!("{}.lock", name)))
    }

    fn has_lock(&self) -> bool {
        if let Ok(pid) = self.read_lock_pid() {
            if pid == id() {
                return true;
            }
        }
        false
    }

    fn read_lock_pid(&self) -> Result<u32> {
        let mut file = File::open(&self.lock)?;
        let mut pid = String::new();
        file.read_to_string(&mut pid)?;
        Ok(pid.parse()?)
    }

    fn remove_file(&self) {
        std::fs::remove_file(&self.file).ok();
    }

    fn remove_lock(&self) {
        std::fs::remove_file(&self.lock).ok();
    }

    fn try_aquire(&mut self) -> Result<bool> {
        if self.lock.exists() {
            if let Ok(pid) = self.read_lock_pid() {
                if pid == id() {
                    return Ok(true);
                }
                if is_process_running(pid) {
                    return Ok(false);
                }
            }

            self.remove_lock();
        }

        hard_link(&self.file, &self.lock).ok();

        Ok(self.has_lock())
    }
}

impl Drop for Lock {
    fn drop(&mut self) {
        if self.file.exists() {
            self.remove_file();
        }
        if self.lock.exists() && self.has_lock() {
            self.remove_lock();
        }
    }
}

pub fn is_process_running(pid: u32) -> bool {
    let output = Command::new("ps")
        .arg("-o")
        .arg("pid=")
        .arg("-p")
        .arg(pid.to_string())
        .output()
        .expect("Failed to execute ps command");

    !output.stdout.is_empty()
}
