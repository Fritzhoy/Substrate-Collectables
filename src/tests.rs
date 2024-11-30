// Tests for the Gatos Pallet.
//
// Normally this file would be split into two parts: `mock.rs` and `tests.rs`.
// The `mock.rs` file would contain all the setup code for our `TestRuntime`.
// Then `tests.rs` would only have the tests for our pallet.
// However, to minimize the project, these have been combined into this single file.
//
// Learn more about creating tests for Pallets:
// https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/guides/your_first_pallet/index.html

// This flag tells rust to only run this file when running `cargo test`.
#![cfg(test)]

use crate as pallet_gatos;
use crate::*;
use frame::deps::sp_io;
use frame::runtime::prelude::*;
use frame::testing_prelude::*;
use frame::traits::fungible::*;

type Balance = u64;
type Block = frame_system::mocking::MockBlock<TestRuntime>;

// In our "test runtime", we represent a user `AccountId` with a `u64`.
// This is just a simplification so that we don't need to generate a bunch of proper cryptographic
// public keys when writing tests. It is just easier to say "user 1 transfers to user 2".
// We create the constants `ALICE` and `BOB` to make it clear when we are representing users below.
const ALICE: u64 = 1;
const BOB: u64 = 2;
#[allow(unused)]
const DEFAULT_KITTY: Gato<TestRuntime> = Gato { dna: [0u8; 32], owner: 0, price: None };

// Our blockchain tests only need 3 Pallets:
// 1. System: Which is included with every FRAME runtime.
// 2. PalletBalances: Which is manages your blockchain's native currency. (i.e. DOT on Polkadot)
// 3. PalletGatos: The pallet you are building in this tutorial!
construct_runtime! {
	pub struct TestRuntime {
		System: frame_system,
		PalletBalances: pallet_balances,
		PalletGatos: pallet_gatos,
	}
}

// Normally `System` would have many more configurations, but you can see that we use some macro
// magic to automatically configure most of the pallet for a "default test configuration".
#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl frame_system::Config for TestRuntime {
	type Block = Block;
	type AccountData = pallet_balances::AccountData<Balance>;
}

// Normally `pallet_balances` would have many more configurations, but you can see that we use some
// macro magic to automatically configure most of the pallet for a "default test configuration".
#[derive_impl(pallet_balances::config_preludes::TestDefaultConfig)]
impl pallet_balances::Config for TestRuntime {
	type AccountStore = System;
	type Balance = Balance;
}

// This is the configuration of our Pallet! If you make changes to the pallet's `trait Config`, you
// will also need to update this configuration to represent that.
impl pallet_gatos::Config for TestRuntime {
	type RuntimeEvent = RuntimeEvent;
	type NativeBalance = PalletBalances;
}

// We need to run most of our tests using this function: `new_test_ext().execute_with(|| { ... });`
// It simulates the blockchain database backend for our tests.
// If you forget to include this and try to access your Pallet storage, you will get an error like:
// "`get_version_1` called outside of an Externalities-provided environment."
pub fn new_test_ext() -> sp_io::TestExternalities {
	frame_system::GenesisConfig::<TestRuntime>::default()
		.build_storage()
		.unwrap()
		.into()
}

#[test]
fn starting_template_is_sane() {
	new_test_ext().execute_with(|| {
		let event = Event::<TestRuntime>::Created { owner: ALICE };
		let _runtime_event: RuntimeEvent = event.into();
		let _call = Call::<TestRuntime>::create_gatos {};
		let result = PalletGatos::create_gatos(RuntimeOrigin::signed(BOB));
		assert_ok!(result);
	});
}

#[test]
fn system_and_balances_work() {
	// This test will just sanity check that we can access `System` and `PalletBalances`.
	new_test_ext().execute_with(|| {
		// We often need to set `System` to block 1 so that we can see events.
		System::set_block_number(1);
		// We often need to add some balance to a user to test features which needs tokens.
		assert_ok!(PalletBalances::mint_into(&ALICE, 100));
		assert_ok!(PalletBalances::mint_into(&BOB, 100));
	});
}
#[test]
fn create_gatos_checks_signed() {
	new_test_ext().execute_with(|| {
		// The `create_gatos` extrinsic should work when being called by a user.
		assert_ok!(PalletGatos::create_gatos(RuntimeOrigin::signed(ALICE)));
		// The `create_gatos` extrinsic should fail when being called by an unsigned message.
		assert_noop!(PalletGatos::create_gatos(RuntimeOrigin::none()), DispatchError::BadOrigin);
	})
}

#[test]
fn create_gatos_emits_event() {
	new_test_ext().execute_with(|| {
		// We need to set block number to 1 to view events.
		System::set_block_number(1);
		// Execute our call, and ensure it is successful.
		assert_ok!(PalletGatos::create_gatos(RuntimeOrigin::signed(ALICE)));
		// Assert the last event by our blockchain is the `Created` event with the correct owner.
		System::assert_last_event(Event::<TestRuntime>::Created { owner: 1 }.into());
	})
}
#[test]
fn count_for_gatos_created_correctly() {
	new_test_ext().execute_with(|| {
		// Querying storage before anything is set will return `0``.
		assert_eq!(CountForGatos::<TestRuntime>::get(), 0);
		// You can `set` the value using an `Option<u32>`.
		CountForGatos::<TestRuntime>::set(1337u32);
		// You can `put` the value directly with a `u32`.
		CountForGatos::<TestRuntime>::put(1337u32);
		// Check that the value is now in storage.
		assert_eq!(CountForGatos::<TestRuntime>::get(), 1337u32);
	})
}

#[test]
fn mint_increments_count_for_gatos() {
	new_test_ext().execute_with(|| {
		// Querying storage before anything is set will return `0`.
		assert_eq!(CountForGatos::<TestRuntime>::get(), 0);
		// Call `create_kitty` which will call `mint`.
		assert_ok!(PalletGatos::create_gatos(RuntimeOrigin::signed(ALICE)));
		// Now the storage should be `1`
		assert_eq!(CountForGatos::<TestRuntime>::get(), 1);
	})
}

#[test]
fn mint_errors_when_overflow() {
	new_test_ext().execute_with(|| {
		// Set the count to the largest value possible.
		CountForGatos::<TestRuntime>::set(u32::MAX);
		// `create_kitty` should not succeed because of safe math.
		assert_noop!(
			PalletGatos::create_gatos(RuntimeOrigin::signed(1)),
			Error::<TestRuntime>::TooManyGatos
		);
	})
}
#[test]
fn gatos_map_created_correctly() {
	new_test_ext().execute_with(|| {
		let zero_key = [0u8; 32];
		assert!(!Gatos::<TestRuntime>::contains_key(zero_key));
		Gatos::<TestRuntime>::insert(zero_key, DEFAULT_KITTY);
		assert!(Gatos::<TestRuntime>::contains_key(zero_key));
	})
}
#[test]
fn create_gatos_adds_to_map() {
	new_test_ext().execute_with(|| {
		assert_ok!(PalletGatos::create_gatos(RuntimeOrigin::signed(ALICE)));
		assert_eq!(Gatos::<TestRuntime>::iter().count(), 1);
	})
}
#[test]
fn cannot_mint_duplicate_gato() {
	new_test_ext().execute_with(|| {
		assert_ok!(PalletGatos::mint(ALICE, [0u8; 32]));
		assert_noop!(PalletGatos::mint(BOB, [0u8; 32]), Error::<TestRuntime>::DuplicateGato);
	})
}

#[test]
fn gato_struct_has_expected_traits() {
	new_test_ext().execute_with(|| {
		let gato = DEFAULT_KITTY;
		let bytes = gato.encode();
		let _decoded_gato = Gato::<TestRuntime>::decode(&mut &bytes[..]).unwrap();
		assert!(Gato::<TestRuntime>::max_encoded_len() > 0);
		let _info = Gato::<TestRuntime>::type_info();
	})
}

#[test]
fn mint_stores_owner_in_kitty() {
	new_test_ext().execute_with(|| {
		assert_ok!(PalletGatos::mint(1337, [42u8; 32]));
		let gato = Gatos::<TestRuntime>::get([42u8; 32]).unwrap();
		assert_eq!(gato.owner, 1337);
		assert_eq!(gato.dna, [42u8; 32]);
	})
}
#[test]
fn create_gato_makes_unique_Gatos() {
	new_test_ext().execute_with(|| {
		// Two calls to `create_kitty` should work.
		assert_ok!(PalletGatos::create_gatos(RuntimeOrigin::signed(ALICE)));
		assert_ok!(PalletGatos::create_gatos(RuntimeOrigin::signed(BOB)));
		// And should result in two Gatos in our system.
		assert_eq!(CountForGatos::<TestRuntime>::get(), 2);
		assert_eq!(Gatos::<TestRuntime>::iter().count(), 2);
	})
}
#[test]
fn gatos_owned_created_correctly() {
	new_test_ext().execute_with(|| {
		// Initially users have no Gatos owned.
		assert_eq!(GatosOwned::<TestRuntime>::get(1).len(), 0);
		// Let's create two Gatos.
		assert_ok!(PalletGatos::create_gatos(RuntimeOrigin::signed(ALICE)));
		assert_ok!(PalletGatos::create_gatos(RuntimeOrigin::signed(ALICE)));
		// Now they should have two Gatos owned.
		assert_eq!(GatosOwned::<TestRuntime>::get(1).len(), 2);
	});
}
#[test]
fn cannot_own_too_many_gatos() {
	new_test_ext().execute_with(|| {
		// If your max owned is different than 100, you will need to update this.
		for _ in 0..100 {
			assert_ok!(PalletGatos::create_gatos(RuntimeOrigin::signed(ALICE)));
		}
		assert_noop!(
			PalletGatos::create_gatos(RuntimeOrigin::signed(1)),
			Error::<TestRuntime>::TooManyOwned
		);
	});
}

#[test]
fn transfer_emits_event() {
	new_test_ext().execute_with(|| {
		// We need to set block number to 1 to view events.
		System::set_block_number(1);
		// Create a kitty to transfer
		assert_ok!(PalletGatos::create_gatos(RuntimeOrigin::signed(ALICE)));
		// Get the kitty id.
		let gato_id = Gatos::<TestRuntime>::iter_keys().collect::<Vec<_>>()[0];
		assert_ok!(PalletGatos::transfer(RuntimeOrigin::signed(ALICE), BOB, gato_id));
		System::assert_last_event(
			Event::<TestRuntime>::Transferred { from: ALICE, to: BOB, gato_id }.into(),
		);
	});
}

#[test]
fn transfer_logic_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(PalletGatos::create_gatos(RuntimeOrigin::signed(ALICE)));
		// Starting state looks good.
		let gato = &Gatos::<TestRuntime>::iter_values().collect::<Vec<_>>()[0];
		let gato_id = gato.dna;
		assert_eq!(gato.owner, ALICE);
		assert_eq!(GatosOwned::<TestRuntime>::get(ALICE), vec![gato_id]);
		assert_eq!(GatosOwned::<TestRuntime>::get(BOB), vec![]);
		// Cannot transfer to yourself.
		assert_noop!(
			PalletGatos::transfer(RuntimeOrigin::signed(ALICE), ALICE, gato_id),
			Error::<TestRuntime>::TransferToSelf
		);
		// Cannot transfer a non-existent kitty.
		assert_noop!(
			PalletGatos::transfer(RuntimeOrigin::signed(ALICE), BOB, [0u8; 32]),
			Error::<TestRuntime>::NoGato
		);
		// Cannot transfer kitty you do not own.
		assert_noop!(
			PalletGatos::transfer(RuntimeOrigin::signed(BOB), ALICE, gato_id),
			Error::<TestRuntime>::NotOwner
		);
		// Transfer should work when parameters are right.
		assert_ok!(PalletGatos::transfer(RuntimeOrigin::signed(ALICE), BOB, gato_id));
		// Storage is updated correctly.
		assert_eq!(GatosOwned::<TestRuntime>::get(ALICE), vec![]);
		assert_eq!(GatosOwned::<TestRuntime>::get(BOB), vec![gato_id]);
		let gato = &Gatos::<TestRuntime>::iter_values().collect::<Vec<_>>()[0];
		assert_eq!(gato.owner, BOB);
	});
}
#[test]
fn native_balance_associated_type_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(<<TestRuntime as Config>::NativeBalance as Mutate<_>>::mint_into(&ALICE, 1337));
		assert_eq!(
			<<TestRuntime as Config>::NativeBalance as Inspect<_>>::total_balance(&ALICE),
			1337
		);
	});
}
#[test]
fn balance_of_type_works() {
	// Inside our tests, the `BalanceOf` type has a concrete type of `u64`.
	let _example_balance: BalanceOf<TestRuntime> = 1337u64;
}
#[test]
fn set_price_emits_event() {
	new_test_ext().execute_with(|| {
		// We need to set block number to 1 to view events.
		System::set_block_number(1);
		assert_ok!(PalletGatos::create_gatos(RuntimeOrigin::signed(ALICE)));
		let gato_id = Gatos::<TestRuntime>::iter_keys().collect::<Vec<_>>()[0];
		assert_ok!(PalletGatos::set_price(RuntimeOrigin::signed(ALICE), gato_id, Some(1337)));
		// Assert the last event is `PriceSet` event with the correct information.
		System::assert_last_event(
			Event::<TestRuntime>::PriceSet { owner: ALICE, gato_id, new_price: Some(1337) }.into(),
		);
	})
}
#[test]
fn set_price_logic_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(PalletGatos::create_gatos(RuntimeOrigin::signed(ALICE)));
		let gato = &Gatos::<TestRuntime>::iter_values().collect::<Vec<_>>()[0];
		assert_eq!(gato.price, None);
		let gato_id = gato.dna;
		assert_ok!(PalletGatos::set_price(RuntimeOrigin::signed(ALICE), gato_id, Some(1337)));
		let gato = Gatos::<TestRuntime>::get(gato_id).unwrap();
		assert_eq!(gato.price, Some(1337));
	})
}
#[test]
fn do_buy_kitty_emits_event() {
	new_test_ext().execute_with(|| {
		// We need to set block number to 1 to view events.
		System::set_block_number(1);
		assert_ok!(PalletGatos::create_gatos(RuntimeOrigin::signed(ALICE)));
		let gato_id = Gatos::<TestRuntime>::iter_keys().collect::<Vec<_>>()[0];
		assert_ok!(PalletGatos::set_price(RuntimeOrigin::signed(ALICE), gato_id, Some(1337)));
		assert_ok!(PalletBalances::mint_into(&BOB, 100_000));
		assert_ok!(PalletGatos::buy_gato(RuntimeOrigin::signed(BOB), gato_id, 1337));
		// Assert the last event by our blockchain is the `Created` event with the correct owner.
		System::assert_last_event(
			Event::<TestRuntime>::Sold { buyer: BOB, gato_id, price: 1337 }.into(),
		);
	})
}

#[test]
fn do_buy_kitty_logic_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(PalletGatos::create_gatos(RuntimeOrigin::signed(ALICE)));
		let gato = &Gatos::<TestRuntime>::iter_values().collect::<Vec<_>>()[0];
		let gato_id = gato.dna;
		assert_eq!(gato.owner, ALICE);
		assert_eq!(GatosOwned::<TestRuntime>::get(ALICE), vec![gato_id]);
		// Cannot buy gato which does not exist.
		assert_noop!(
			PalletGatos::buy_gato(RuntimeOrigin::signed(BOB), [0u8; 32], 1337),
			Error::<TestRuntime>::NoGato
		);
		// Cannot buy kitty which is not for sale.
		assert_noop!(
			PalletGatos::buy_gato(RuntimeOrigin::signed(BOB), gato_id, 1337),
			Error::<TestRuntime>::GatoNotForSale
		);
		assert_ok!(PalletGatos::set_price(RuntimeOrigin::signed(ALICE), gato_id, Some(1337)));
		// Cannot buy kitty for a lower price.
		assert_noop!(
			PalletGatos::buy_gato(RuntimeOrigin::signed(BOB), gato_id, 1336),
			Error::<TestRuntime>::MaxPriceTooLow
		);
		// Cannot buy kitty if you don't have the funds.
		assert_noop!(
			PalletGatos::buy_gato(RuntimeOrigin::signed(BOB), gato_id, 1337),
			frame::arithmetic::ArithmeticError::Underflow
		);
		// Cannot buy kitty if it would kill your account (i.e. set your balance to 0).
		assert_ok!(PalletBalances::mint_into(&BOB, 1337));
		assert!(
			PalletGatos::buy_gato(RuntimeOrigin::signed(BOB), gato_id, 1337).is_err(),
			// TODO: assert_noop on DispatchError::Token(TokenError::NotExpendable)
		);
		// When everything is right, it works.
		assert_ok!(PalletBalances::mint_into(&BOB, 100_000));
		assert_ok!(PalletGatos::buy_gato(RuntimeOrigin::signed(BOB), gato_id, 1337));
		// State is updated correctly.
		assert_eq!(GatosOwned::<TestRuntime>::get(BOB), vec![gato_id]);
		let gato = Gatos::<TestRuntime>::get(gato_id).unwrap();
		assert_eq!(gato.owner, BOB);
		// Price is reset to `None`.
		assert_eq!(gato.price, None);
		// BOB transferred funds to ALICE.
		assert_eq!(PalletBalances::balance(&ALICE), 1337);
		assert_eq!(PalletBalances::balance(&BOB), 100_000);
	})
}
