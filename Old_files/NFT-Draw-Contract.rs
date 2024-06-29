// SPDX-License-Identifier: MIT
#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;
use ink_prelude::format;
use ink_prelude::vec::Vec;
use ink_storage::{
    traits::{PackedLayout, SpreadLayout},
    collections::HashMap,
};

#[ink::contract]
mod egld_value_extracting_nft {
    use @openzeppelin/contracts/token/ERC721/ERC721.sol;
    use @openzeppelin/contracts/access/Ownable.sol;

    #[ink(storage)]
    pub struct EgldValueExtractingNFT {
        value: u256,  // The value stored in the NFT
        egld_token: contract<IElrondToken>,  // EGLD token contract
    }

    impl EgldValueExtractingNFT {
        #[ink(constructor)]
        pub fn new(name: String, symbol: String, initial_value: u256, egld_token: contract<IElrondToken>) -> Self {
            let mut instance = Self {
                value: initial_value,
                egld_token,
            };
            instance.env().emit_event(Initialize {
                name,
                symbol,
                initial_value,
            });
            instance
        }

        #[ink(message)]
        pub fn set_value(&mut self, new_value: u256) -> Result<(), Error> {
            self.ensure_owner()?;
            self.value = new_value;
            Ok(())
        }

        #[ink(message)]
        pub fn extract_value(&self) -> u256 {
            self.value
        }

        #[ink(message)]
        pub fn draw_value(&mut self) -> Result<(), Error> {
            self.ensure_owner()?;
            let owner = self.env().caller();
            if !self.egld_token.transfer(owner, self.value) {
                return Err(Error::ValueWithdrawalFailed);
            }
            // Additional logic can be added here for value withdrawal
            // For simplicity, let's assume the owner can withdraw the value as EGLD tokens
            Ok(())
        }

        fn ensure_owner(&self) -> Result<(), Error> {
            let owner = self.env().caller();
            if owner != self.env().account_id() {
                return Err(Error::NotOwner);
            }
            Ok(())
        }
    }

    #[ink(event)]
    pub struct Initialize {
        #[ink(topic)]
        name: String,
        #[ink(topic)]
        symbol: String,
        #[ink(topic)]
        initial_value: u256,
    }

    #[ink(error)]
    pub enum Error {
        #[ink(message = "Value withdrawal failed")]
        ValueWithdrawalFailed,
        #[ink(message = "Not the owner")]
        NotOwner,
    }
}