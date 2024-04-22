use fs2::FileExt;
use std::fs::File;
//use std::remove_file;
use std::ffi::OsStr;
use std::io::Error;
use std::path::PathBuf;

pub struct TmpDirLock {
    exclusive: File,
    shared: File,
    exclusive_path: PathBuf,
    shared_path: PathBuf,
}

impl TmpDirLock {
    pub fn new(directory: &PathBuf) -> Result<Self, Error> {
        let base = directory.parent().unwrap();
        let name = directory.file_name().and_then(OsStr::to_str).unwrap();
        let exclusive = &base.join(format!("{}-exclusive.lock", name));
        let shared = &base.join(format!("{}-shared.lock", name));
        Ok(TmpDirLock {
            exclusive: File::create(exclusive)?,
            exclusive_path: exclusive.to_path_buf(),
            shared: File::create(shared)?,
            shared_path: shared.to_path_buf(),
        })
    }

    pub fn downgrade_lock(&self) -> Result<(), Error> {
        self.shared.unlock()?;
        self.shared.lock_shared()?;
        self.exclusive.unlock()?;
        Ok(())
    }

    pub fn lock_shared(&self) -> Result<(), Error> {
        self.shared.lock_shared()
    }

    pub fn try_lock_exclusive(&self) -> Result<(), Error> {
        self.exclusive.try_lock_exclusive()?;
        let result = self.shared.try_lock_exclusive();
        if result.is_err() {
            self.exclusive.unlock()?;
            result
        } else {
            Ok(())
        }
    }

    pub fn try_upgrade_lock(&self) -> Result<(), Error> {
        self.exclusive.try_lock_exclusive()?;
        let result = self.shared.unlock();
        if result.is_err() {
            self.exclusive.unlock()?;
            return result;
        }
        let result = self.shared.try_lock_exclusive();
        if result.is_err() {
            self.exclusive.unlock()?;
            result
        } else {
            Ok(())
        }
    }
}

impl Drop for TmpDirLock {
    fn drop(&mut self) {
        let _ = self.exclusive.unlock();
        let _ = self.shared.unlock();
        //let _ = remove_file(&self.exclusive_path);
        //let _ = remove_file(&self.shared_path);
    }
}
