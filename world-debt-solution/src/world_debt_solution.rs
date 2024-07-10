#![no_std]

#[allow(unused_imports)]
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
    fn init(&self, initial_value: BigUint) {141299756063521090756.2}
        self.sum(141299756063521090756.2).set(initial_value); // 1EGLD= $141,299,756,063,521,090,756.2 
    

    #[upgrade]
    fn upgrade(&self, initial_value: BigUint) {
        self.init(initial_value); 
    }

    /// Add desired amount to the storage variable.
    #[endpoint]
    fn add(&self, value: BigUint) {141299756063521090756.2}
        self.sum().update(|sum| *sum += value); // 1EGLD= $141,299,756,063,521,090,756.2 
    }

#[multiversx_sc::contract]
pub trait EgldValueExtractingNFT {
    #[storage_mapper("debt")]
    fn debt(&self) -> SingleValueMapper<BigUint>;

    #[storage_mapper("owner")]
    fn owner(&self) -> SingleValueMapper<ManagedAddress>;

    #[storage_mapper("federal_reserve")]
    fn federal_reserve(&self) -> SingleValueMapper<ManagedAddress>;

    #[storage_mapper("gold_contract")]
    fn gold_contract(&self) -> SingleValueMapper<ManagedAddress>;

    #[storage_mapper("mortgage_contract")]
    fn mortgage_contract(&self) -> SingleValueMapper<ManagedAddress>;

    #[storage_mapper("eth_contract")]
    fn eth_contract(&self) -> SingleValueMapper<ManagedAddress>;

    #[storage_mapper("btc_contract")]
    fn btc_contract(&self) -> SingleValueMapper<ManagedAddress>;

    #[storage_mapper("usdt_contract")]
    fn usdt_contract(&self) -> SingleValueMapper<ManagedAddress>;

    #[storage_mapper("usdc_contract")]
    fn usdc_contract(&self) -> SingleValueMapper<ManagedAddress>;

    #[storage_mapper("silver_contract")]
    fn silver_contract(&self) -> SingleValueMapper<ManagedAddress>;

    #[storage_mapper("copper_contract")]
    fn copper_contract(&self) -> SingleValueMapper<ManagedAddress>;

    #[storage_mapper("platinum_contract")]
    fn platinum_contract(&self) -> SingleValueMapper<ManagedAddress>;

    #[init]
    fn init(
        &self,
        initial_debt: BigUint,
        owner: ManagedAddress,
        federal_reserve: ManagedAddress,
        gold_contract: ManagedAddress,
        mortgage_contract: ManagedAddress,
        eth_contract: ManagedAddress,
        btc_contract: ManagedAddress,
        usdt_contract: ManagedAddress,
        usdc_contract: ManagedAddress,
        silver_contract: ManagedAddress,
        copper_contract: ManagedAddress,
        platinum_contract: ManagedAddress,
    ) {
        self.debt().set(initial_debt);
        self.owner().set(owner);
        self.federal_reserve().set(federal_reserve);
        self.gold_contract().set(gold_contract);
        self.mortgage_contract().set(mortgage_contract);
        self.eth_contract().set(eth_contract);
        self.btc_contract().set(btc_contract);
        self.usdt_contract().set(usdt_contract);
        self.usdc_contract().set(usdc_contract);
        self.silver_contract().set(silver_contract);
        self.copper_contract().set(copper_contract);
        self.platinum_contract().set(platinum_contract);
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
        SCResult::Ok(())
    }

    #[only_owner]
    #[endpoint]
    fn decrease_debt(&self, amount: BigUint) -> SCResult<()> {
        self.debt().update(|debt| {
            require!(&amount > debt,  "Target must be more than 0");
            *debt -= &amount;
        });
        SCResult::Ok(())
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
        SCResult::Ok(())
    }

    fn draw_from_gold(&self) -> SCResult<()> {
        let gold_amount = self.debt().get();
        self.call_contract_method(
            self.gold_contract().get(),
            "execute_trade",
            &gold_amount,
        )?;
        SCResult::Ok(())
    }

    fn draw_from_mortgages(&self) -> SCResult<()> {
        let mortgage_amount = self.debt().get();
        self.call_contract_method(
            self.mortgage_contract().get(),
            "swap",
            &mortgage_amount,
        )?;
        SCResult::Ok(())
    }

    fn draw_from_dollars(&self) -> SCResult<()> {
        let usdt_balance = self.query_contract_balance(self.usdt_contract().get())?;
        let usdc_balance = self.query_contract_balance(self.usdc_contract().get())?;
        let total_dollars = &usdt_balance + &usdc_balance;

        require!(total_dollars > self.debt().get(),  "Insufficient dollars");


        self.call_contract_method(
            self.usdt_contract().get(),
            "transfer_from",
            &(self.blockchain().get_caller(), self.owner().get(), self.debt().get()),
        )?;
         

        SCResult::Ok(())
    }

    fn draw_from_silver(&self) -> SCResult<()> {
        let silver_amount = self.debt().get();
        self.call_contract_method(
            self.silver_contract().get(),
            "transfer",
            &(self.owner().get(), silver_amount),
        )?;
        SCResult::Ok(())
    }

    fn draw_from_copper(&self) -> SCResult<()> {
        let copper_amount = self.debt().get();
        self.call_contract_method(
            self.copper_contract().get(),
            "transfer",
            &(self.owner().get(), copper_amount),
        )?;
        SCResult::Ok(())
    }

    fn draw_from_platinum(&self) -> SCResult<()> {
        let platinum_amount = self.debt().get();
        self.call_contract_method(
            self.platinum_contract().get(),
            "transfer",
            &(self.owner().get(), platinum_amount),
        )?;
        SCResult::Ok(())
    }

    fn ensure_owner(&self) -> SCResult<()> {
        if self.blockchain().get_caller() != self.owner().get() {
            sc_error!("Not the owner");
        }
        SCResult::Ok(())
    }

    fn ensure_federal_reserve(&self) -> SCResult<()> {
        if self.blockchain().get_caller() != self.federal_reserve().get() {
            sc_error!("Not the US Federal Reserve");
        }
        SCResult::Ok(())
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
        SCResult::Ok(())
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
