use ink_lang as ink;
use ink_prelude::{string::String, vec::Vec};

#[ink::contract]
mod time_locked_swap {
    use ink_prelude::format;
    use @openzeppelin/contracts/token/ERC20/IERC20.sol;

    #[ink(storage)]
    pub struct TimeLockedSwap {
        egld_token: contract<IElrondToken>,
        party_a: AccountId,
        party_b: AccountId,
        unlock_timestamp: Timestamp,
        locked: bool,
    }

    impl TimeLockedSwap {
        #[ink(constructor)]
        pub fn new(party_b: AccountId, unlock_timestamp: Timestamp, egld_token: contract<IElrondToken>) -> Self {
            Self {
                egld_token,
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
        pub fn execute_swap(&mut self, egld_amount: Balance) {
            let caller = self.env().caller();
            assert_eq!(caller, self.party_b, "Not authorized to execute swap");
            assert!(!self.locked, "Contract not yet unlocked");
            let transfer_result = self.egld_token.transfer(self.party_a, egld_amount);
            assert_eq!(transfer_result, true, "Token transfer failed");
            // Additional swap logic if needed