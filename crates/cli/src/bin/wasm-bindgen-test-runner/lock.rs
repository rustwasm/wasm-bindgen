use anyhow::Result;
use std::fs::{hard_link, metadata, read_dir, DirEntry, File, OpenOptions};
use std::io::Error;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::{id, Command};
use std::time::Duration;
use std::time::SystemTime;
use std::{env, thread};

pub struct Lock {
    file: PathBuf,
    lock: PathBuf,
    name: String,
    pid: u32,
    timestamp: SystemTime,
}

impl Lock {
    pub fn try_new(name: &str) -> Result<Self> {
        let pid = id();
        let file = Self::create_file(name, pid)?;
        let timestamp = metadata(&file).unwrap().modified().unwrap();
        let mut lock = Self {
            file,
            lock: Self::create_lock(name)?,
            name: name.to_string(),
            pid,
            timestamp,
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

    fn create_file(name: &str, pid: u32) -> Result<PathBuf> {
        let file = env::temp_dir().join(format!("{}.{}", name, pid));

        let mut file_handle = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&file)?;

        file_handle.write_all(pid.to_string().as_bytes())?;
        file_handle.sync_all()?;

        Ok(file)
    }

    fn create_lock(name: &str) -> Result<PathBuf> {
        Ok(env::temp_dir().join(format!("{}.lock", name)))
    }

    fn has_lock(&self) -> bool {
        if let Ok(pid) = self.read_lock_pid() {
            if pid == self.pid {
                return true;
            }
        }
        false
    }

    fn is_oldest_waiting(&self) -> bool {
        let lock_name = format!("{}.lock", &self.name);
        let file_prefix = format!("{}.", &self.name);

        for entry in read_dir(env::temp_dir()).unwrap() {
            if let Some((path, name)) = Self::is_candidate(entry, &lock_name, &file_prefix) {
                if self.is_candidate_older(&path) && Self::is_candidate_running(&name) {
                    return false;
                }
            }
        }

        true
    }

    fn is_candidate(
        entry: Result<DirEntry, Error>,
        lock_name: &str,
        file_prefix: &str,
    ) -> Option<(PathBuf, String)> {
        if let Ok(entry) = entry {
            let path = entry.path();
            if let Some(name) = path.clone().file_name().and_then(|n| n.to_str()) {
                if name.starts_with(file_prefix) && name != lock_name {
                    return Some((path, name.to_string()));
                }
            }
        }
        None
    }

    fn is_candidate_older(&self, candidate: &PathBuf) -> bool {
        metadata(candidate).unwrap().modified().unwrap() < self.timestamp
    }

    fn is_candidate_running(name: &str) -> bool {
        let pid = name.split('.').last().unwrap().parse().unwrap();
        is_process_running(pid)
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
                if pid == self.pid {
                    return Ok(true);
                }
                if is_process_running(pid) {
                    return Ok(false);
                }
            }

            self.remove_lock();
        }

        if !self.is_oldest_waiting() {
            return Ok(false);
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
