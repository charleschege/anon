// Copyright 2018-2021 Parity Technologies (UK) Ltd.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![cfg_attr(not(feature = "std"), no_std)]

use ink_env::Environment;
use ink_lang as ink;

use frame_support::traits::PalletInfo;
use frame_support::weights::Pays;
use frame_support::{pallet, traits::Get};
use frame_system::RawOrigin;
use pallet_merkle::traits::dispatch::PostDispatchInfo;
use sp_core::H256;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
    Perbill,
};
/// This is an example of how ink! contract should
/// call substrate runtime `RandomnessCollectiveFlip::random_seed`.

/// Define the operations to interact with the substrate runtime

#[ink::chain_extension]
pub trait MerkleExtensionTrait {
    type ErrorCode = PalletMerkleExtError;

    // Create a group
    #[ink(extension = 1101, returns_result = false)]
    fn create_group() -> Result<CustomPostDispatchInfo, PalletMerkleExtError>;

    // Get Group
    #[ink(extension = 1102, returns_result = false)]
    fn add_elements() -> Result<CustomPostDispatchInfo, PalletMerkleExtError>;

    // Get Group
    #[ink(extension = 1103, returns_result = false)]
    fn verify_group() -> Result<CustomPostDispatchInfo, PalletMerkleExtError>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, scale::Encode, scale::Decode, scale_info::TypeInfo)]
pub enum PalletMerkleExtError {
    InvalidGroupCreation,
    CannotAddProvidedElementsToGroup,
    CouldNotVerifyGroup,
    InvalidScaleEncoding,
    EncounteredUnknownStatusCode,
}

impl From<scale::Error> for PalletMerkleExtError {
    fn from(_: scale::Error) -> Self {
        Self::InvalidScaleEncoding
    }
}

impl ink_env::chain_extension::FromStatusCode for PalletMerkleExtError {
    fn from_status_code(status_code: u32) -> Result<(), Self> {
        match status_code {
            0 => Ok(()),
            1101 => Err(Self::InvalidGroupCreation),
            1102 => Err(Self::CannotAddProvidedElementsToGroup),
            1103 => Err(Self::CouldNotVerifyGroup),
            _ => Err(Self::EncounteredUnknownStatusCode),
        }
    }
}

// Custom `frame_support::weights::Pays`
#[derive(Debug, Clone, Copy, PartialEq, Eq, scale::Encode, scale::Decode, scale_info::TypeInfo)]
pub enum CustomPays {
    Yes,
    No,
}

impl From<Pays> for CustomPays {
    fn from(value: Pays) -> Self {
        match value {
            Pays::Yes => CustomPays::Yes,
            Pays::No => CustomPays::No,
        }
    }
}

impl From<CustomPays> for Pays {
    fn from(value: CustomPays) -> Self {
        match value {
            CustomPays::Yes => Pays::Yes,
            CustomPays::No => Pays::No,
        }
    }
}

// Custom `PostDispatchInfo`
#[derive(Debug, Clone, Copy, PartialEq, Eq, scale::Encode, scale::Decode, scale_info::TypeInfo)]
pub struct CustomPostDispatchInfo {
    pub actual_weight: Option<u64>,
    pub pays_fee: CustomPays,
}

impl From<PostDispatchInfo> for CustomPostDispatchInfo {
    fn from(value: PostDispatchInfo) -> Self {
        let actual_weight = value.actual_weight;
        let pays_fee = match value.pays_fee {
            Pays::Yes => CustomPays::Yes,
            Pays::No => CustomPays::No,
        };

        Self {
            actual_weight,
            pays_fee,
        }
    }
}

pub enum CustomEnvironment {}

impl Environment for CustomEnvironment {
    type AccountId = <ink_env::DefaultEnvironment as Environment>::AccountId;
    type Balance = <ink_env::DefaultEnvironment as Environment>::Balance;
    type BlockNumber = <ink_env::DefaultEnvironment as Environment>::BlockNumber;
    type ChainExtension = MerkleExtensionTrait;
    type Hash = <ink_env::DefaultEnvironment as Environment>::Hash;
    type Timestamp = <ink_env::DefaultEnvironment as Environment>::Timestamp;

    const MAX_EVENT_TOPICS: usize = <ink_env::DefaultEnvironment as Environment>::MAX_EVENT_TOPICS;
}

#[ink::contract(env = crate::CustomEnvironment)]
mod pallet_merkle_operations {
    use super::{CustomPays, CustomPostDispatchInfo, PalletMerkleExtError};
    use pallet_merkle::{
        traits::Group,
        utils::keys::{Commitment, ScalarData},
        Pallet,
    };

    #[ink(storage)]
    pub struct MerkleExtension;

    impl MerkleExtension {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {}
        }

        #[ink(message)]
        pub fn create_group(&self) -> Result<CustomPostDispatchInfo, PalletMerkleExtError> {
            self.env().extension().create_group()?
        }

        #[ink(message)]
        pub fn add_elements(&self) -> Result<CustomPostDispatchInfo, PalletMerkleExtError> {
            self.env().extension().add_elements()?
        }

        #[ink(message)]
        pub fn verify_group(&self) -> Result<CustomPostDispatchInfo, PalletMerkleExtError> {
            self.env().extension().verify_group()?
        }
    }
}
