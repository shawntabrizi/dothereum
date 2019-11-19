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

//! Utilites to build a `TestClient` for `dothereum-runtime`.

use sr_primitives::BuildStorage;

/// Re-export test-client utilities.
pub use test_client::*;

/// Call executor for `dothereum-runtime` `TestClient`.
pub type Executor = substrate_executor::NativeExecutor<dothereum_executor::Executor>;

/// Default backend type.
pub type Backend = client_db::Backend<dothereum_primitives::Block>;

/// Test client type.
pub type Client = client::Client<
	Backend,
	client::LocalCallExecutor<Backend, Executor>,
	dothereum_primitives::Block,
	dothereum_runtime::RuntimeApi,
>;

/// Genesis configuration parameters for `TestClient`.
#[derive(Default)]
pub struct GenesisParameters {
	support_changes_trie: bool,
}

impl test_client::GenesisInit for GenesisParameters {
	fn genesis_storage(&self) -> (StorageOverlay, ChildrenStorageOverlay) {
		crate::genesis::config(self.support_changes_trie, None).build_storage().unwrap()
	}
}

/// A `test-runtime` extensions to `TestClientBuilder`.
pub trait TestClientBuilderExt: Sized {
	/// Create test client builder.
	fn new() -> Self;

	/// Build the test client.
	fn build(self) -> Client;
}

impl TestClientBuilderExt for test_client::TestClientBuilder<
	client::LocalCallExecutor<Backend, Executor>,
	Backend,
	GenesisParameters,
> {
	fn new() -> Self{
		Self::default()
	}

	fn build(self) -> Client {
		self.build_with_native_executor(None).0
	}
}


