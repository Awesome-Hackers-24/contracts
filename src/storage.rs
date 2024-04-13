use multiversx_sc::{derive_imports::*, imports::*};

#[multiversx_sc::module]
pub trait StorageModule {

    #[view(getNftTokenId)]
    #[storage_mapper("nftTokenId")]
    fn nft_token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(getNftTokenName)]
    #[storage_mapper("nftTokenName")]
    fn nft_token_name(&self) -> SingleValueMapper<ManagedBuffer>;

    #[view(getNftTokenCid)]
    #[storage_mapper("nftTokenCid")]
    fn nft_token_cid(&self) -> SingleValueMapper<ManagedBuffer>;

}