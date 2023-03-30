#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod shibuya34 {
    use openbrush::{
        contracts::ownable::*,
        contracts::psp34::extensions::{enumerable::*, metadata::*},
        traits::{Storage, String},
    };
    use payable_mint_pkg::impls::payable_mint::*;
    use payable_mint_pkg::traits::payable_mint::{self, *};

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Shibuya34 {
        #[storage_field]
        psp34: psp34::Data<enumerable::Balances>,
        #[storage_field]
        ownable: ownable::Data,
        #[storage_field]
        metadata: metadata::Data,
        #[storage_field]
        payable_mint_data: types::Data,
    }

    impl PSP34 for Shibuya34 {}
    impl Ownable for Shibuya34 {}
    impl PSP34Enumerable for Shibuya34 {}
    impl PSP34Metadata for Shibuya34 {}
    impl PayableMint for Shibuya34 {}

    impl Shibuya34 {
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut instance = Self::default();
            instance._init_with_owner(instance.env().caller());
            instance
                ._mint_to(instance.env().caller(), Id::U8(1))
                .expect("Can't mint");
            let collection_id = instance.collection_id();
            instance._set_attribute(
                collection_id.clone(),
                String::from("name"),
                String::from("Shibuya34"),
            );
            instance._set_attribute(collection_id, String::from("symbol"), String::from("SH34"));
            instance
        }
    }
}
