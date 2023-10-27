#![cfg_attr(not(feature = "std"), no_std)]

pub mod balance_change;
pub mod macros;
pub mod note;
pub mod notebook;

pub use balance_change::*;
pub use macros::*;
pub use note::*;
pub use notebook::*;
use sp_core::crypto::AccountId32;

pub type AccountId = AccountId32;