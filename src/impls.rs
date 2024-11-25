use super::*;
use frame::prelude::*;
use frame::primitives::BlakeTwo256;
use frame::traits::Hash;

impl<T: Config> Pallet<T> {
	pub fn gen_dna() -> [u8; 32] {
		let unique_payload = (
			frame_system::Pallet::<T>::parent_hash(),
			frame_system::Pallet::<T>::block_number(),
			frame_system::Pallet::<T>::extrinsic_index(),
			CountForGatos::<T>::get(),
		);
		BlakeTwo256::hash_of(&unique_payload).into()
	}

	pub fn mint(owner: T::AccountId, dna: [u8; 32]) -> DispatchResult {
		let gato = Gato { dna, owner: owner.clone() };
		//Check if the dna of a gato already exist
		ensure!(!Gatos::<T>::contains_key(dna), Error::<T>::DuplicateGato);

		//Incluir o generic trait <T> em todos os lugares
		//Na definição do CountForGatos, foi usado a generic trait <T: Config>
		//Portanto precisa incluir o <T> para acessar qualquer API.
		let current_count: u32 = CountForGatos::<T>::get();

		// The checked math APIs will check if there are any underflows or overflows, and return
		// None in /those cases. Otherwise, if the math operation is calculated without error, it
		// returns Some(result).
		let new_count: u32 = current_count.checked_add(1).ok_or(Error::<T>::TooManyGatos)?;

		GatosOwned::<T>::try_append(&owner, dna).map_err(|_| Error::<T>::TooManyOwned)?;
		Gatos::<T>::insert(dna, gato);
		CountForGatos::<T>::set(new_count);

		Self::deposit_event(Event::<T>::Created { owner });
		Ok(())
	}
	pub fn do_transfer(from: T::AccountId, to: T::AccountId, gato_id: [u8; 32]) -> DispatchResult {
		Self::deposit_event(Event::<T>::Transferred { from, to, gato_id });
		Ok(())
	}
}
