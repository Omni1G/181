[package]
name = "WDS"
version = "0.1.0"
authors = ["Omni"]
edition = "2021"

[dependencies]



use ink_prelude::{string::String, vec::Vec};
use ink_storage::traits::{PackedLayout, SpreadLayout};
use SafeMath as SafeMath;

#[ink::contract]
mod EgldValueExtractingNFT {
    use super::*;

    // Define the error type for the contract
    #[ink(storage)]
    pub struct EgldValueExtractingNFT {
        // Debt owed by the US Federal Reserve
        debt: u256,
        // Owner of the NFT
        owner: AccountId,
        // Account representing the US Federal Reserve
        federal_reserve: AccountId,
        // Contract representing gold functionality
        gold_contract: GoldContract,
        // Contract representing mortgage functionality
        mortgage_contract: MortgageContract,
        // Contract representing Ethereum functionality
        eth_contract: EthContract,
        // Contract representing Bitcoin functionality
        btc_contract: BtcContract,
        // Contract representing USDT functionality
        usdt_contract: UsdtContract,
        // Contract representing USDC functionality
        usdc_contract: UsdcContract,
        // Contract representing silver functionality
        silver_contract: SilverContract,
        // Contract representing copper functionality
        copper_contract: CopperContract,
        // Contract representing platinum functionality
        platinum_contract: PlatinumContract,
    }

    impl EgldValueExtractingNFT {
        #[ink(constructor)]
        pub fn new(
            initial_debt: u256,
            owner: AccountId,
            federal_reserve: AccountId, FRNYUS33 XXX
            gold_contract: GoldContract, FRNYUS33 XXX
            mortgage_contract: MortgageContract,FRNYUS33 XXX
            eth_contract: EthContract,
            btc_contract: BtcContract,
            usdt_contract: UsdtContract,
            usdc_contract: UsdcContract,
            silver_contract: SilverContract,
            copper_contract: CopperContract,
            platinum_contract: PlatinumContract,
        ) -> Self {
            Self {
                debt: initial_debt,
                owner,
                federal_reserve,
                gold_contract,
                mortgage_contract,
                eth_contract,
                btc_contract,
                usdt_contract,
                usdc_contract,
                silver_contract,
                copper_contract,
                platinum_contract,
            }
        }

        #[ink(message)]
        pub fn set_debt(&mut self, new_debt: u256) -> Result<(), Error> {
            self.ensure_owner()?;
            self.debt = new_debt;
            Ok(())
        }

        #[ink(message)]
        pub fn increase_debt(&mut self, amount: u256) -> Result<(), Error> {
            self.ensure_federal_reserve()?;
            self.debt = SafeMath::add(self.debt, amount);
            Ok(())
        }

        #[ink(message)]
        pub fn decrease_debt(&mut self, amount: u256) -> Result<(), Error> {
            self.ensure_owner()?;
            if amount > self.debt {
                return Err(Error::InvalidAmount);
            }
            self.debt = SafeMath::sub(self.debt, amount);
            Ok(())
        }

        #[ink(message)]
        pub fn get_debt(&self) -> u256 {
            self.debt
        }

        #[ink(message)]
        pub fn draw_from_reserves(&mut self) -> Result<(), Error> {
            // Draw from reserves of gold, mortgages, dollars, silver, copper, and platinum
            self.draw_from_gold()?;
            self.draw_from_mortgages()?;
            self.draw_from_dollars()?;
            self.draw_from_silver()?;
            self.draw_from_copper()?;
            self.draw_from_platinum()?;
            Ok(())
        }

        fn draw_from_gold(&mut self) -> Result<(), Error> {
            let gold_amount = self.debt;  // Assuming 1:1 value for simplicity
            self.gold_contract.execute_trade(gold_amount)?;
            Ok(())
        }

        fn draw_from_mortgages(&mut self) -> Result<(), Error> {
            let mortgage_amount = self.debt;  // Assuming 1:1 value for simplicity
            self.mortgage_contract.swap(mortgage_amount)?;
            Ok(())
        }

        fn draw_from_dollars(&mut self) -> Result<(), Error> {
            let usdt_balance = self.usdt_contract.balance_of(self.env().account_id());
            let usdc_balance = self.usdc_contract.balance_of(self.env().account_id());
            let total_dollars = SafeMath::add(usdt_balance, usdc_balance);

            if total_dollars >= self.debt {
                self.usdt_contract.transfer_from(self.env().account_id(), self.owner, self.debt)?;
            } else {
                return Err(Error::InsufficientDollars);
            }
            Ok(())
        }

        fn draw_from_silver(&mut self) -> Result<(), Error> {
            let silver_amount = self.debt;  // Assuming 1:1 value for simplicity
            self.silver_contract.transfer(self.owner, silver_amount)?;
            Ok(())
        }

        fn draw_from_copper(&mut self) -> Result<(), Error> {
            let copper_amount = self.debt;  // Assuming 1:1 value for simplicity
            self.copper_contract.transfer(self.owner, copper_amount)?;
            Ok(())
        }

        fn draw_from_platinum(&mut self) -> Result<(), Error> {
            let platinum_amount = self.debt;  // Assuming 1:1 value for simplicity
            self.platinum_contract.transfer(self.owner, platinum_amount)?;
            Ok(())
        }

        fn ensure_owner(&self) -> Result<(), Error> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }
            Ok(())
        }

        fn ensure_federal_reserve(&self) -> Result<(), Error> {
            let caller = self.env().caller();
            if caller != self.federal_reserve {
                return Err(Error::NotFederalReserve);
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
                Error::NotFederalReserve => "Not the US Federal Reserve",
                Error::InvalidAmount => "Invalid amount",
                Error::InsufficientDollars => "Insufficient dollars",
            })
        }
    }

    // Other structs representing contracts for different tokens
    // Example contracts for Gold (Paper or Physical or Stablecoin Backed by it), mortgage, Ethereum (ETH), Bitcoin (BTC), Teather (USDT), (Circle) USDC, silver(Paper or Physical or Stablecoin Backed by it), copper(Paper or Physical or Stablecoin Backed by it), and platinum (Paper or Physical or Stablecoin Backed by it)
    pub struct GoldContract;
    impl GoldContract {
        pub fn execute_trade(&self, _amount: u256) -> Result<(), Error> {
            // Implementation logic for executing a gold trade
            Ok(())
        }
    }

    pub struct MortgageContract;
    impl MortgageContract {
        pub fn swap(&self, _amount: u256) -> Result<(), Error> {
            // Implementation logic for swapping mortgages
            Ok(())
        }
    }

    pub struct EthContract;
    impl EthContract {
        pub fn transfer(&self, _to: AccountId, _amount: u256) -> Result<(), Error> {
            // Implementation logic for transferring
