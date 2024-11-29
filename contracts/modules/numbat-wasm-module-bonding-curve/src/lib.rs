#![no_std]

numbat_wasm::imports!();
numbat_wasm::derive_imports!();

pub mod curves;
pub mod function_selector;
pub mod utils;
use crate::utils::{events, owner_endpoints, storage, user_endpoints};

#[numbat_wasm::module]
pub trait BondingCurveModule:
    storage::StorageModule
    + events::EventsModule
    + user_endpoints::UserEndpointsModule
    + owner_endpoints::OwnerEndpointsModule
{
}
