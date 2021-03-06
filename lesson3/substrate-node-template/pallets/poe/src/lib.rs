#![cfg_attr(not(feature = "std"), no_std)]

/// A FRAME pallet template with necessary imports
use frame_support::{
    decl_error, decl_event, decl_module, decl_storage, dispatch, ensure, traits::Get,
};
use frame_system::{self as system, ensure_signed};
use sp_runtime::traits::StaticLookup;
use sp_std::prelude::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

/// The pallet's configuration trait.
pub trait Trait: system::Trait {
    // Add other types and constants required to configure this pallet.

    /// The overarching event type.
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;

    type ValidClaimLen: Get<u32>;
}

// This pallet's storage items.
decl_storage! {
    // It is important to update your storage name so that your pallet's
    // storage items are isolated from other pallets.
    // ---------------------------------vvvvvvvvvvvvvv
    trait Store for Module<T: Trait> as TemplateModule {
        Proofs get(fn proofs): map hasher(blake2_128_concat) Vec<u8> => (T::AccountId, T::BlockNumber);
    }
}

// The pallet's events
decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as system::Trait>::AccountId,
    {
        ClaimCreated(AccountId, Vec<u8>),
        ClaimRevoked(AccountId, Vec<u8>),
        ClaimTransfered(AccountId, Vec<u8>, AccountId),
    }
);

// The pallet's errors
decl_error! {
    pub enum Error for Module<T: Trait> {
        ProofAlreadyExist,
        ClaimNotExist,
        NotClaimOwner,
        NotValidClaimLen,
    }
}

// The pallet's dispatchable functions.
decl_module! {
    /// The module declaration.
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        // Initializing errors
        // this includes information about your errors in the node's metadata.
        // it is needed only if you are using errors in your pallet
        type Error = Error<T>;

        // Initializing events
        // this is needed only if you are using events in your pallet
        fn deposit_event() = default;

        #[weight = 0]
        pub fn create_claim(origin, claim: Vec<u8>) -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;

            ensure!(!Proofs::<T>::contains_key(&claim), Error::<T>::ProofAlreadyExist);

            //创建存证时，为存证内容的哈希值设置界限
            let claim_len =  claim.len() as u32;
            //ensure!(claim_len >= T::ValidClaimLen::get(),  Error::<T>::NotValidClaimLen);

            Proofs::<T>::insert(&claim, (sender.clone(), system::Module::<T>::block_number()));

            Self::deposit_event(RawEvent::ClaimCreated(sender, claim));

            Ok(())
        }

        #[weight = 0]
        pub fn revoke_claim(origin, claim: Vec<u8>) -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;

            ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::ClaimNotExist);

            let (owner, _block_number) = Proofs::<T>::get(&claim);

            ensure!(owner == sender, Error::<T>::NotClaimOwner);

            Proofs::<T>::remove(&claim);

            Self::deposit_event(RawEvent::ClaimRevoked(sender, claim));

            Ok(())
        }

        #[weight = 0]
        pub fn transfer_claim(origin, claim: Vec<u8>, to: <T::Lookup as StaticLookup>::Source) -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;

            ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::ClaimNotExist);

            let (owner, _) = Proofs::<T>::get(&claim);
            ensure!(owner == sender, Error::<T>::NotClaimOwner);

            let to = T::Lookup::lookup(to)?;
            Proofs::<T>::insert(&claim, (to.clone(), system::Module::<T>::block_number()));

            Self::deposit_event(RawEvent::ClaimTransfered(sender, claim, to));

            Ok(())
        }
    }
}
