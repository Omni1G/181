// SPDX-License-Identifier: MIT
#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;
use @openzeppelin/contracts/token/ERC20/IERC20.sol as Erc20;
use @openzeppelin/contracts/token/ERC721/IERC721.sol as Erc721;
use @openzeppelin/contracts/token/ERC721/utils/ERC721Holder.sol as Erc721Holder;
use @openzeppelin/contracts/utils/math/SafeMath.sol as SafeMath;

mod egld_value_extracting_nft {
    use ink_prelude::string::String;
    use @openzeppelin/contracts/token/ERC721/ERC721.sol;
    use @openzeppelin/contracts/access/Ownable.sol;
    use @openzeppelin/contracts/token/ERC20/IERC20.sol;

    #[ink(storage)]
    pub struct EgldValueExtractingNFT {
        value: u256,  // The value stored in the NFT
        egld_token: contract<IElrondToken>,  // EGLD token contract
    }

    impl EgldValueExtractingNFT {
        #[ink(constructor)]
        pub fn new(name: String, symbol: String, initial_value: u256, egld_token: contract<IElrondToken>) -> Self {
            Self {
                value: initial_value,
                egld_token,
            }
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

mod nft_liquidity_counter {
    use ink_prelude::{string::String, vec::Vec};
    use ink_storage::collections::HashMap;
    use Erc20 as IERC20;
    use Erc721 as IERC721;
    use Erc721Holder as ERC721Holder;
    use SafeMath as SafeMath;

    #[ink(storage)]
    pub struct NFTLiquidityCounter {
        egld_token: contract<IERC20>,
        nft_token: contract<IERC721>,
        egld_per_coin: u256,
        total_liquidity: u256,
        user_liquidity: ink_storage::collections::HashMap<AccountId, u256>,
    }

    impl NFTLiquidityCounter {
        #[ink(constructor)]
        pub fn new(egld_token: contract<IERC20>, nft_token: contract<IERC721>, egld_per_coin: u256) -> Self {
            Self {
                egld_token,
                nft_token,
                egld_per_coin,
                total_liquidity: 0,
                user_liquidity: Default::default(),
            }
        }

        #[ink(message)]
        pub fn deposit_liquidity(&mut self, nft_id: u256, amount: u256) {
            let caller = self.env().caller();
            assert!(self.nft_token.owner_of(nft_id) == caller, "Not the NFT owner");
            assert!(self.egld_token.balance_of(caller) >= amount, "Insufficient EGLD balance");

            self.egld_token.transfer_from(caller, self.env().account_id(), amount);
            self.nft_token.safe_transfer_from(caller, self.env().account_id(), nft_id);

            let liquidity = amount.checked_mul(self.egld_per_coin).unwrap();
            self.total_liquidity += liquidity;
            *self.user_liquidity.entry(caller).or_insert(0) += liquidity;

            self.env().emit_event(LiquidityDeposited {
                depositor: caller,
                amount: liquidity,
            });
        }

        #[ink(message)]
        pub fn withdraw_liquidity(&mut self, nft_id: u256) {
            let caller = self.env().caller();
            let liquidity = *self.user_liquidity.get(&caller).unwrap_or(&0);
            assert!(liquidity > 0, "No liquidity deposited");

            let amount = liquidity.checked_div(self.egld_per_coin).unwrap();

            self.total_liquidity -= liquidity;
            self.user_liquidity.insert(caller, 0);

            self.nft_token.safe_transfer_from(self.env().account_id(), caller, nft_id);
            self.egld_token.transfer(caller, amount);

            self.env().emit_event(LiquidityWithdrawn {
                withdrawer: caller,
                amount: liquidity,
            });
        }
    }

    #[ink(event)]
    pub struct LiquidityDeposited {
        #[ink(topic)]
        depositor: AccountId,
        amount: u256,
    }

    #[ink(event)]
    pub struct LiquidityWithdrawn {
        #[ink(topic)]
        withdrawer: AccountId,
        amount: u256,
    }
}
