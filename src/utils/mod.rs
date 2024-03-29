use inquire::Confirm;

pub mod args;
pub mod docker;
pub mod sub_process;

mod error;
pub use error::{Error, ErrorKind};

pub fn prompt(message: &str) -> bool {
    match Confirm::new(&message).prompt() {
        Ok(true) => true,
        _ => false,
    }
}
