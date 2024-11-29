use crate::ArwenApiImpl;
use numbat_wasm::{api::VMApi, numbat_codec::TryStaticCast};

impl TryStaticCast for ArwenApiImpl {}

impl VMApi for ArwenApiImpl {}
