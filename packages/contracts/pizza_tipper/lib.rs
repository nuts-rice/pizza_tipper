#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod tipper {
    use highlighted_pizzas::{
        HighlightedPizzasError, HighlightedPizzasRef, HIGHLIGHT_PIZZA_SELECTOR,
    };
     
   
    use ink::reflect::ContractEventBase;
    use ink::{
        codegen::EmitEvent,
        env::{
            call::{build_call, ExecutionInput, FromAccountId, Selector},
            DefaultEnvironment,
        },
        prelude::{string::String, vec::Vec},
        storage::Mapping,
        LangError,
    };
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
    pub type ResolveResult<AccountId> = Result<AccountId, TipperError>;

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
        InsufficientAmount(u128),
        AlreadyTipped,
        TipError,
        HighlightError(HighlightedPizzasError),
        //Error for azero resolver
        DoesntExist,
    }

    type Event = <Tipper as ContractEventBase>::Type;
    impl Tipper {
        #[ink(constructor)]
        pub fn new(
            _version: u8,
            // _pizza_oracle_hash: Hash,
            _highlighted_pizzas_hash: Hash,
            price_per_pizza: u128,
        ) -> Self {
            let _caller = Self::env().caller();
            //TODO: cross contract refs
            //let pizza_oracle_ref = PizzaOracleRef::new();
            let _highlighted_pizzas_ref = HighlightedPizzasRef::new();
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
                return Err(TipperError::InsufficientAmount(pizza_cost));
            }
            self.env()
                .transfer(to, transfered_amount)
                .map_err(|_| TipperError::TipError)?;
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
                let tip = self
                    .tip_map
                    .get(tip_id)
                    .unwrap_or_else(|| panic!("expected tip to exist for caller"));
                Some(tip)
            } else {
                None
            }
        }
        fn get_by_id(&self, id: u32) -> Option<Tip> {
            self.tip_map.get(id)
        }
        fn get_pizza_cost(&self) -> u128 {
            self.price_per_pizza
        }
        #[ink(message)]
        pub fn terminate_contract(&mut self) {
            if self.elements_count == 0 {
                self.env().terminate_contract(self.env().caller());
            }
        }
        fn highlight_tip(&self, from: AccountId, id: u32, cost: u128) -> Result<(), TipperError> {
            if let Some(highlight_pizzas) = self.highlighted_pizzas {
                let call_result: Result<Result<(), HighlightedPizzasError>, ink::LangError> =
                    build_call::<DefaultEnvironment>()
                        .call(highlight_pizzas)
                        .exec_input(
                            ExecutionInput::new(Selector::new(HIGHLIGHT_PIZZA_SELECTOR))
                                .push_arg(from)
                                .push_arg(id),
                        )
                        .transferred_value(cost)
                        .returns::<Result<Result<(), HighlightedPizzasError>, LangError>>()
                        .invoke();
                match call_result {
                    Err(lang_error) => {
                        panic!("Unexpected ink::LangError: {:?}", lang_error)
                    }
                    Ok(Err(contract_call_error)) => {
                        return Err(TipperError::HighlightError(contract_call_error))
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
        use ink::env::{
            test::{
                default_accounts, get_account_balance, recorded_events,
                DefaultAccounts, EmittedEvent,
            },
        };
        use ink::primitives::AccountId;
        use ink_e2e::subxt::tx::Signer;
        use ink_e2e::{build_message};
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
            let msg: ink::prelude::string::String = "dummy".into();
            let expected_tip = Tip {
                from: alice,
                to: bob,
                pizzas: 1,
                message: msg.clone(),
            };
            set_from(alice);
            assert!(instance.tip(msg, bob, 1).is_ok(), "tipping expected");
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
            let _executed_tip = tipper.tip(msg.clone(), accts.bob, 1);
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
            let tip = tip_from_alice(&mut instance);
            let recorded_events = recorded_events().collect::<Vec<_>>();
            assert_expected_tip_event(&recorded_events[0], tip.to, tip.from, 0, tip.pizzas);
        }

        #[ink::test]
        fn tipper_works() {
            let tipper = Tipper::free();
            assert_eq!(tipper.price_per_pizza, 0)
        }
        fn assert_expected_tip_event(
            event: &EmittedEvent,
            expected_to: AccountId,
            expected_from: AccountId,
            expected_id: u32,
            expected_pizzas: u32,
        ) {
            let decoded_event = <Event as Decode>::decode(&mut &event.data[..])
                .expect("invalid contract eventy data buffer");
            if let Event::PizzaSent(PizzaSent {
                from,
                to,
                id,
                pizzas,
            }) = decoded_event
            {
                assert_eq!(from, expected_from);
                assert_eq!(to, expected_to);
                assert_eq!(id, expected_id);
                assert_eq!(pizzas, expected_pizzas);
            } else {
                panic!("expected PizzaSent")
            };
        }
        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;
        #[ink::test]
        fn tip_event_test() {
            let _tipper = Tipper::free();
            let _accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
        }
        fn pizza_oracle_works() {
            unimplemented!()
        }

        fn get_bob() -> AccountId {
            let bob_acct_id: AccountId =
                AccountId::try_from(ink_e2e::bob().public_key().to_account_id().as_ref()).unwrap();
            bob_acct_id
        }
        #[ink_e2e::test]
        fn get_tippers_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let tipper = TipperRef::free();
            let contract_acc_id = client
                .instantiate("pizza_tipper", &ink_e2e::alice(), tipper, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;
            let tip = build_message::<TipperRef>(contract_acc_id)
                .call(|tipper| tipper.tip("dummy".to_string(), get_test_accts().eve, 1));
            let _tip_res = client
                .call(&ink_e2e::bob(), tip, 0, None)
                .await
                .expect("tip failed");
            let get = build_message::<TipperRef>(contract_acc_id)
                .call(|tipper| tipper.get_pizza_tippers());
            let get_res = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
            let bob_acct_id = get_bob();
            // ink::env::debug_println!("bob e2e acct id res: {:?}", bob_acct_id);
            assert!(&get_res.return_value().contains(&bob_acct_id));
            Ok(())
        }
        #[ink::test]
        fn pizza_cost_works() {
            let accts = get_test_accts();
            let dummy_hash: Hash = Hash::from([0x00; 32]);
            let mut instance = Tipper::new(1, dummy_hash, PRICE_PER_PIZZA, );
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(7);
            let before = get_balance(accts.alice);
            ink::env::debug_println!("before value: {}", before);
            let _tip = tip_from_alice(&mut instance);
            let after = get_balance(accts.alice);
            ink::env::debug_println!("after value: {}", after);
            let expected_balance = before - PRICE_PER_PIZZA;
            assert_eq!(after, expected_balance, "Balance is incorrect");
        }

        #[ink::test]
        #[should_panic(expected = "")]
        fn pizza_cost_fails() {
            let accts = get_test_accts();
            let dummy_hash: Hash = Hash::from([0x00; 32]);
            let mut instance = Tipper::new(1, dummy_hash, PRICE_PER_PIZZA, );
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(6);
            let before = get_balance(accts.alice);
            ink::env::debug_println!("before value: {}", before);
            let _tip = tip_from_alice(&mut instance);
            let after = get_balance(accts.alice);
            ink::env::debug_println!("after value: {}", after);
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
