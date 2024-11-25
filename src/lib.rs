#![cfg_attr(not(feature = "std"), no_std)]

mod impls;
mod tests;

use frame::prelude::*;
pub use pallet::*;

#[frame::pallet(dev_mode)]
pub mod pallet {
	use super::*;

	#[pallet::pallet]
	pub struct Pallet<T>(core::marker::PhantomData<T>);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	#[derive(Encode, Decode, MaxEncodedLen, TypeInfo)]
	#[scale_info(skip_type_params(T))]

	pub struct Gato<T: Config> {
		pub dna: [u8; 32],
		pub owner: T::AccountId,
	}

	#[pallet::storage]
	//A StorageValue stores a single value into a single key in the Merkle Trie.
	pub type CountForGatos<T: Config> = StorageValue<Value = u32, QueryKind = ValueQuery>;

	#[pallet::storage]
	// A StorageMap stores multiple values under different storage keys, all into different places
	// in the Merkle Trie.
	pub(super) type Gatos<T: Config> = StorageMap<Key = [u8; 32], Value = Gato<T>>;

	#[pallet::storage]
	pub(super) type GatosOwned<T: Config> = StorageMap<
		Key = T::AccountId,
		Value = BoundedVec<[u8; 32], ConstU32<100>>,
		QueryKind = ValueQuery,
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		Created { owner: T::AccountId },
	}

	#[pallet::error]
	pub enum Error<T> {
		TooManyGatos,
		DuplicateGato,
		TooManyOwned,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		pub fn create_gatos(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let dna = Self::gen_dna();
			Self::mint(who, dna)?;
			Ok(())
		}
	}
}
