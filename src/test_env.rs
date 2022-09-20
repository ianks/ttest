use lazy_static::lazy_static;
use std::{env::temp_dir, panic::UnwindSafe, path::PathBuf, sync::Mutex};
use uuid::Uuid;

use crate::test_selector::TestSelector;

#[derive(Debug)]
pub struct TestEnv {
    pub temp_dir: PathBuf,
    pub old_cwd: PathBuf,
}

lazy_static! {
    static ref TEST_ENV: Mutex<()> = Mutex::new(());
}

impl TestEnv {
    fn new() -> Self {
        let old_cwd = std::env::current_dir().unwrap();
        let temp_dir = temp_dir()
            .join("ttest_env")
            .join(Uuid::new_v4().to_string());

        // change into temp dir
        std::fs::create_dir_all(&temp_dir).unwrap();
        std::env::set_current_dir(&temp_dir).unwrap();

        Self { temp_dir, old_cwd }
    }
    pub fn write_file(&self, path: &str, contents: &str) {
        let path = self.temp_dir.join(path);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, contents).unwrap();
    }

    pub fn selector(&self, path: &str) -> TestSelector {
        path.parse().unwrap()
    }
}

impl Drop for TestEnv {
    fn drop(&mut self) {
        std::env::set_current_dir(&self.old_cwd).unwrap();
        std::fs::remove_dir_all(&self.temp_dir).unwrap();
    }
}

pub fn with<F, T>(func: F) -> T
where
    F: FnOnce(&TestEnv) -> T + UnwindSafe,
{
    let guard = TEST_ENV.lock().unwrap();
    let test_env = TestEnv::new();

    match std::panic::catch_unwind(|| func(&test_env)) {
        Err(e) => {
            drop(test_env);
            drop(guard);
            std::panic::resume_unwind(e);
        }
        Ok(result) => {
            drop(test_env);
            drop(guard);
            result
        }
    }
}
