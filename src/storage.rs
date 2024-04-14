use multiversx_sc::{derive_imports::*, imports::*};

#[multiversx_sc::module]
pub trait StorageModule {

    #[endpoint]
    fn add_whitelisted_address(&self, address: ManagedAddress) {
        self.whitelisted_address().set(address);
    }

    #[view(getNftTokenId)]
    #[storage_mapper("nftTokenId")]
    fn nft_token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(getNftTokenName)]
    #[storage_mapper("nftTokenName")]
    fn nft_token_name(&self) -> SingleValueMapper<ManagedBuffer>;

    #[view(getNftTokenCid)]
    #[storage_mapper("nftTokenCid")]
    fn nft_token_cid(&self) -> SingleValueMapper<ManagedBuffer>;

    #[view]
    #[storage_mapper("getWhitelistedAddress")]
    fn whitelisted_address(&self) -> SingleValueMapper<ManagedAddress>;

    #[storage_mapper("is_paused")]
    fn is_paused(&self) -> SingleValueMapper<bool>;

}