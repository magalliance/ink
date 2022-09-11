#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
pub mod passport {
    use ink_storage::traits::SpreadAllocate;
    use ink_prelude::string::String;
    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct Passport {
        name: String,
        surname: String,
        birth: String,
        assets: ink_storage::Mapping<AccountId, Balance>,
        client: String,
        owner: AccountId,
        is_active: bool,
    }

    impl Passport {
        /// Creates a new Passport smart contract initialized with the given values
        #[ink(constructor)]
        pub fn new(name: String, surname: String, birth: String, client: String) -> Self {
            ink_lang::utils::initialize_contract(|contract: &mut Self| {
                let caller = Self::env().caller();
                contract.name = name;
                contract.surname = surname;
                contract.birth = birth;
                contract.client = client;
                //contract.map.insert(&caller, &0); ?
                contract.owner = caller;
                contract.is_active = true;
            })
        }

        /// Returns the current status of the Passport's boolean
        #[ink(message)]
        pub fn get_status(&self) -> bool {
            self.is_active
        }

        ///  Set passport's status only owner
        #[ink(message)]
        pub fn set_status(&mut self, status: bool) {
            assert_eq!(
                self.env().caller(),
                self.owner,
                "Only owner have permissions to change status"
            );
            self.is_active = status;
        }

        /// Returns the current name of the Passport's
        #[ink(message)]
        pub fn get_name(&self) -> String {
            if self.owner == self.env().caller() {
                let mut full_name = String::from(&self.surname);
                let _name = &self.name;
                full_name.push_str(" ");
                full_name.push_str(_name);
                return full_name;
            }
            String::from(&self.surname)
        }

        /// Returns the client info of the Passport's only owner
        #[ink(message)]
        pub fn get_info(&self) -> String {
            assert_eq!(
                self.env().caller(),
                self.owner,
                "Only owner have permissions to get client info"
            );
            String::from(&self.client)
        }
    }


    #[cfg(test)]
    mod tests {
        use super::*;
        use ink_lang as ink;

        #[ink::test]
        fn it_works() {
            let mut passport = Passport::new("Ivan".to_string(), "Ivanov".to_string(), "01.01.1970".to_string(), "INN: 123456".to_string());
            assert!(passport.get_status());
            passport.set_status(false);
            assert!(!passport.get_status());
            assert_eq!(passport.get_name(), "Ivanov Ivan".to_string());
        }
    }
}