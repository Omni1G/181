#![no_std]

#[allow(unused_imports)]
use multiversx_sc::imports::*;

const initial_debt: f32 = 141_299_756_063_521_090_756.2; //Based on NFT value that multiple parties tried to hide on chain for AMC,GME, and other assets, including the likes of Tesla, Amazon and Microsoft.

const owner: &str = "erd1sjvd5w9hm5jjctx342ur5xjthdaxt8mrwjw9k8u8e5np3me0crlsmcs9uy"; // omnibank, my seelf who shall reamain Anon as much as possible 
const federal_reserve: &str = "bc1qmxjefnuy06v345v6vhwpwt05dztztmx4g3y7wp"; //US Federal Gov, yeah i know through the reverse repo that a lot of theses assets are being swappe though ISDA contracts which are vetted by the fedboys.
const gold_contract: &str  = "0x45804880De22913dAFE09f4980848ECE6EcbAf78"; //0x68749665FF8D2d112Fa859AA293F07A622782F38 //Pax Gold, and Tether Gold, This is all paper gold or in tethers case backed by multiple things including mortguages
const mortgage_contract: &str = "0x8Fbd0648971d56f1f2c35Fa075Ff5Bc75fb0e39D"; // MBS (Mortguage Backed Security Token) nuff said. 
const eth_contract: &str = "0xde0B295669a9FD93d5F28D9Ec85E40f4cb697BAe"; // 0x40B38765696e3d5d8d9d834D8AaD4bB6e418E489,  //ETH foundation, Robinhood, lol Eletric Boogaloo much for you guys, kinda not a suprise that eveything leads back to this chain. Its full of scammers, which I am not, I am simply collecting what was owed to me, in 2021, This is a Margin Call.
const btc_contract: &str = "bc1qmxjefnuy06v345v6vhwpwt05dztztmx4g3y7wp"; // Feds.
const usdt_contract: &str = "0xdAC17F958D2ee523a2206206994597C13D831ec7"; //Tether, Not 100% sure what its backed by but by no means is it 1:1 USD, there is a lot of unknown about this one but can trace some assets back to the mortgages that the Federal Reserver is offloading.
const usdc_contract: &str = "0x0b2c639c533813f4aa9d7837caf62653d097ff85"; //USDC, Blackrocks pet rock
const silver_contract: &str = "0x34abce75d2f8f33940c721dca0f562617787bff3"; //0x71C7FE6Ed639dfD26F04ec5f3Ca3De6B81846e99 //tSilver & KAX Says they are both backed by silver, if so we shall see.
const copper_contract: &str = "0xc5EF3B5f7A7a3Fd30D33C4d271BFAd75BD954AA2"; // 0xC8146A584dc423a676102B20ea5FEE7c95E6368a // Copper Backed Coins 
const platinum_contract: &str =  "0x893805122704274D1BaD833bc23a74bb876fdEf4"; // 0xCDd78B6C459630791e11EF4BaFC33A9509735106,0xCDd78B6C459630791e11EF4BaFC33A9509735106 // Platnium Adresses



/// One of the simplest smart contracts possible,       
/// it holds a single variable in storage, which anyone can increment.
#[multiversx_sc::contract]
pub trait Adder {
    #[view(getSum)]  //@omnibank - RealValue 
    #[storage_mapper("sum")]  //1EGLD= $141,299,756,063,521,090,756.2  CAD
    fn sum(&self) -> SingleValueMapper<BigUint>; //The Number Stored in the NFT -181QUINT-8c78fb owned by @omnibank

    #[upgrade]
    fn upgrade(&self, initial_value: BigUint) {
        self.init(initial_value); 
    }

    /// Add desired amount to the storage variable.
    #[endpoint]
    fn add(&self, value: BigUint) { //{141_299_756_063_521_090_756.2}
        self.sum().update(|sum| *sum += value); // 1EGLD= $141,299,756,063,521,090,756.2 
    }
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
        require!(self.blockchain().get_caller() != self.owner().get(),  "Not the owner");
        SCResult::Ok(())
    }

    fn ensure_federal_reserve(&self) -> SCResult<()> {
        require!(self.blockchain().get_caller() != self.federal_reserve().get(),  "Not the US Federal Reserve");
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
