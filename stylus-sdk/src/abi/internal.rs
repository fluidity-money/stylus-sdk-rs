// Copyright 2023-2024, Offchain Labs, Inc.
// For licensing, see https://github.com/OffchainLabs/stylus-sdk-rs/blob/stylus/licenses/COPYRIGHT.md

//! This module provides functions for code generated by `stylus-sdk-proc`.
//! Most users shouldn't call these.

use crate::{abi::AbiType, console, msg, ArbResult};
use alloc::{vec, vec::Vec};
use alloy_primitives::U256;
use alloy_sol_types::SolType;
use core::fmt;

pub trait EncodableReturnType {
    fn encode(self) -> ArbResult;
}

impl<T> EncodableReturnType for T
where
    T: AbiType + alloy_sol_types::private::SolTypeValue<<T as AbiType>::SolType>,
{
    #[inline(always)]
    fn encode(self) -> ArbResult {
        // coerce types into a tuple of at least 1 element
        Ok(<<T as AbiType>::SolType>::abi_encode(&self))
    }
}

impl<T, E: Into<Vec<u8>>> EncodableReturnType for Result<T, E>
where
    T: AbiType + alloy_sol_types::private::SolTypeValue<<T as AbiType>::SolType>,
{
    #[inline(always)]
    fn encode(self) -> ArbResult {
        match self {
            Ok(result) => result.encode(),
            Err(err) => Err(err.into()),
        }
    }
}

#[inline(always)]
pub const fn digest_to_selector(digest: [u8; 32]) -> [u8; 4] {
    let mut selector = [0u8; 4];
    selector[0] = digest[0];
    selector[1] = digest[1];
    selector[2] = digest[2];
    selector[3] = digest[3];
    selector
}

#[allow(unused)]
pub fn deny_value(method_name: &str) -> Result<(), Vec<u8>> {
    if msg::value() == U256::ZERO {
        return Ok(());
    }
    console!("method {method_name} not payable");
    Err(vec![])
}

#[allow(unused)]
pub fn failed_to_decode_arguments(err: alloy_sol_types::Error) {
    console!("failed to decode arguments: {err}");
}

pub trait AbiResult {
    type OkType;
}

impl<O, E> AbiResult for Result<O, E> {
    type OkType = O;
}

impl<T: AbiType> AbiResult for T {
    type OkType = T;
}

pub fn write_solidity_returns<T: AbiResult>(f: &mut fmt::Formatter) -> fmt::Result
where
    T::OkType: AbiType,
{
    let abi = T::OkType::EXPORT_ABI_RET.as_str();
    if abi == "()" {
        Ok(())
    } else if abi.starts_with('(') {
        write!(f, " returns {abi}")
    } else {
        write!(f, " returns ({abi})")
    }
}
