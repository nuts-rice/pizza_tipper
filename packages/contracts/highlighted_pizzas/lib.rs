#![cfg_attr(not(feature = "std"), no_std, no_main)]

pub use highlighted_pizzas::{HighlightedPizzasError, HighlightedPizzasRef, DELETE_PIZZA_SELECTOR, GET_BY_AUTHOR_SELECTOR, HIGHLIGHTED_PIZZA_SELECTOR, HIGHLIGHTED_CONTENT_SELECTOR, HIGHLIGHT_PIZZA_SELECTOR, HIGHLIGHT_CONTENT_SELECTOR};

#[ink::contract]
mod highlighted_pizzas {
    use ink::{codegen::EmitEvent, prelude::vec::Vec, storage::Mapping,  };
    pub const HIGHLIGHTED_PIZZA_SELECTOR: [u8; 4] = [0, 0, 0, 6];
    pub const HIGHLIGHT_PIZZA_SELECTOR: [u8; 4] = [0, 0, 0, 7];
    pub const HIGHLIGHTED_CONTENT_SELECTOR: [u8; 4] = [0, 0, 0, 8];
    pub const DELETE_PIZZA_SELECTOR: [u8; 4] = [0, 0, 0, 4];
    pub const DELETE_CONTENT_SELECTOR : [u8; 4] = [0, 0, 0, 3];
    pub const HIGHLIGHT_CONTENT_SELECTOR: [u8; 4] = [0, 0, 0, 9];
    pub const GET_BY_AUTHOR_SELECTOR: [u8; 4] = [0, 0, 0, 5];
    type Event = <HighlightedPizzas as ink::reflect::ContractEventBase>::Type;

     #[ink(event)]
    pub struct ContentHighlighted {
        author: AccountId,
        id: u32,
    }
     #[ink(event)]
    pub struct HighlightRemoved {
        author: AccountId,
    }

    #[ink(event)]
    pub struct PizzaHighlighted {
        from: AccountId,
        to: AccountId,
        id: u32,
        pizzas: u32,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature="std", derive(scale_info::TypeInfo))]
    pub enum HighlightedPizzasError {
        AlreadyHighlighted,
        HighlightNotFound,
        AccessDenied,
    }

    #[ink(storage)]
    pub struct HighlightedPizzas {
        created_by: AccountId,
        highlighted_pizzas: Mapping<AccountId, u32>,
        highlighted_content: Mapping<AccountId, u32>,
        highlighted_ids: Vec<AccountId>,
        pizzas: Mapping<AccountId, u32>,
    }


    impl HighlightedPizzas {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new() -> Self {
            let caller = Self::env().caller(); 
            Self { created_by: caller, highlighted_pizzas: Mapping::default(), highlighted_content: Mapping::default(), highlighted_ids: Vec::new(), pizzas: Mapping::default() }
        }

        #[ink(message, payable, selector = 7)]
        pub fn add(&mut self, from: AccountId, to: AccountId, id: u32, pizzas: u32) -> Result<(), HighlightedPizzasError>{
            if Self::env().caller() != self.created_by {
                return Err(HighlightedPizzasError::AccessDenied);
            }
            if self.highlighted_pizzas.contains(from) {
                return Result::Err(HighlightedPizzasError::AlreadyHighlighted);
            } else {
                self.highlighted_pizzas.insert(from, &id);
                self.highlighted_ids.push(from);
                Self::emit_event(Self::env(), Event::PizzaHighlighted(PizzaHighlighted {from, to, id, pizzas}),
                );
                Ok(())
            }


        }
        #[ink(message, payable, selector = 9)]
        pub fn add_content(&mut self, author: AccountId, id: u32) -> Result<(), HighlightedPizzasError> {
            if Self::env().caller() != self.created_by {
                return Err(HighlightedPizzasError::AccessDenied);

            } if self.highlighted_content.contains(author) {
                return Result::Err(HighlightedPizzasError::AlreadyHighlighted);
            } else {
                self.highlighted_content.insert(author, &id);
                self.highlighted_ids.push(author);
                Self::emit_event(Self::env(), Event::ContentHighlighted(ContentHighlighted { author, id}),
                );
                Ok(())

            }
        }



        fn emit_event<EE>(emmiter: EE, event: Event)
            where
                EE: EmitEvent<HighlightedPizzas>,
            {
                emmiter.emit_event(event);
            }


        #[ink(message, selector = 6)]
        pub fn get_highlighted_pizzas(&self, from: AccountId) -> Option<u32> {
            self.highlighted_pizzas.get(from)
        }
        #[ink(message, selector = 8)]
        pub fn get_content_by_author(&self, author: AccountId) -> Option<u32> {
            self.highlighted_content.get(author)
        }
        #[ink(message, selector = 4)]
        pub fn delete_tip_by_author(&mut self, from: AccountId) -> Result<(), HighlightedPizzasError> {
            if Self::env().caller() != self.created_by {
                return Err(HighlightedPizzasError::AccessDenied)
            }
            if !self.highlighted_pizzas.contains(from) {
                return Err(HighlightedPizzasError::HighlightNotFound);
             } else {
                 self.highlighted_pizzas.remove(from);
                 self.highlighted_ids.retain(|tip_from| tip_from != &from);
                 Self::emit_event(Self::env(),
                 Event::HighlightRemoved(HighlightRemoved {author: from}),
                 );
                 Ok(())
             }
        }
        #[ink(message, selector = 3)]
        pub fn delete_content_by_author(&mut self, author: AccountId) -> Result<(), HighlightedPizzasError> {
            if Self::env().caller() != self.created_by {
                return Err(HighlightedPizzasError::AccessDenied)
            }
            if !self.highlighted_content.contains(author) {
                return Err(HighlightedPizzasError::HighlightNotFound);
            } else {
                 self.highlighted_content.remove(author);
                 self.highlighted_ids.retain(|content_from| content_from != &author);
                 Self::emit_event(Self::env(),
                 Event::HighlightRemoved(HighlightRemoved {author}),
                 );
                 Ok(())

            }
        }



        // #[ink(message)]
        // pub fn created_by(&self) -> AccountId {
        //     self.created_by
        // }


    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

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
            let constructor = HighlightedPizzasRef::default();

            // When
            let contract_account_id = client
                .instantiate("highlighted_pizzas", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            // Then
            let get = build_message::<HighlightedPizzasRef>(contract_account_id.clone())
                .call(|highlighted_pizzas| highlighted_pizzas.get());
            let get_result = client.call_dry_run(&ink_e2e::alice(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), false));

            Ok(())
        }

        /// We test that we can read and write a value from the on-chain contract contract.
        #[ink_e2e::test]
        async fn it_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let constructor = HighlightedPizzasRef::new(false);
            let contract_account_id = client
                .instantiate("highlighted_pizzas", &ink_e2e::bob(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let get = build_message::<HighlightedPizzasRef>(contract_account_id.clone())
                .call(|highlighted_pizzas| highlighted_pizzas.get());
            let get_result = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), false));

            // When
            let flip = build_message::<HighlightedPizzasRef>(contract_account_id.clone())
                .call(|highlighted_pizzas| highlighted_pizzas.flip());
            let _flip_result = client
                .call(&ink_e2e::bob(), flip, 0, None)
                .await
                .expect("flip failed");

            // Then
            let get = build_message::<HighlightedPizzasRef>(contract_account_id.clone())
                .call(|highlighted_pizzas| highlighted_pizzas.get());
            let get_result = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), true));

            Ok(())
        }
    }
}
