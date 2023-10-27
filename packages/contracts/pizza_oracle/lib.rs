#![cfg_attr(not(feature = "std"), no_std, no_main)]
pub use pizza_oracle::{}
#[ink::contract]
mod pizza_oracle {
    use ink::{codegen::EmitEvent, prelude::vec::Vec, storage::Mapping};
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
    pub enum PriceStatus {

    }
    #[ink(storage)]
    pub struct PizzaOracle {

        is_init: bool,
        pizza_tipper_id: Option<AccountId>,
        price_map: Mapping<u32, PizzaPrice>,
        //stablecoin
        usd_id: Option<AccountId>,
        upgrader: Option<AccountId>,
    }

    impl is_init for PizzaOracle {
        fn is_init(&self) -> bool {
            self.is_init
        }
    }
    pub struct PizzaPrice {
        confidence: u64,
        status: PriceStatus,
        current_pizza_price: u128,

    }

    impl PizzaOracle {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self { value: init_value }
        }

        /// Constructor that initializes the `bool` value to `false`.
        ///
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        /// A message that can be called on instantiated contracts.
        /// This one flips the value of the stored `bool` from `true`
        /// to `false` and vice versa.
        #[ink(message)]
        pub fn flip(&mut self) {
            self.value = !self.value;
        }
        #[ink(message, payable, selector = 5)]
        pub fn get_price(&self, id: u32) -> Option<PizzaPrice> {
            self.price_map.get(id)
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let pizza_oracle = PizzaOracle::default();
            assert_eq!(pizza_oracle.get(), false);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut pizza_oracle = PizzaOracle::new(false);
            assert_eq!(pizza_oracle.get(), false);
            pizza_oracle.flip();
            assert_eq!(pizza_oracle.get(), true);
        }
    }


    /// This is how you'd write end-to-end (E2E) or integration tests for ink! contracts.
    ///
    /// When running these you need to make sure that you:
    /// - Compile the tests with the `e2e-tests` feature flag enabled (`--features e2e-tests`)
    /// - Are running a Substrate node which contains `pallet-contracts` in the background
    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// A helper function used for calling contract messages.
        use ink_e2e::build_message;

        /// The End-to-End test `Result` type.
        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        /// We test that we can upload and instantiate the contract using its default constructor.
        #[ink_e2e::test]
        async fn default_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let constructor = PizzaOracleRef::default();

            // When
            let contract_account_id = client
                .instantiate("pizza_oracle", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            // Then
            let get = build_message::<PizzaOracleRef>(contract_account_id.clone())
                .call(|pizza_oracle| pizza_oracle.get());
            let get_result = client.call_dry_run(&ink_e2e::alice(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), false));

            Ok(())
        }

        /// We test that we can read and write a value from the on-chain contract contract.
        #[ink_e2e::test]
        async fn it_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let constructor = PizzaOracleRef::new(false);
            let contract_account_id = client
                .instantiate("pizza_oracle", &ink_e2e::bob(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let get = build_message::<PizzaOracleRef>(contract_account_id.clone())
                .call(|pizza_oracle| pizza_oracle.get());
            let get_result = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), false));

            // When
            let flip = build_message::<PizzaOracleRef>(contract_account_id.clone())
                .call(|pizza_oracle| pizza_oracle.flip());
            let _flip_result = client
                .call(&ink_e2e::bob(), flip, 0, None)
                .await
                .expect("flip failed");

            // Then
            let get = build_message::<PizzaOracleRef>(contract_account_id.clone())
                .call(|pizza_oracle| pizza_oracle.get());
            let get_result = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), true));

            Ok(())
        }
    }
}
