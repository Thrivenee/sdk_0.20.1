use crate::{
    abi::{OutputAbi, TypeAbi, TypeDescriptionContainer},
    api::{SendApi, StorageReadApi},
    contract_base::SendWrapper,
    io::EndpointResult,
    types::{BigUint, ManagedAddress, ManagedBuffer, TokenIdentifier},
};
use alloc::{string::String, vec::Vec};

pub struct SendDcdt<SA>
where
    SA: SendApi + StorageReadApi + 'static,
{
    pub(super) api: SA,
    pub(super) to: ManagedAddress<SA>,
    pub(super) token_identifier: TokenIdentifier<SA>,
    pub(super) amount: BigUint<SA>,
    pub data: ManagedBuffer<SA>,
}

impl<SA> EndpointResult for SendDcdt<SA>
where
    SA: SendApi + StorageReadApi + 'static,
{
    type DecodeAs = ();

    #[inline]
    fn finish<FA>(&self, _api: FA) {
        SendWrapper::new(self.api.clone()).transfer_dcdt_via_async_call(
            &self.to,
            &self.token_identifier,
            0,
            &self.amount,
            self.data.clone(),
        );
    }
}

impl<SA> TypeAbi for SendDcdt<SA>
where
    SA: SendApi + StorageReadApi + 'static,
{
    fn type_name() -> String {
        "SendDcdt".into()
    }

    /// No ABI output.
    fn output_abis(_: &[&'static str]) -> Vec<OutputAbi> {
        Vec::new()
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(_: &mut TDC) {}
}
