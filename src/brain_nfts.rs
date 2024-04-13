#![no_std]

use multiversx_sc::imports::*;

mod storage;

/// One of the simplest smart contracts possible,
/// it holds a single variable in storage, which anyone can increment.
#[multiversx_sc::contract]
pub trait BrainNfts {
    #[view(getSum)]
    #[storage_mapper("sum")]
    fn sum(&self) -> SingleValueMapper<BigUint>;

    #[init]
    fn init(&self, initial_value: BigUint) {
        
    }

    #[upgrade]
    fn upgrade(&self, initial_value: BigUint) {
        self.init(initial_value);
    }

    #[only_owner]
    #[payable("EGLD")]
    #[endpoint(issueToken)]
    fn issue_collection(
        &self,
        #[payment] issue_cost: BigUint,
        collection_name: ManagedBuffer,
        collection_ticker: ManagedBuffer,
    ) {
        require!(self.nft_token_id().is_empty(), "Token already issued!");
        self.nft_token_name().set(&collection_name);
        self.send()
            .esdt_system_sc_proxy()
            .issue_non_fungible(
                issue_cost,
                &collection_name,
                &collection_ticker,
                NonFungibleTokenProperties {
                    can_freeze: true,
                    can_wipe: true,
                    can_pause: true,
                    can_change_owner: true,
                    can_upgrade: true,
                    can_add_special_roles: true,
                },
            )
            .async_call()
            .with_callback(self.callbacks().issue_callback())
            .call_and_exit()
    }

    #[callback]
    fn issue_callback(&self, #[call_result] result: ManagedAsyncCallResult<TokenIdentifier>) {
        match result {
            ManagedAsyncCallResult::Ok(token_id) => {
                self.nft_token_id().set(&token_id);
            }
            ManagedAsyncCallResult::Err(_) => {
                let caller = self.blockchain().get_owner_address();
                let (returned_tokens, token_id) = self.call_value().payment_token_pair();
                if token_id.is_egld() && returned_tokens > 0 {
                    self.send()
                        .direct(&caller, &token_id, 0, &returned_tokens, &[]);
                }
            }
        }
    }

    #[only_owner]
    #[endpoint(setLocalRoles)]
    fn set_local_roles(&self) {
        require!(!self.nft_token_id().is_empty(), "Token not issued!");
        let roles = [EsdtLocalRole::NftCreate, EsdtLocalRole::NftBurn];
        self.send()
            .esdt_system_sc_proxy()
            .set_special_roles(
                &self.blockchain().get_sc_address(),
                &self.nft_token_id().get(),
                roles.iter().cloned(),
            )
            .async_call()
            .call_and_exit()
    }

    #[endpoint]
    fn claim_nft() {
        require!(self.is_paused().get() == false, "Contract is paused");

        let caller = self.blockchain().get_caller_address();
        let token_id = self.nft_token_id().get();
    }

    /// Add desired amount to the storage variable.
    #[endpoint]
    fn add(&self, value: BigUint) {
        self.sum().update(|sum| *sum += value);
    }
}
