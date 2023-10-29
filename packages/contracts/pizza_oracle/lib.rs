#![cfg_attr(not(feature = "std"), no_std, no_main)]
pub use pizza_oracle::{OracleQueried, OracleError, GET_PIZZA_PRICE_SELECTOR, PizzaOracle, PizzaPrice, PizzaOracleRef};

#[ink::contract]
mod pizza_oracle {

    use ink::{codegen::EmitEvent, prelude::vec::Vec, storage::Mapping, env::AccountIdGuard};
    pub const GET_PIZZA_PRICE_SELECTOR: [u8; 4] = [0,0,0,5]; 

    type Event = <PizzaOracle as ink::reflect::ContractEventBase>::Type;

    #[ink(event)]
    pub struct OracleQueried {
        from: AccountId,
        query_id: u32,
    } 


    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum OracleError {
        InvalidQuery,
        PriceNotFound,
        AccessDenied,
    }
    #[ink(storage)]
    pub struct PizzaOracle {
        caller: Option<AccountId>,
        is_init: bool,
        pizza_tipper_id: Option<AccountId>,
        price_map: Mapping<u32, PizzaPrice>,
        //stablecoin
        usd_id: Option<AccountId>,
        upgrader: Option<AccountId>,
    }
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]

       pub struct PizzaPrice {
        confidence: u64,
        current_pizza_price: u128,

    }

    impl PizzaOracle {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new() -> Self {
            Self { is_init: true, price_map: Mapping::default(), upgrader: None, pizza_tipper_id: None, usd_id: None, caller: None }
                    }

        /// Constructor that initializes the `bool` value to `false`.
        ///
        /// Constructors can delegate to other constructors.
        #[ink(message, payable, selector = 5)]
        pub fn get_price(&self, id: u32) -> Option<PizzaPrice> {
            self.price_map.get(id)
        }

    }

}
