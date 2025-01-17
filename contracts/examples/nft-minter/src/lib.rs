#![no_std]

numbat_wasm::imports!();
numbat_wasm::derive_imports!();

mod nft_module;

#[derive(TypeAbi, TopEncode, TopDecode)]
pub struct ExampleAttributes {
    pub creation_timestamp: u64,
}

#[numbat_wasm::contract]
pub trait NftMinter: nft_module::NftModule {
    #[init]
    fn init(&self) {}

    #[allow(clippy::too_many_arguments)]
    #[only_owner]
    #[endpoint(createNft)]
    fn create_nft(
        &self,
        name: ManagedBuffer,
        royalties: BigUint,
        uri: ManagedBuffer,
        selling_price: BigUint,
        #[var_args] opt_token_used_as_payment: OptionalArg<TokenIdentifier>,
        #[var_args] opt_token_used_as_payment_nonce: OptionalArg<u64>,
    ) -> SCResult<u64> {
        let token_used_as_payment = opt_token_used_as_payment
            .into_option()
            .unwrap_or_else(|| self.types().token_identifier_rewa());

        let token_used_as_payment_nonce = if token_used_as_payment.is_rewa() {
            0
        } else {
            opt_token_used_as_payment_nonce
                .into_option()
                .unwrap_or_default()
        };

        let attributes = ExampleAttributes {
            creation_timestamp: self.blockchain().get_block_timestamp(),
        };
        self.create_nft_with_attributes(
            name,
            royalties,
            attributes,
            uri,
            selling_price,
            token_used_as_payment,
            token_used_as_payment_nonce,
        )
    }
}
