#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod tipper {
    use ink::LangError;
    // use ink::primitives::AccountId;
    use ink::{
        codegen::EmitEvent,
        prelude::{format, string::String, vec::Vec},
        storage::Mapping,
        ToAccountId,
    };
    use ink::{
        env::{
            call::{build_call, ExecutionInput, FromAccountId, Selector},
            DefaultEnvironment, Error as InkEnvError,
        },
        reflect::ContractEventBase,
    };
    // use tracing::Event;

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct Tip {
        from: AccountId,
        pizzas: u32,
        message: String,
    }

    #[ink(storage)]
    #[derive(Default)]
    pub struct Tipper {
        id_counter: u32,
        id_map: Mapping<AccountId, u32>,
        tip_map: Mapping<u32, Tip>,
        total_supply: Balance,
        balances: Mapping<AccountId, Balance>,
        elements_count: u32,
        //oracle -> pizza cost goodness
        price_per_pizza: u128,
        pizza_oracle: Option<AccountId>,
        //highlighted tips and creator posts :
        highlighted_pizzas: Option<AccountId>,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        InsufficientBalance,
        PizzaCostTooLow(u128),
        AlreadyTipped,
        TipError,
    }
    type Event = <Tipper as ContractEventBase>::Type;

    impl Tipper {
        #[ink(constructor)]
        pub fn new(
            version: u8,
            pizza_oracle_hash: Hash,
            highlighted_pizzas_hash: Hash,
            total_supply: Balance,
            price_per_pizza: u128,
        ) -> Self {
            let mut balances = Mapping::default();
            let caller = Self::env().caller();
            balances.insert(caller, &total_supply);
            //TODO: cross contract refs
            //let pizza_oracle_ref = PizzaOracleRef::new();
            //let highlighted_pizzas_ref = HighlightedPizzaRef::new();
            Self {
                id_counter: 0,
                elements_count: 0,
                price_per_pizza: price_per_pizza,
                tip_map: Mapping::default(),
                id_map: Mapping::default(),
                pizza_oracle: None,
                highlighted_pizzas: None,
                total_supply,
                balances,
            }
        }

        #[ink(constructor)]
        pub fn free() -> Self {
            Self {
                id_counter: 0,
                price_per_pizza: 0,
                total_supply: 0,
                balances: Mapping::default(),
                elements_count: 0,
                tip_map: Mapping::default(),
                id_map: Mapping::default(),
                pizza_oracle: None,
                highlighted_pizzas: None,
            }
        }

        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            self.total_supply
        }

        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> Balance {
            self.balances.get(owner).unwrap_or_default()
        }

        pub fn lookup_pzzas(&self) -> Result<(), Error> {
            Ok(())
        }

        pub fn transfer(&mut self, to: AccountId, value: Balance) -> Result<(), Error> {
            let from = self.env().caller();
            let from_balance = self.balance_of(from);
            if from_balance < value {
                return Err(Error::InsufficientBalance);
            }

            self.balances.insert(from, &(from_balance - value));
            let to_balance = self.balance_of(to);
            self.balances.insert(to, &(to_balance + value));
            Ok(())
        }

        #[ink(message, payable)]
        pub fn tip(
            &mut self,
            tip_message: String,
            to: AccountId,
            n_pizzas: u32,
        ) -> Result<(), Error> {
            let from = Self::env().caller();
            ink::env::debug_println!(
                "{:?} wants to tip {:?} with {:?} pizzas with the message '{:?}' ",
                from,
                to,
                n_pizzas,
                tip_message
            );
            if self.id_map.contains(&from) {
                return Err(Error::AlreadyTipped);
            }
            let transfered_amount = self.env().transferred_value();
            let pizza_cost = self
                .price_per_pizza
                .checked_mul(n_pizzas.into())
                .unwrap_or(u128::MAX);

            if transfered_amount < pizza_cost {
                return Err(Error::PizzaCostTooLow(pizza_cost));
            }
            let event = self._tip(tip_message, from, to, n_pizzas);
            Self::emit_event(Self::env(), Event::PizzaSent(event));
            Ok(())
        }

        fn _tip(
            &mut self,
            tip_message: String,
            from: AccountId,
            to: AccountId,
            n_pizzas: u32,
        ) -> PizzaSent {
            let tip = Tip {
                from,
                pizzas: n_pizzas,
                message: tip_message,
            };
            let tip_id = self.insert_tip(&from, tip);

            PizzaSent {
                from: from,
                to: to,
                id: tip_id,
            }
        }

        fn insert_tip(&mut self, caller: &AccountId, tip: Tip) -> u32 {
            unimplemented!()
        }

        fn emit_event<EE>(emitter: EE, event: Event)
        where
            EE: EmitEvent<Tipper>,
        {
            emitter.emit_event(event);
        }
    }

    #[ink(event)]
    pub struct PizzaSent {
        #[ink(topic)]
        from: AccountId,
        to: AccountId,
        id: u32,
    }

    #[ink(event)]
    pub struct ContentPosted {
        #[ink(topic)]
        author: AccountId,
        id: u32,
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink::{
            env::{
                test::{
                    default_accounts, get_account_balance, recorded_events, DefaultAccounts,
                    EmittedEvent,
                },
                DefaultEnvironment,
            },
            primitives::AccountId,
        };
        use scale::Decode;

        fn get_test_accts() -> DefaultAccounts<ink::env::DefaultEnvironment> {
            default_accounts::<ink::env::DefaultEnvironment>()
        }
        fn get_balance(acct_id: AccountId) -> Balance {
            get_account_balance::<ink::env::DefaultEnvironment>(acct_id)
                .expect("can't get account balance")
        }
        fn set_from(from: AccountId) {
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(from);
        }

        #[ink::test]
        fn total_supply_works() {
            let tipper = Tipper::new(100, 25);
            assert_eq!(tipper.total_supply(), 100);
        }

        fn event_on_tip() {
            let mut instance = Tipper::free();
        }

        #[ink::test]
        fn tipper_works() {
            let mut tipper = Tipper::new(100, 25);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
        }
        #[ink::test]
        fn pizza_oracle_works() {
            unimplemented!()
        }
    }
}
// impl fmt::Display for tipper::Error {
//     fn fmt(&self, f: &mut Formatter) -> fmt::Result {
//         write!(f, "")
//     }
// }
// Sets `message` to the given value.
// #[ink(message)]
// pub fn set_message(&mut self, new_value: String) {
//     self.message = new_value.clone();

//     let from = self.env().caller();
//     self.env().emit_event(Greeted {
//         from: Some(from),
//         message: new_value,
//     });
// }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

// #[ink::test]
// fn new_works() {
//     let message = "Hello ink! v4".to_string();
//     let greeter = Greeter::new(message.clone());
//     assert_eq!(greeter.greet(), message);
// }

// #[ink::test]
// fn default_new_works() {
//     let greeter = Greeter::default();
//     let default_message = String::from("Hello ink!");
//     assert_eq!(greeter.greet(), default_message);
// }

// #[ink::test]
// fn set_message_works() {
//     let message_1 = String::from("gm ink!");
//     let mut greeter = Greeter::new(message_1.clone());
//     assert_eq!(greeter.greet(), message_1);
//     let message_2 = String::from("gn");
//     greeter.set_message(message_2.clone());
//     assert_eq!(greeter.greet(), message_2);
// }
// }
// }
