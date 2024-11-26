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
		ensure!(from != to, Error::<T>::TransferToSelf);
		let mut gato = Gatos::<T>::get(gato_id).ok_or(Error::<T>::NoGato)?;
		ensure!(gato.owner == from, Error::<T>::NotOwner);
		gato.owner = to.clone();

		let mut to_owned = GatosOwned::<T>::get(&to);
		to_owned.try_push(gato_id).map_err(|_| Error::<T>::TooManyOwned)?;
		let mut from_owned = GatosOwned::<T>::get(&from);
		if let Some(index) = from_owned.iter().position(|&id| id == gato_id) {
			from_owned.swap_remove(index);
		} else {
			return Err(Error::<T>::NoGato.into());
		}
		Gatos::<T>::insert(gato_id, gato);
		GatosOwned::<T>::insert(&to, to_owned);
		GatosOwned::<T>::insert(&from, from_owned);

		Self::deposit_event(Event::<T>::Transferred { from, to, gato_id });
		Ok(())
	}
}
