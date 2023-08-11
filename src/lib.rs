pub mod commands;
pub mod handlers;
mod nodes;
pub mod utils;

use bitcoin::Network;

use log::{debug, error, warn};

use crate::commands::CliOpts;
use crate::handlers::*;
use bdk::{bitcoin, Error};
use bdk_macros::{maybe_async, maybe_await};
use clap::Parser;

const REPL_LINE_SPLIT_REGEX: &str = r#""([^"]*)"|'([^']*)'|([\w\-]+)"#;
