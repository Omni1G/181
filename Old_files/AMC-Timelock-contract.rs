// SPDX-License-Identifier: MIT
#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;
use ink_prelude::{string::String, vec::Vec};

#[ink::contract]
mod time_locked_swap {
    use @openzeppelin/contracts/token/ERC20/IERC20.sol;
    use @openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol;

    #[ink(storage)]
    pub struct TimeLockedSwap {
        amc_token: contract<IERC20>,
        party_a: AccountId,
        party_b: AccountId,
        unlock_timestamp: Timestamp,
        locked: bool,
    }

    impl TimeLockedSwap {
        #[ink(constructor)]
        pub fn new(party_b: AccountId, unlock_timestamp: Timestamp, amc_token: contract<IERC20>) -> Self {
            Self {
                amc_token,
                party_a: Self::env().caller(),
                party_b,
                unlock_timestamp,
                locked: true,
            }
        }

        #[ink(message)]
        pub fn unlock(&mut self) {
            let caller = self.env().caller();
            assert_eq!(caller, self.party_a, "Not authorized");
            assert!(self.env().block_timestamp() >= self.unlock_timestamp, "Unlock time not reached");
            self.locked = false;
        }

        #[ink(message)]
        pub fn execute_swap(&mut self, amount: Balance) {
            let caller = self.env().caller();
            assert_eq!(caller, self.party_b, "Not authorized to execute swap");
            assert!(!self.locked, "Contract not yet unlocked");
            let transfer_result = self.amc_token.transfer_from(self.party_a, self.party_b, amount);
            assert_eq!(transfer_result, true, "Token transfer failed");
            // Additional swap logic if needed
        }
    }
}
