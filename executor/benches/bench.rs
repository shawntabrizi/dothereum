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

use codec::{Decode, Encode};
use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use dothereum_executor::Executor;
use dothereum_primitives::{BlockNumber, Hash};
use dothereum_runtime::{
	Block, BuildStorage, Call, CheckedExtrinsic, GenesisConfig, Header, UncheckedExtrinsic,
};
use dothereum_runtime::constants::currency::*;
use dothereum_testing::keyring::*;
use primitives::{Blake2Hasher, NativeOrEncoded, NeverNativeValue};
use primitives::storage::well_known_keys;
use primitives::traits::CodeExecutor;
use runtime_support::Hashable;
use state_machine::TestExternalities as CoreTestExternalities;
use substrate_executor::{NativeExecutor, RuntimeInfo, WasmExecutionMethod, Externalities};

criterion_group!(benches, bench_execute_block);
criterion_main!(benches);

/// The wasm runtime code.
const COMPACT_CODE: &[u8] = dothereum_runtime::WASM_BINARY;

const GENESIS_HASH: [u8; 32] = [69u8; 32];

const VERSION: u32 = dothereum_runtime::VERSION.spec_version;

const HEAP_PAGES: u64 = 20;

type TestExternalities<H> = CoreTestExternalities<H, u64>;

#[derive(Debug)]
enum ExecutionMethod {
	Native,
	Wasm(WasmExecutionMethod),
}

fn sign(xt: CheckedExtrinsic) -> UncheckedExtrinsic {
	dothereum_testing::keyring::sign(xt, VERSION, GENESIS_HASH)
}

fn new_test_ext(genesis_config: &GenesisConfig) -> TestExternalities<Blake2Hasher> {
	let mut test_ext = TestExternalities::new_with_code(
		COMPACT_CODE,
		genesis_config.build_storage().unwrap(),
	);
	test_ext.ext().place_storage(well_known_keys::HEAP_PAGES.to_vec(), Some(HEAP_PAGES.encode()));
	test_ext
}

fn construct_block<E: Externalities>(
	executor: &NativeExecutor<Executor>,
	ext: &mut E,
	number: BlockNumber,
	parent_hash: Hash,
	extrinsics: Vec<CheckedExtrinsic>,
) -> (Vec<u8>, Hash) {
	use trie::{TrieConfiguration, trie_types::Layout};

	// sign extrinsics.
	let extrinsics = extrinsics.into_iter().map(sign).collect::<Vec<_>>();

	// calculate the header fields that we can.
	let extrinsics_root = Layout::<Blake2Hasher>::ordered_trie_root(
		extrinsics.iter().map(Encode::encode)
	).to_fixed_bytes()
		.into();

	let header = Header {
		parent_hash,
		number,
		extrinsics_root,
		state_root: Default::default(),
		digest: Default::default(),
	};

	// execute the block to get the real header.
	executor.call::<_, NeverNativeValue, fn() -> _>(
		ext,
		"Core_initialize_block",
		&header.encode(),
		true,
		None,
	).0.unwrap();

	for i in extrinsics.iter() {
		executor.call::<_, NeverNativeValue, fn() -> _>(
			ext,
			"BlockBuilder_apply_extrinsic",
			&i.encode(),
			true,
			None,
		).0.unwrap();
	}

	let header = match executor.call::<_, NeverNativeValue, fn() -> _>(
		ext,
		"BlockBuilder_finalize_block",
		&[0u8;0],
		true,
		None,
	).0.unwrap() {
		NativeOrEncoded::Native(_) => unreachable!(),
		NativeOrEncoded::Encoded(h) => Header::decode(&mut &h[..]).unwrap(),
	};

	let hash = header.blake2_256();
	(Block { header, extrinsics }.encode(), hash.into())
}


fn test_blocks(genesis_config: &GenesisConfig, executor: &NativeExecutor<Executor>)
	-> Vec<(Vec<u8>, Hash)>
{
	let mut test_ext = new_test_ext(genesis_config);
	let mut block1_extrinsics = vec![
		CheckedExtrinsic {
			signed: None,
			function: Call::Timestamp(timestamp::Call::set(42 * 1000)),
		},
	];
	block1_extrinsics.extend((0..20).map(|i| {
		CheckedExtrinsic {
			signed: Some((alice(), signed_extra(i, 0))),
			function: Call::Balances(balances::Call::transfer(bob().into(), 1 * DOLLARS)),
		}
	}));
	let block1 = construct_block(
		executor,
		&mut test_ext.ext(),
		1,
		GENESIS_HASH.into(),
		block1_extrinsics,
	);

	vec![block1]
}

fn bench_execute_block(c: &mut Criterion) {
	c.bench_function_over_inputs(
		"execute blocks",
		|b, strategy| {
			let genesis_config = dothereum_testing::genesis::config(false, Some(COMPACT_CODE));
			let (use_native, wasm_method) = match strategy {
				ExecutionMethod::Native => (true, WasmExecutionMethod::Interpreted),
				ExecutionMethod::Wasm(wasm_method) => (false, *wasm_method),
			};
			let executor = NativeExecutor::new(wasm_method, None);

			// Get the runtime version to initialize the runtimes cache.
			{
				let mut test_ext = new_test_ext(&genesis_config);
				executor.runtime_version(&mut test_ext.ext());
			}

			let blocks = test_blocks(&genesis_config, &executor);

			b.iter_batched_ref(
				|| new_test_ext(&genesis_config),
				|test_ext| {
					for block in blocks.iter() {
						executor.call::<_, NeverNativeValue, fn() -> _>(
							&mut test_ext.ext(),
							"Core_execute_block",
							&block.0,
							use_native,
							None,
						).0.unwrap();
					}
				},
				BatchSize::LargeInput,
			);
		},
		vec![
			ExecutionMethod::Native,
			ExecutionMethod::Wasm(WasmExecutionMethod::Interpreted),
			#[cfg(feature = "wasmtime")]
			ExecutionMethod::Wasm(WasmExecutionMethod::Compiled),
		],
	);
}
