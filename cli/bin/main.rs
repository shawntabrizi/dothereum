// Copyright 2018-2019 Parity Technologies (UK) Ltd.
// Copyright 2019-2020 Dothereum UG (DE).
// This file is part of Dothereum.

// Dothereum is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Dothereum is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Dothereum.  If not, see <http://www.gnu.org/licenses/>.

//! Dothereum Node CLI

#![warn(missing_docs)]

use futures::sync::oneshot;
use futures::{future, Future};
use substrate_cli::VersionInfo;

use std::cell::RefCell;

// handles ctrl-c
struct Exit;
impl substrate_cli::IntoExit for Exit {
	type Exit = future::MapErr<oneshot::Receiver<()>, fn(oneshot::Canceled) -> ()>;
	fn into_exit(self) -> Self::Exit {
		// can't use signal directly here because CtrlC takes only `Fn`.
		let (exit_send, exit) = oneshot::channel();

		let exit_send_cell = RefCell::new(Some(exit_send));
		ctrlc::set_handler(move || {
			if let Some(exit_send) = exit_send_cell.try_borrow_mut().expect("signal handler not reentrant; qed").take() {
				exit_send.send(()).expect("Error sending exit notification");
			}
		}).expect("Error setting Ctrl-C handler");

		exit.map_err(drop)
	}
}

fn main() -> Result<(), substrate_cli::error::Error> {
	let version = VersionInfo {
		name: "Dothereum Node",
		commit: env!("VERGEN_SHA_SHORT"),
		version: env!("CARGO_PKG_VERSION"),
		executable_name: "dothereum",
		author: "Schoedon, Akinfiev",
		description: "Dothereum node implementation",
		support_url: "https://github.com/dothereum/dothereum/issues/new",
	};

	dothereum_cli::run(std::env::args(), Exit, version)
}
