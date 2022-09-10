use std::env;
use std::path::PathBuf;

pub fn get_current_dir() -> PathBuf {
  env::current_dir()
      .expect("Couldn't get the currenct directory")
}
