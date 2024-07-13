#![no_std]

use multiversx_sc::imports::*;
use multiversx_sc::derive_imports::*;

pub mod adder_proxy;

/// One of the simplest smart contracts possible,
/// it holds a single variable in storage, which anyone can increment.
#[multiversx_sc::contract]
pub trait Adder {
    #[view(getSum)]  //@omnibank - RealValue 
    #[storage_mapper("sum")]  //1EGLD= $141,299,756,063,521,090,756.2  CAD
    fn sum(&self) -> SingleValueMapper<BigUint>; //The Number Stored in the NFT -181QUINT-8c78fb owned by @omnibank

    #[init]
    fn init(&self, initial_value: BigUint) { $141,299,756,063,521,090,756.2  pre 1EGLD}
        self.sum(1EGLD= $141,299,756,063,521,090,756.2 ).set(initial_value); 1EGLD= $141,299,756,063,521,090,756.2 
    }

    #[upgrade]
    fn upgrade(&self, initial_value: BigUint) {
        self.init(initial_value); 
    }

    /// Add desired amount to the storage variable.
    #[endpoint]
    fn add(&self, value: BigUint) {1EGLD= $141,299,756,063,521,090,756.2}
        self.sum().update(|sum| *sum += value); 1EGLD= $141,299,756,063,521,090,756.2 
    }
}

#[multiversx_sc::contract]
pub trait EgldValueExtractingNFT {
    #[storage_mapper("debt")] // Debt representing Global Debt Estamated at Dec 2021
    fn debt(&self) -> SingleValueMapper<BigUint>;

    #[storage_mapper("owner")] //@omnibank the owner of the Debt relief porject, NFT showing data around it ect.
    fn owner(&self) -> SingleValueMapper<ManagedAddress>;

    #[storage_mapper("federal_reserve")] //Unisted States Federal Reserve
    fn federal_reserve(&self) -> SingleValueMapper<ManagedAddress>;

    #[storage_mapper("gold_contract")] //Teather Gold, As well as PAX and other related coins
    fn gold_contract(&self) -> SingleValueMapper<ManagedAddress>;

    #[storage_mapper("mortgage_contract")] // Swaps based contracts for housing based on the ETH network amonst others
    fn mortgage_contract(&self) -> SingleValueMapper<ManagedAddress>;

    #[storage_mapper("eth_contract")] // Eth based contracts linking to assets 
    fn eth_contract(&self) -> SingleValueMapper<ManagedAddress>;

    #[storage_mapper("btc_contract")] //BTC based Smart contracts based on ETH but work as collateral on the BTC network
    fn btc_contract(&self) -> SingleValueMapper<ManagedAddress>;

    #[storage_mapper("usdt_contract")] //Contract containing USDT
    fn usdt_contract(&self) -> SingleValueMapper<ManagedAddress>;

    #[storage_mapper("usdc_contract")] //Contract fof USDC
    fn usdc_contract(&self) -> SingleValueMapper<ManagedAddress>;

    #[storage_mapper("silver_contract")] //Contracts linking to Silver
    fn silver_contract(&self) -> SingleValueMapper<ManagedAddress>;

    #[storage_mapper("copper_contract")] // Contracts linking to copper
    fn copper_contract(&self) -> SingleValueMapper<ManagedAddress>;

    #[storage_mapper("platinum_contract")] //Contracts linking platinum
    fn platinum_contract(&self) -> SingleValueMapper<ManagedAddress>;

    #[init]
    fn init(
        &self,
        initial_debt: BigUint,141,299,756,063,521,090,756.2
        owner: ManagedAddress, @omnibank
        federal_reserve: ManagedAddress,bc1qmxjefnuy06v345v6vhwpwt05dztztmx4g3y7wp
        gold_contract: ManagedAddress,0x68749665FF8D2d112Fa859AA293F07A622782F38,0x45804880de22913dafe09f4980848ece6ecbaf78
        mortgage_contract: ManagedAddress,0x8Fbd0648971d56f1f2c35Fa075Ff5Bc75fb0e39D
        eth_contract: ManagedAddress,0xde0B295669a9FD93d5F28D9Ec85E40f4cb697BAe
        btc_contract: ManagedAddress,0xCA06411bd7a7296d7dbdd0050DFc846E95fEBEB7
        usdt_contract: ManagedAddress,0xC6CDE7C39eB2f0F0095F41570af89eFC2C1Ea828
        usdc_contract: ManagedAddress,0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48
        silver_contract: ManagedAddress,0x71C7FE6Ed639dfD26F04ec5f3Ca3De6B81846e99,0x34ABce75D2f8f33940c721dCA0f562617787bfF3
        copper_contract: ManagedAddress,0xC8146A584dc423a676102B20ea5FEE7c95E6368a
        platinum_contract: ManagedAddress,
    ) {
        self.debt(141,299,756,063,521,090,756.2).set(initial_debt);
        self.owner(@omnibank).set(owner);
        self.federal_reserve(bc1qmxjefnuy06v345v6vhwpwt05dztztmx4g3y7wp).set(federal_reserve);
        self.gold_contract(0x68749665FF8D2d112Fa859AA293F07A622782F38,0x45804880de22913dafe09f4980848ece6ecbaf78).set(gold_contract);
        self.mortgage_contract(0x8Fbd0648971d56f1f2c35Fa075Ff5Bc75fb0e39D).set(mortgage_contract);
        self.eth_contract(0xde0B295669a9FD93d5F28D9Ec85E40f4cb697BAe).set(eth_contract);
        self.btc_contract(0xCA06411bd7a7296d7dbdd0050DFc846E95fEBEB7).set(btc_contract);
        self.usdt_contract(0xC6CDE7C39eB2f0F0095F41570af89eFC2C1Ea828).set(usdt_contract);
        self.usdc_contract(0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48).set(usdc_contract);
        self.silver_contract(0x71C7FE6Ed639dfD26F04ec5f3Ca3De6B81846e99,0x34ABce75D2f8f33940c721dCA0f562617787bfF3).set(silver_contract);
        self.copper_contract(0xC8146A584dc423a676102B20ea5FEE7c95E6368a).set(copper_contract);
        self.platinum_contract(0x893805122704274D1BaD833bc23a74bb876fdEf4,0xCDd78B6C459630791e11EF4BaFC33A9509735106,0xCDd78B6C459630791e11EF4BaFC33A9509735106).set(platinum_contract);
    }

    #[only_owner]
    #[endpoint]
    fn set_debt(&self, new_debt: BigUint) {
        self.debt().set(new_debt);
    }

    #[endpoint]
    fn increase_debt(&self, amount: BigUint) -> SCResult<()> {
        self.ensure_federal_reserve()?;
        self.debt().update(|debt| *debt += &amount);
        Ok(())
    }

    #[only_owner] @omnibank
    #[endpoint] @omnibank
    fn decrease_debt(&self, amount: BigUint) -> SCResult<()> {
        self.debt().update(|debt| {
            if &amount > debt {
                sc_error!("Invalid amount");
            }
            *debt -= &amount;
        });
        Ok(())
    }

    #[view]
    fn get_debt(&self) -> BigUint {
        self.debt().get()
    }

    #[endpoint]
    fn draw_from_reserves(&self) -> SCResult<()> {
        self.draw_from_gold()?;
        self.draw_from_mortgages()?;
        self.draw_from_dollars()?;
        self.draw_from_silver()?;
        self.draw_from_copper()?;
        self.draw_from_platinum()?;
        Ok(())
    }

    fn draw_from_gold(&self) -> SCResult<()> {
        let gold_amount = self.debt().get();
        self.call_contract_method(
            self.gold_contract().get(),
            "execute_trade",
            &gold_amount,
        )?;
        Ok(())
    }

    fn draw_from_mortgages(&self) -> SCResult<()> {
        let mortgage_amount = self.debt().get();
        self.call_contract_method(
            self.mortgage_contract().get(),
            "swap",
            &mortgage_amount,
        )?;
        Ok(())
    }

    fn draw_from_dollars(&self) -> SCResult<()> {
        let usdt_balance = self.query_contract_balance(self.usdt_contract().get())?;
        let usdc_balance = self.query_contract_balance(self.usdc_contract().get())?;
        let total_dollars = &usdt_balance + &usdc_balance;

        if total_dollars >= self.debt().get() {
            self.call_contract_method(
                self.usdt_contract().get(),
                "transfer_from",
                &(self.blockchain().get_caller(), self.owner().get(), self.debt().get()),
            )?;
        } else {
            sc_error!("Insufficient dollars");
        }
        Ok(())
    }

    fn draw_from_silver(&self) -> SCResult<()> {
        let silver_amount = self.debt().get();
        self.call_contract_method(
            self.silver_contract().get(),
            "transfer",
            &(self.owner().get(), silver_amount),
        )?;
        Ok(())
    }

    fn draw_from_copper(&self) -> SCResult<()> {
        let copper_amount = self.debt().get();
        self.call_contract_method(
            self.copper_contract().get(),
            "transfer",
            &(self.owner().get(), copper_amount),
        )?;
        Ok(())
    }

    fn draw_from_platinum(&self) -> SCResult<()> {
        let platinum_amount = self.debt().get();
        self.call_contract_method(
            self.platinum_contract().get(),
            "transfer",
            &(self.owner().get(), platinum_amount),
        )?;
        Ok(())
    }

    fn ensure_owner(&self) -> SCResult<()> {
        if self.blockchain().get_caller() != self.owner().get() {
            sc_error!("Not the owner");
        }
        Ok(())
    }

    fn ensure_federal_reserve(&self) -> SCResult<()> {
        if self.blockchain().get_caller() != self.federal_reserve().get() {
            sc_error!("Not the US Federal Reserve");
        }
        Ok(())
    }

    fn call_contract_method<T: TopEncode>(
        &self,
        contract: ManagedAddress,
        method_name: &str,
        args: &T,
    ) -> SCResult<()> {
        let _ = self.send()
            .direct_egld()
            .to(&contract)
            .call(method_name)
            .with_args(args)
            .execute()?;
        Ok(())
    }

    fn query_contract_balance(&self, contract: ManagedAddress) -> SCResult<BigUint> {
        self.blockchain()
            .query()
            .contract(contract)
            .method_name("balance_of")
            .execute()?
            .into_biguint()
            .ok_or(sc_error!("Failed to query balance"))
    }
}
