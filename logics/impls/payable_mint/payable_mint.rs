use ink::prelude::string::{String as PreludeString, ToString};

use crate::impls::payable_mint::types::Data;
pub use crate::traits::payable_mint::PayableMint;

use openbrush::{
    contracts::{ownable::*, psp34::extensions::enumerable::*},
    modifiers,
    traits::{AccountId, Storage, String},
};

pub trait Internal {
    /// Check if the transferred mint values is as expected
    fn check_value(&self, transferred_value: u128, mint_amount: u64) -> Result<(), PSP34Error>;

    /// Check amount of tokens to be minted
    fn check_amount(&self, mint_amount: u64) -> Result<(), PSP34Error>;

    // Check if token is minted
    //fn token_exists(&self, id: Id) -> Result<(), PSP34Error>;
}

impl<T> PayableMint for T
where
    T: Storage<Data>
        + Storage<psp34::Data<enumerable::Balances>>
        + psp34::extensions::metadata::PSP34Metadata
        + psp34::Internal,
{
    default fn mint(&mut self, to: AccountId, mint_amount: u64) -> Result<(), PSP34Error> {
        self.check_value(Self::env().transferred_value(), mint_amount)?;
        self.check_amount(mint_amount)?;

        let next_to_mint = self.data::<Data>().last_token_id + 1; // first mint id is 1
        let mint_offset = next_to_mint + mint_amount;

        for mint_id in next_to_mint..mint_offset {
            self.data::<psp34::Data<enumerable::Balances>>()
                ._mint_to(to, Id::U64(mint_id))?;
            self.data::<Data>().last_token_id += 1;
        }

        Ok(())
    }

    /// Withdraws funds to contract owner
    #[modifiers(only_owner)]
    default fn withdraw(&mut self) -> Result<(), PSP34Error> {
        let balance = Self::env().balance();
        let current_balance = balance
            .checked_sub(Self::env().minimum_balance())
            .unwrap_or_default();
        Self::env()
            .transfer(self.data::<ownable::Data>().owner(), current_balance)
            .map_err(|_| PSP34Error::Custom(String::from("WithdrawalFailed")))?;
        Ok(())
    }
}

impl<T> Internal for T
where
    T: Storage<Data> + Storage<psp34::Data<enumerable::Balances>>,
{
    /// Check if transferred funds are sufficient for minting the desired nunmber of tokens.
    default fn check_value(
        &self,
        transferred_value: u128,
        mint_amount: u64,
    ) -> Result<(), PSP34Error> {
        if let Some(value) = (mint_amount as u128).checked_mul(self.data::<Data>().price_per_mint) {
            if transferred_value == value {
                return Ok(());
            }
        }
        return Err(PSP34Error::Custom(String::from("BadMintValue")));
    }
    /// Check if amount of tokens to be minted does not overflow max supply.
    default fn check_amount(&self, mint_amount: u64) -> Result<(), PSP34Error> {
        if mint_amount == 0 {
            return Err(PSP34Error::Custom(String::from("CannotMintZeroTokens")));
        }
        if let Some(amount) = self.data::<Data>().last_token_id.checked_add(mint_amount) {
            if amount <= self.data::<Data>().max_supply {
                return Ok(());
            }
        }
        return Err(PSP34Error::Custom(String::from("CollectionIsFull")));
    }
}
