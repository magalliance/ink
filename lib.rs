#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
pub mod passport {
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
            Self { name, surname, birth, assets: Default::default(), client, owner: Self::env().caller(), is_active: true }
        }

        /// Returns the current value of the Passport's boolean
        #[ink(message)]
        pub fn get_status(&self) -> bool {
            self.is_active
        }

        ///  Set passport's status only owner
        #[ink(message)]
        pub fn set_status(&mut self, status: bool) {
            assert!(self.owner, ink_env::caller());
            self.is_active = status;
        }

        /// Returns the current name of the Passport's
        #[ink(message)]
        pub fn get_name(&self) -> String {
            if self.owner == ink_env::caller() {
                format!("{} {}", *self.surname, *self.name);
            }
            String::from(*self.name)
        }

        /// Returns the client info of the Passport's only owner
        #[ink(message)]
        pub fn get_info(&self) -> Client {
            assert!(self.owner, ink_env::caller());
            *self.client
        }
    }


    #[cfg(test)]
    mod tests {
        use super::*;
        use ink_lang as ink;

        #[ink::test]
        fn it_works() {
            let mut passport = Passport::new("name", "surname", "birth", "inn");
            assert!(passport.get_status());

            passport.set_status(false);
            assert!(!passport.get_status());

            passport.set_status(true);
            assert!(passport.get_status());
        }
    }
}