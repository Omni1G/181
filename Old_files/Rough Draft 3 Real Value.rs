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
    use ink_lang as ink;
    use ink_prelude::vec::Vec;
    // SPDX-License-Identifier: MIT
#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;
use ink_prelude::vec::Vec;

#[ink::contract]
mod mortgage_swap_contract {
    use ink_prelude::string::String;
    use ink_prelude::vec::Vec;
    use ink_prelude::format;
    use ink_storage::collections::HashMap;

    #[ink(storage)]
    pub struct MortgageSwapContract {
        mortgage_token: AccountId,
        crypto_token: AccountId,
        owner: AccountId,
        total_mortgage_value: u256,
        total_crypto_value: u256,
        mortgage_balances: HashMap<AccountId, u256>,
        crypto_balances: HashMap<AccountId, u256>,
    }

    impl MortgageSwapContract {
        #[ink(constructor)]
        pub fn new(
            mortgage_token: AccountId,
            crypto_token: AccountId,
        ) -> Self {
            Self {
                mortgage_token,
                crypto_token,
                owner: Self::env().caller(),
                total_mortgage_value: 0,
                total_crypto_value: 0,
                mortgage_balances: HashMap::new(),
                crypto_balances: HashMap::new(),
            }
        }

        #[ink(message)]
        pub fn deposit_mortgages(&mut self, amount: u256) -> Result<(), Error> {
            self.ensure_positive_amount(amount)?;

            let caller = self.env().caller();
            self.transfer_from_mortgage(caller, amount)?;

            let balance = self.mortgage_balances.entry(caller).or_insert(0);
            *balance += amount;
            self.total_mortgage_value += amount;

            Ok(())
        }

        #[ink(message)]
        pub fn withdraw_mortgages(&mut self, amount: u256) -> Result<(), Error> {
            self.ensure_positive_amount(amount)?;

            let caller = self.env().caller();
            let balance = self.mortgage_balances.entry(caller).or_insert(0);
            *balance = balance.saturating_sub(amount);

            self.total_mortgage_value = self.total_mortgage_value.saturating_sub(amount);

            self.transfer_to_mortgage(caller, amount)?;

            Ok(())
        }

        #[ink(message)]
        pub fn deposit_crypto(&mut self, amount: u256) -> Result<(), Error> {
            self.ensure_positive_amount(amount)?;

            let caller = self.env().caller();
            self.transfer_from_crypto(caller, amount)?;

            let balance = self.crypto_balances.entry(caller).or_insert(0);
            *balance += amount;
            self.total_crypto_value += amount;

            Ok(())
        }

        #[ink(message)]
        pub fn withdraw_crypto(&mut self, amount: u256) -> Result<(), Error> {
            self.ensure_positive_amount(amount)?;

            let caller = self.env().caller();
            let balance = self.crypto_balances.entry(caller).or_insert(0);
            *balance = balance.saturating_sub(amount);

            self.total_crypto_value = self.total_crypto_value.saturating_sub(amount);

            self.transfer_to_crypto(caller, amount)?;

            Ok(())
        }

        #[ink(message)]
        pub fn swap(&mut self, mortgage_amount: u256) -> Result<(), Error> {
            self.ensure_positive_amount(mortgage_amount)?;

            let caller = self.env().caller();

            let mortgage_balance = *self.mortgage_balances.get(&caller).unwrap_or(&0);
            let crypto_balance = *self.crypto_balances.get(&caller).unwrap_or(&0);

            assert!(mortgage_amount <= mortgage_balance, "Insufficient mortgage balance");
            
            let crypto_amount = mortgage_amount
                .checked_mul(self.total_crypto_value)
                .and_then(|val| val.checked_div(self.total_mortgage_value))
                .ok_or(Error::ArithmeticOverflow)?;

            assert!(crypto_amount <= crypto_balance, "Insufficient crypto balance");

            let mortgage_balance = self.mortgage_balances.entry(caller).or_insert(0);
            *mortgage_balance -= mortgage_amount;

            let crypto_balance = self.crypto_balances.entry(caller).or_insert(0);
            *crypto_balance += crypto_amount;

            self.total_mortgage_value -= mortgage_amount;
            self.total_crypto_value += crypto_amount;

            Ok(())
        }

        fn transfer_from_mortgage(&self, from: AccountId, amount: u256) -> Result<(), Error> {
            self.transfer_from_token(self.mortgage_token, from, amount)
        }

        fn transfer_to_mortgage(&self, to: AccountId, amount: u256) -> Result<(), Error> {
            self.transfer_to_token(self.mortgage_token, to, amount)
        }

        fn transfer_from_crypto(&self, from: AccountId, amount: u256) -> Result<(), Error> {
            self.transfer_from_token(self.crypto_token, from, amount)
        }

        fn transfer_to_crypto(&self, to: AccountId, amount: u256) -> Result<(), Error> {
            self.transfer_to_token(self.crypto_token, to, amount)
        }

        fn transfer_from_token(&self, token: AccountId, from: AccountId, amount: u256) -> Result<(), Error> {
            let result = Self::env().transfer_from(token, from, self.env().account_id(), amount);
            if !result {
                return Err(Error::TransferFailed);
            }
            Ok(())
        }

        fn transfer_to_token(&self, token: AccountId, to: AccountId, amount: u256) -> Result<(), Error> {
            let result = Self::env().transfer(token, to, amount);
            if !result {
                return Err(Error::TransferFailed);
            }
            Ok(())
        }

        fn ensure_positive_amount(&self, amount: u256) -> Result<(), Error> {
            if amount == 0 {
                return Err(Error::ZeroAmount);
            }
            Ok(())
        }
    }

    #[ink(event)]
    pub struct Error {
        #[ink(topic)]
        message: String,
    }

    impl From<Error> for ink_env::Error {
        fn from(error: Error) -> Self {
            ink_env::Error::from(match error {
                Error::ZeroAmount => "Zero amount",
                Error::TransferFailed => "Transfer failed",
                Error::ArithmeticOverflow => "Arithmetic overflow",
            })
        }
    }
}

    #[ink::contract]
    mod isda_contract {
        use ink_prelude::string::String;
        use ink_prelude::vec::Vec;
        use ink_prelude::format;
        use ink_storage::collections::HashMap;
    
        #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
        #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
        pub enum State {
            Created,
            Executed,
            Settled,
        }
    
        #[ink(storage)]
        pub struct ISDAContract {
            gold_address: AccountId,
            crypto_address: AccountId,
            counterparty1: AccountId,
            counterparty2: AccountId,
            gold_amount: u256,
            crypto_amount: u256,
            exchange_rate: u256,
            state: State,
        }
    
        impl ISDAContract {
            #[ink(constructor)]
            pub fn new(
                gold_address: AccountId,
                crypto_address: AccountId,
                counterparty1: AccountId,
                counterparty2: AccountId,
                gold_amount: u256,
                crypto_amount: u256,
                exchange_rate: u256,
            ) -> Self {
                Self {
                    gold_address,
                    crypto_address,
                    counterparty1,
                    counterparty2,
                    gold_amount,
                    crypto_amount,
                    exchange_rate,
                    state: State::Created,
                }
            }
    
            #[ink(message)]
            pub fn execute_trade(&mut self) -> Result<(), Error> {
                self.ensure_counterparty()?;
                self.ensure_state(State::Created)?;
    
                self.transfer_gold()?;
                self.transfer_crypto()?;
    
                self.state = State::Executed;
                Ok(())
            }
    
            #[ink(message)]
            pub fn settle(&mut self) -> Result<(), Error> {
                self.ensure_counterparty()?;
                self.ensure_state(State::Executed)?;
    
                let gold_settlement = self.crypto_amount
                    .checked_mul(self.exchange_rate)
                    .map(|val| val / 1e18)
                    .ok_or(Error::ArithmeticOverflow)?;
    
                let crypto_settlement = self.gold_amount
                    .checked_mul(1e18)
                    .map(|val| val / self.exchange_rate)
                    .ok_or(Error::ArithmeticOverflow)?;
    
                self.transfer_gold_settlement(gold_settlement)?;
                self.transfer_crypto_settlement(crypto_settlement)?;
    
                self.state = State::Settled;
                Ok(())
            }
    
            fn ensure_counterparty(&self) -> Result<(), Error> {
                let caller = self.env().caller();
                if caller != self.counterparty1 && caller != self.counterparty2 {
                    return Err(Error::Unauthorized);
                }
                Ok(())
            }
    
            fn ensure_state(&self, required_state: State) -> Result<(), Error> {
                if self.state != required_state {
                    return Err(Error::InvalidState);
                }
                Ok(())
            }
    
            fn transfer_gold(&self) -> Result<(), Error> {
                let transfer_result = self.env().transfer_from(
                    self.gold_address,
                    self.counterparty1,
                    self.counterparty2,
                    self.gold_amount,
                );
                if !transfer_result {
                    return Err(Error::GoldTransferFailed);
                }
                Ok(())
            }
    
            fn transfer_crypto(&self) -> Result<(), Error> {
                let transfer_result = self.env().transfer_from(
                    self.crypto_address,
                    self.counterparty2,
                    self.counterparty1,
                    self.crypto_amount,
                );
                if !transfer_result {
                    return Err(Error::CryptoTransferFailed);
                }
                Ok(())
            }
    
            fn transfer_gold_settlement(&self, amount: u256) -> Result<(), Error> {
                let transfer_result = self.env().transfer_from(
                    self.gold_address,
                    self.counterparty2,
                    self.counterparty1,
                    amount,
                );
                if !transfer_result {
                    return Err(Error::GoldSettlementTransferFailed);
                }
                Ok(())
            }
    
            fn transfer_crypto_settlement(&self, amount: u256) -> Result<(), Error> {
                let transfer_result = self.env().transfer_from(
                    self.crypto_address,
                    self.counterparty1,
                    self.counterparty2,
                    amount,
                );
                if !transfer_result {
                    return Err(Error::CryptoSettlementTransferFailed);
                }
                Ok(())
            }
        }
    
        #[ink(event)]
        pub struct Error {
            #[ink(topic)]
            message: String,
        }
    
        impl From<Error> for ink_env::Error {
            fn from(error: Error) -> Self {
                ink_env::Error::from(match error {
                    Error::Unauthorized => "Unauthorized access",
                    Error::InvalidState => "Invalid state",
                    Error::GoldTransferFailed => "Gold transfer failed",
                    Error::CryptoTransferFailed => "Cryptocurrency transfer failed",
                    Error::GoldSettlementTransferFailed => "Gold settlement transfer failed",
                    Error::CryptoSettlementTransferFailed => "Cryptocurrency settlement transfer failed",
                    Error::ArithmeticOverflow => "Arithmetic overflow",
                })
            }
        }
    }
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
[ink::contract]
mod EgldValueExtractingNFT {
    use ink_prelude::{string::String, vec::Vec};
    use ink_storage::traits::{PackedLayout, SpreadLayout};
    use ink_storage::collections::HashMap;
    use IERC20;
    use IERC721;
    use ERC721Holder;
    use SafeMath;

    #[ink(storage)]
    pub struct EgldValueExtractingNFT {
        value: u256,  // The value stored in the NFT
        egld_token: contract<IERC20>,  // EGLD token contract
        usdt_token: contract<IERC20>,  // USDT token contract
        usdc_token: contract<IERC20>,  // USDC token contract
        mortgage_contract: contract<MortgageSwapContract>,  // Mortgage contract
        gold_contract: contract<ISDAContract>,  // Gold contract
        owner: AccountId,  // Owner of the NFT
    }

    impl EgldValueExtractingNFT {
        #[ink(constructor)]
        pub fn new(
            initial_value: u256,
            egld_token: contract<IERC20>,
            usdt_token: contract<IERC20>,
            usdc_token: contract<IERC20>,
            mortgage_contract: contract<MortgageSwapContract>,
            gold_contract: contract<ISDAContract>,
        ) -> Self {
            Self {
                value: initial_value,
                egld_token,
                usdt_token,
                usdc_token,
                mortgage_contract,
                gold_contract,
                owner: Self::env().caller(),
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
        pub fn draw_from_reserves(&mut self) -> Result<(), Error> {
            self.ensure_owner()?;
            // Draw from reserves of gold, mortgages, and dollars
            self.draw_from_gold()?;
            self.draw_from_mortgages()?;
            self.draw_from_dollars()?;
            Ok(())
        }

        fn draw_from_gold(&mut self) -> Result<(), Error> {
            let gold_amount = self.value;  // Assuming 1:1 value for simplicity
            self.gold_contract.execute_trade(gold_amount)?;
            Ok(())
        }

        fn draw_from_mortgages(&mut self) -> Result<(), Error> {
            let mortgage_amount = self.value;  // Assuming 1:1 value for simplicity
            self.mortgage_contract.swap(mortgage_amount)?;
            Ok(())
        }

        fn draw_from_dollars(&mut self) -> Result<(), Error> {
            let usdt_balance = self.usdt_token.balance_of(self.env().account_id());
            let usdc_balance = self.usdc_token.balance_of(self.env().account_id());
            let total_dollars = SafeMath::add(usdt_balance, usdc_balance);

            if total_dollars >= self.value {
                self.usdt_token.transfer_from(self.env().account_id(), self.owner, self.value)?;
            } else {
                return Err(Error::InsufficientDollars);
            }
            Ok(())
        }

        fn ensure_owner(&self) -> Result<(), Error> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }
            Ok(())
        }
    }

    #[ink(event)]
    pub struct Error {
        #[ink(topic)]
        message: String,
    }

    impl From<Error> for ink_env::Error {
        fn from(error: Error) -> Self {
            ink_env::Error::from(match error {
                Error::NotOwner => "Not the owner",
                Error::InsufficientDollars => "Insufficient dollars",
            })
        }
    }
}
#[ink::contract]
mod EgldValueExtractingNFT {
    use ink_prelude::{string::String, vec::Vec};
    use ink_storage::collections::HashMap;
    use IERC20;
    use IERC721;
    use ERC721Holder;
    use SafeMath;

    #[ink(storage)]
    pub struct EgldValueExtractingNFT {
        value: u256,  // The value stored in the NFT, representing the debt owed
        us_fed_reserve: AccountId,  // Account representing the US Federal Reserve
        owner: AccountId,  // Owner of the NFT
    }

    impl EgldValueExtractingNFT {
        #[ink(constructor)]
        pub fn new(initial_value: u256, us_fed_reserve: AccountId) -> Self {
            Self {
                value: initial_value,
                us_fed_reserve,
                owner: Self::env().caller(),
            }
        }

        #[ink(message)]
        pub fn set_value(&mut self, new_value: u256) -> Result<(), Error> {
            self.ensure_owner()?;
            self.value = new_value;
            Ok(())
        }

        #[ink(message)]
        pub fn issue_debt(&mut self, amount: u256) -> Result<(), Error> {
            self.ensure_owner()?;
            self.value = SafeMath::add(self.value, amount);
            Ok(())
        }

        #[ink(message)]
        pub fn settle_debt(&mut self, amount: u256) -> Result<(), Error> {
            self.ensure_owner()?;
            if amount > self.value {
                return Err(Error::InvalidAmount);
            }
            self.value = SafeMath::sub(self.value, amount);
            self.transfer_payment(amount)?;
            Ok(())
        }

        #[ink(message)]
        pub fn get_debt_value(&self) -> u256 {
            self.value
        }

        fn transfer_payment(&self, amount: u256) -> Result<(), Error> {
            let result = self.env().transfer(self.us_fed_reserve, amount);
            if !result {
                return Err(Error::PaymentTransferFailed);
            }
            Ok(())
        }

        fn ensure_owner(&self) -> Result<(), Error> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }
            Ok(())
        }
    }

    #[ink(event)]
    pub struct Error {
        #[ink(topic)]
        message: String,
    }

    impl From<Error> for ink_env::Error {
        fn from(error: Error) -> Self {
            ink_env::Error::from(match error {
                Error::NotOwner => "Not the owner",
                Error::InvalidAmount => "Invalid amount",
                Error::PaymentTransferFailed => "Payment transfer failed",
            })
        }
    }