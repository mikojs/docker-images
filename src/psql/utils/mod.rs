use std::env;
use std::process;

use inquire::Confirm;

pub use crate::utils::proxy_args;

pub use database::Database;

pub mod docker;
pub mod database;
