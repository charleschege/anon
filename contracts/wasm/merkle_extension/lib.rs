#![cfg_attr(not(feature = "std"), no_std)]

use frame_system::Origin;
use ink_env::Environment;
use ink_lang as ink;
use pallet_merkle::{
	traits::dispatch::PostDispatchInfo,
	utils::keys::{Commitment, ScalarData},
	Pallet,
};

use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
	DispatchErrorWithPostInfo, Perbill,
};

///Custom pallet-merkle extension to read and write from the runtime
#[ink::chain_extension]
pub trait RuntimeMerkleExtension {
	type ErrorCode = MerkleExtensionError;

	///Create a Group
	#[ink(extension = 1)]
	fn create_group(r_is_mgt: bool, depth: Option<u8>) -> Result<PostDispatchInfo, MerkleExtensionError>;

	///Add Members to a Group
	#[ink(extension = 2)]
	fn get_group(group_id: u32, members: Vec<ScalarData>) -> Result<PostDispatchInfo, MerkleExtensionError>;

	///Verify Membership proof
	#[ink(extension = 3)]
	fn verify_membership(
		cached_root: ScalarData,
		comms: Vec<Commitment>,
		nullifier_hash: ScalarData,
		proof_bytes: Vec<u8>,
		leaf_index_commitments: Vec<Commitment>,
		proof_commitments: Vec<Commitment>,
		recipient: ScalarData,
		relayer: ScalarData,
	) -> Result<(), MerkleExtensionError>;
}

#[derive(scale::Encode, scale::Decode, scale_info::TypeInfo)]
pub enum MerkleExtensionError {
	InvalidCall,
	GroupCreationError,
	AddElementsError,
	MemberVerificationError,
	DispatchError, //TODO (DispatchErrorWithPostInfo<PostDispatchInfo>),
	ParityCodec,   //TODO (scale::Error),
	EncounteredUnknownStatusCode,
}

impl From<DispatchErrorWithPostInfo<PostDispatchInfo>> for MerkleExtensionError {
	fn from(error_code: DispatchErrorWithPostInfo<PostDispatchInfo>) -> MerkleExtensionError {
		Self::DispatchError //(error_code)
	}
}

impl From<scale::Error> for MerkleExtensionError {
	fn from(error_code: scale::Error) -> MerkleExtensionError {
		Self::ParityCodec //TODO (error_code)
	}
}

impl ink_env::chain_extension::FromStatusCode for MerkleExtensionError {
	fn from_status_code(status_code: u32) -> Result<(), Self> {
		match status_code {
			0 => Ok(()),
			1 => Err(Self::GroupCreationError),
			2 => Err(Self::AddElementsError),
			3 => Err(Self::MemberVerificationError),
			_ => Err(Self::EncounteredUnknownStatusCode),
		}
	}
}

pub enum PalletMerkleEnvironment {}

impl Environment for PalletMerkleEnvironment {
	type AccountId = <ink_env::DefaultEnvironment as Environment>::AccountId;
	type Balance = <ink_env::DefaultEnvironment as Environment>::Balance;
	type BlockNumber = <ink_env::DefaultEnvironment as Environment>::BlockNumber;
	type ChainExtension = RuntimeMerkleExtension;
	type Hash = <ink_env::DefaultEnvironment as Environment>::Hash;
	type Timestamp = <ink_env::DefaultEnvironment as Environment>::Timestamp;

	const MAX_EVENT_TOPICS: usize = <ink_env::DefaultEnvironment as Environment>::MAX_EVENT_TOPICS;
}

#[ink::contract(env = crate::PalletMerkleEnvironment)]
mod merkle_extension {
	use super::*;
	use frame_support::weights::Pays;

	#[ink(storage)]
	#[derive(Clone, PartialEq, Eq)]
	pub struct MerkleExtension {
		weight: Option<u64>,
		pays_fee: bool,
	}

	impl<T: frame_system::Config> frame_system::Config for MerkleExtension {
		type AccountData = u64;
		type AccountId = u64;
		type BaseCallFilter = ();
		type BlockHashCount = u64;
		type BlockLength = ();
		type BlockNumber = u64;
		type BlockWeights = ();
		type Call = pallet_merkle::pallet::Call<T>;
		type DbWeight = ();
		type Event = Self::Event;
		type Hash = sp_runtime::testing::H256;
		type Hashing = BlakeTwo256;
		type Header = Header;
		type Index = u64;
		type Lookup = IdentityLookup<Self::AccountId>;
		type OnKilledAccount = ();
		type OnNewAccount = ();
		type Origin = u64;
		type PalletInfo = Self::PalletInfo;
		type SS58Prefix = u8;
		type SystemWeightInfo = ();
		type Version = ();
	}

	impl MerkleExtension {
		#[ink(constructor)]
		pub fn new() -> Self {
			Self {
				weight: Option::default(),
				pays_fee: bool::default(),
			}
		}

		#[ink(message)]
		pub fn create_group(
			&mut self,
			r_is_mgr: bool,
			_depth: Option<u8>,
		) -> Result<PostDispatchInfo, MerkleExtensionError> {
			let origin = self.env().account_id();

			let new_group = Pallet::<Self>::create_group(origin, r_is_mgr, _depth)?;

			self.weight = new_group.actual_weight;

			match new_group.pays_fee {
				Pays::Yes => self.pays_fee = true,
				Pays::No => self.pays_fee = false,
			}

			Ok(new_group)
		}

		#[ink(message)]
		pub fn get_group(&self) {
			//self.value = !self.value;
		}
	}
}

/*
use ink_env::AccountId;
use ink_lang as ink;

#[ink::contract]
mod merkle_extension {
	use pallet_merkle::{
		traits::dispatch::{DispatchErrorWithPostInfo, PostDispatchInfo},
		utils::keys::{Commitment, ScalarData},
		Config, Pallet,
	};

	#[derive(Debug, Copy, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
	#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
	pub enum MerkleExtensionError {
		FailedToPerformOperation,
	}

	impl ink_env::chain_extension::FromStatusCode for MerkleExtensionError {
		fn from_status_code(status_code: u32) -> Result<(), Self> {
			match status_code {
				0 => Ok(()),
				1 => Err(Self::FailedToPerformOperation),
				_ => panic!("encountered unknown status code"),
			}
		}
	}

	pub type MerkleResult<T> = Result<T, MerkleExtensionError>;

	/// Defines the storage of your contract.
	/// Add new fields to the below struct in order
	/// to add new static storage fields to your contract.
	#[ink(storage)]
	#[derive(Clone, PartialEq, Eq)]
	pub struct MerkleExtension {
		account_id: AccountId,
	}

	impl Config for MerkleExtension {}

	impl frame_system::pallet::Config for MerkleExtension {}

	impl MerkleExtension {
		#[ink(constructor)]
		pub fn new() -> Self {
			let account_id = Self::env().account_id();

			Self { account_id }
		}

		#[ink(message)]
		pub fn create_group(&self, r_is_mgr: bool, _depth: Option<u8>) -> MerkleResult<()> {
			let origin = Self::env().account_id();
			match Pallet::<MerkleExtension>::create_group(origin, r_is_mgr, _depth) {
				Ok(_) => Ok(()),
				Err(_) => Err(MerkleExtensionError::FailedToPerformOperation),
			}
		}

		#[ink(message)]
		pub fn get_group(&self) {
			//self.value = !self.value;
		}
	}
	/*
	/// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
	/// module and test functions are marked with a `#[test]` attribute.
	/// The below code is technically just normal Rust code.
	#[cfg(test)]
	mod tests {
		/// Imports all the definitions from the outer scope so we can use them
		/// here.
		use super::*;

		/// We test if the default constructor does its job.
		#[test]
		fn default_works() {
			let merkle_extension = MerkleExtension::default();
			assert_eq!(merkle_extension.get(), false);
		}

		/// We test a simple use case of our contract.
		#[test]
		fn it_works() {
			let mut merkle_extension = MerkleExtension::new(false);
			assert_eq!(merkle_extension.get(), false);
			merkle_extension.flip();
			assert_eq!(merkle_extension.get(), true);
		}
	}*/
}
*/
