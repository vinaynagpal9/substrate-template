#![cfg_attr(not(feature = "std"), no_std)]
use frame_support::{decl_module, decl_storage, dispatch, ensure};
use system::ensure_signed;
use sp_runtime::{
  DispatchResult,
  traits::{
    SimpleArithmetic, CheckedAdd, CheckedSub,
  },
};

// the module configuration trait
pub trait Trait: system::Trait { }

// storage for this runtime module
decl_storage! {
  trait Store for Module<T: Trait> as Template {
    TotalSupply get(fn total_supply) config(): u64 = 21000000;

    BalanceOf get(fn balance_of): map hasher(blake2_256) T::AccountId => u64;
  }
}

// public interface for this runtime module
decl_module! {
  pub struct Module<T: Trait> for enum Call where origin: T::Origin {

      // initialize the token
      // transfers the total_supply amout to the caller
      fn init(origin) -> DispatchResult {
        let sender = ensure_signed(origin)?;
        <BalanceOf<T>>::insert(sender, Self::total_supply());
        Ok(())
      }

      // transfer tokens from one account to another
      fn transfer(origin, to: T::AccountId, value: u64) -> DispatchResult {
        let sender = ensure_signed(origin)?;
        let sender_balance = Self::balance_of(sender.clone());
        ensure!(sender_balance >= value, "Not enough balance.");

        let updated_from_balance = sender_balance.checked_sub(value).ok_or("overflow in calculating balance")?;
        let receiver_balance = Self::balance_of(to.clone());
        let updated_to_balance = receiver_balance.checked_add(value).ok_or("overflow in calculating balance")?;
        
        // reduce sender's balance
        <BalanceOf<T>>::insert(sender, updated_from_balance);

        // increase receiver's balance
        <BalanceOf<T>>::insert(to.clone(), updated_to_balance);
        
        Ok(())
      }
  }
}