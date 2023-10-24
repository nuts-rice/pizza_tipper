#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod tipper {
    
    use ink::{
        codegen::EmitEvent,
        prelude::{string::String, vec::Vec},
        storage::Mapping, env::{call::{build_call, ExecutionInput, FromAccountId, Selector}, DefaultEnvironment, }, LangError,   
    };
    use ink::{
        reflect::ContractEventBase,
    };
    use highlighted_pizzas::{HighlightedPizzasRef, HighlightedPizzasError, HIGHLIGHT_PIZZA_SELECTOR};
    // use tracing::Event;

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct Tip {
        from: AccountId,
        to: AccountId,
        pizzas: u32,
        message: String,
        //Payment channel: amount withdrawn by 'to' 
        
    }

    #[ink(storage)]
    #[derive(Default)]
    pub struct Tipper {
        id_counter: u32,
        id_map: Mapping<AccountId, u32>,
        tip_map: Mapping<u32, Tip>,
        balances: Mapping<AccountId, Balance>,
        elements_count: u32,
        pizza_tippers: Vec<AccountId>,
        //oracle -> pizza cost goodness
        price_per_pizza: u128,
        pizza_oracle: Option<AccountId>,
        //highlighted tips and creator posts :
        highlighted_pizzas: Option<AccountId>,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum TipperError {
        InsufficientBalance,
        PizzaCostTooLow(u128),
        AlreadyTipped,
        TipError,
        HighlightError(HighlightedPizzasError),
        
    }

    type Event = <Tipper as ContractEventBase>::Type;

    impl Tipper {
        #[ink(constructor)]
        pub fn new(
            version: u8,
            // _pizza_oracle_hash: Hash,
            highlighted_pizzas_hash: Hash,
            price_per_pizza: u128,
        ) -> Self {
            let mut balances = Mapping::default();
            let caller = Self::env().caller();
            //TODO: cross contract refs
            //let pizza_oracle_ref = PizzaOracleRef::new();
            let highlighted_pizzas_ref = HighlightedPizzasRef::new();
            Self {
                id_counter: 0,
                elements_count: 0,
                price_per_pizza,
                tip_map: Mapping::default(),
                id_map: Mapping::default(),
                pizza_oracle: None,
                highlighted_pizzas: None,
                pizza_tippers: Vec::new(),
                balances,
            }
        }

        #[ink(constructor)]
        pub fn free() -> Self {
            Self {
                id_counter: 0,
                price_per_pizza: 0,
                balances: Mapping::default(),
                elements_count: 0,
                tip_map: Mapping::default(),
                id_map: Mapping::default(),
                pizza_tippers: Vec::new(),
                pizza_oracle: None,
                highlighted_pizzas: None,
            }
        }


        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> Balance {
            self.balances.get(owner).unwrap_or_default()
        }

        #[ink(message)]
        pub fn get_pizza_tippers(&self) -> Vec<AccountId> {
            self.pizza_tippers.clone()
        }

        pub fn lookup_pizzas(&self) -> Result<(), TipperError> {
            Ok(())
        }


        #[ink(message, payable)]
        pub fn tip(
            &mut self,
            tip_message: String,
            to: AccountId,
            n_pizzas: u32,
        ) -> Result<(), TipperError> {
            let from = Self::env().caller();
            ink::env::debug_println!(
                "{:?} wants to tip {:?} with {:?} pizzas with the message '{:?}' ",
                from,
                to,
                n_pizzas,
                tip_message
            );
            // if self.id_map.contains(from) {
            //     return Err(Error::AlreadyTipped);
            // }
            let transfered_amount = self.env().transferred_value();
            let pizza_cost = self
                .price_per_pizza
                .checked_mul(n_pizzas.into())
                .unwrap_or(u128::MAX);

            if transfered_amount < pizza_cost {
                return Err(TipperError::PizzaCostTooLow(pizza_cost));
            }
            let event = self._tip(tip_message, from, to, n_pizzas );
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
                to,
                pizzas: n_pizzas,
                message: tip_message,
            };
            let tip_id = self.insert_tip(&from, tip);

            PizzaSent {
                from,
                to,
                pizzas: n_pizzas,
                id: tip_id,
            }
        }

        fn insert_tip(&mut self, from: &AccountId, tip: Tip) -> u32 {
            let pizza_id = self.id_counter;
            self.id_map.insert(from, &pizza_id);
            self.tip_map.insert(pizza_id, &tip);
            self.id_counter = pizza_id + 1;
            self.pizza_tippers.push(*from);
            pizza_id
        }
        fn reimburse(&self, to: AccountId, amount: u128) {
            if Self::env().transfer(to, amount).is_err() {
                panic!("failed to reimburse caller")
            }
        }
        fn get_by_account(&self, from: &AccountId) -> Option<Tip> {
            if let Some(tip_id) = self.id_map.get(from) {
                let tip = self.tip_map.get(tip_id).unwrap_or_else(|| {
                    panic!("expected tip to exist for caller")
                });
                Some(tip)
            } else {
                None

            }
        }
        fn get_by_id(&self, id: u32) -> Option<Tip> {
            self.tip_map.get(id)
        }
        #[ink(message)]
        pub fn terminate_contract(&mut self) {
            if self.elements_count == 0 {
                self.env().terminate_contract(self.env().caller());
            }
        }
        fn highlight_tip(&self, from: AccountId, id: u32, cost: u128) -> Result<(), TipperError > {
            if let Some(highlight_pizzas) = self.highlighted_pizzas {
                let call_result : Result<Result<(), HighlightedPizzasError>, ink::LangError>  = build_call::<DefaultEnvironment>()
                    .call(highlight_pizzas)
                    .exec_input(ExecutionInput::new(Selector::new(HIGHLIGHT_PIZZA_SELECTOR
                                                                  ))
                                .push_arg(from)
                                .push_arg(id),
                                )
                    .transferred_value(cost)
                    .returns::<Result<Result<(), HighlightedPizzasError>, LangError>> ()
                    .invoke();
                match call_result {
                    Err(lang_error) => {
                        panic!("Unexpected ink::LangError: {:?}", lang_error)
                    }
                    Ok(Err(contract_call_error)) =>  {
                        return Err(TipperError::HighlightError(contract_call_error,))
                    }
                    Ok(Ok(_unit)) => return Ok(()),

                }
            }
            Ok(())

        }
        
        fn delete_tip_highlight(&self, from: AccountId) -> Result<(), TipperError> {
            
            if let Some(highlight_tip) = self.highlighted_pizzas {
                <HighlightedPizzasRef as FromAccountId<super::tipper::Environment, >>::from_account_id(highlight_tip).delete_tip_by_author(from);
            }
            Ok(())
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
        pizzas: u32,
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

        fn tip_from_alice(instance: &mut Tipper) -> Tip {
            let accts = get_test_accts();
            let alice = accts.alice;
            let bob = accts.bob;
            let msg : ink::prelude::string::String = "dummy".into();
            let expected_tip = Tip {
                from: alice,
                to: bob,
                pizzas: 1,
                message: msg.clone()
            };
            set_from(alice);
            assert!(instance.tip(msg, bob, 1).is_ok(),
            "tipping expected");
            expected_tip
        }

        const PRICE_PER_PIZZA: u128 = 7;

        #[ink::test]
        fn constructor_works() {
            let tipper = Tipper::free();
            assert_eq!(tipper.price_per_pizza, 0);
        }

        #[ink::test]
        fn pizza_msg_test() {
            let accts = get_test_accts();
            let alice = accts.alice;
            let bob = accts.bob;
            let mut tipper = Tipper::free();
            let msg: ink::prelude::string::String = "dummy".into();
            set_from(alice);
            let executed_tip = tipper.tip(msg.clone(), accts.bob, 1);
            let expected_tip = Tip {                
                from: alice,
                to: bob,
                pizzas: 1,
                message: "dummy".into(),
            };
            assert_eq!(tipper.get_by_id(0).unwrap().message, expected_tip.message);
        }

        #[ink::test]
        fn event_on_tip() {
            let mut instance = Tipper::free();
            let tip = tip_from_alice(&mut instance) ;
            let recorded_events = recorded_events().collect::<Vec<_>>();
            assert_expected_tip_event(&recorded_events[0], tip.to, tip.from, 0, tip.pizzas);
        }

        #[ink::test]
        fn tipper_works() {
            let mut tipper = Tipper::free();
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            unimplemented!()
        }
        fn assert_expected_tip_event(event: &EmittedEvent, expected_to: AccountId, expected_from: AccountId, expected_id: u32, expected_pizzas: u32) {
            let decoded_event = <Event as Decode>::decode(&mut &event.data[..])
                .expect("invalid contract eventy data buffer");
            if let Event::PizzaSent(PizzaSent {                
                from,
                to,
                id,
                pizzas,
            }) = decoded_event {
                assert_eq!(from, expected_from);
                assert_eq!(to, expected_to);
                assert_eq!(id, expected_id);
                assert_eq!(pizzas, expected_pizzas);
            } else {
                panic!("expected PizzaSent")
            };
        }

        #[ink::test]
        fn tip_event_test() {
            let mut tipper = Tipper::free();
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
