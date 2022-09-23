use std::env;
use std::path::PathBuf;

pub fn main() -> PathBuf {
  env::current_dir()
      .expect("Couldn't get the currenct directory")
}
