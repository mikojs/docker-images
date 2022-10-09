use inquire::Confirm;

pub mod proxy_args;
pub mod sub_process;
pub mod docker;

mod error;
pub use error::{Error, ErrorKind};

pub fn prompt(message: &str) -> bool {
    match Confirm::new(&message).prompt() {
        Ok(true) => true,
        _ => false,
    }
}
