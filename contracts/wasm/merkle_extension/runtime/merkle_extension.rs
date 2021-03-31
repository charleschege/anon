use codec::{Encode, Decode};

use frame_support::debug::{error, native};
use frame_support::traits::Randomness;
use pallet_contracts::chain_extension::{
    ChainExtension, Environment, Ext, InitState, RetVal, SysConfig, UncheckedFrom,
};
use sp_runtime::DispatchError;
use sp_std::vec::Vec;
use pallet_merkle::traits::Group;

pub trait Config: frame_system::Config + pallet_merkle::Config {
	type Group: Group<Self::AccountId, Self::BlockNumber, Self::GroupId>;
}

/// contract extension for `FetchRandom`
pub struct PalletMerkleExtension;

impl Config for PalletMerkleExtension {}

impl ChainExtension for PalletMerkleExtension {
    fn call<E: Ext>(func_id: u32, env: Environment<E, InitState>) -> Result<RetVal, DispatchError>
        where
            <E::T as SysConfig>::AccountId: UncheckedFrom<<E::T as SysConfig>::Hash> + AsRef<[u8]>,
    {

        match func_id {
            1101 => {
            let mut env = env.buf_in_buf_out();
                pallet_merkle::Pallet<T>::create_group()?
            },
            1102 => {
            let mut env = env.buf_in_buf_out();
                pallet_merkle::Pallet<T>::add_members(AccountId, GroupId)?
            },
            1103 => {
            let mut env = env.buf_in_buf_out();
                pallet_merkle::Pallet<T>::verify_group()?
            },
            _ => {
                error!("call an unregistered `func_id`, func_id:{:}", func_id);
                return Err(DispatchError::Other("Unimplemented func_id"));
            }
        }
        Ok(RetVal::Converging(0))
    }

    fn enabled() -> bool {
        true
    }
}
