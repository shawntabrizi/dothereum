//! Dothereum CLI library.

#![warn(missing_docs)]
#![warn(unused_extern_crates)]

mod chain_spec;
#[macro_use]
mod service;
mod cli;

#[macro_use]
extern crate rust_embed;

pub use substrate_cli::{VersionInfo, IntoExit, error};

fn main() {
	let version = VersionInfo {
		name: "Dothereum Node",
		commit: env!("VERGEN_SHA_SHORT"),
		version: env!("CARGO_PKG_VERSION"),
		executable_name: "dothereum",
		author: "Schoedon",
		description: "Dothereum",
		support_url: "https://github.com/dothereum/dothereum/issues",
	};

	if let Err(e) = cli::run(::std::env::args(), cli::Exit, version) {
		eprintln!("Fatal error: {}\n\n{:?}", e, e);
		std::process::exit(1)
	}
}
