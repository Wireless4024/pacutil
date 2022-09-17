use std::process::exit;

use clap::Parser;
use serde_json::Value;
use tracing::error;

#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
pub enum CommandLine {
	RemoveAll {
		/// target folder to link to ramdisk
		#[clap(value_parser)]
		filter: String,
	}
}