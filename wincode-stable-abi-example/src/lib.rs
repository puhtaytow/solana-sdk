#![cfg_attr(feature = "frozen-abi", feature(specialization))]
#![allow(incomplete_features)]

use serde_derive::{Deserialize, Serialize};
#[cfg(feature = "frozen-abi")]
use solana_frozen_abi_macro::frozen_abi;
use wincode::{SchemaRead, SchemaWrite};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SchemaWrite, SchemaRead)]
#[cfg_attr(
    feature = "frozen-abi",
    derive(
        solana_frozen_abi_macro::AbiExample,
        solana_frozen_abi_macro::StableAbi
    ),
    frozen_abi(
        api_digest = "Dg8oSsrG6xKMzPhK32tcePQdvS8JtyodF2BZz8k7NPaS",
        abi_digest = "Ed9z2inVXfgQrgo4aHo77Cx38hSap5ACxib5ZdfGzTyp",
        abi_serializer = "wincode",
    )
)]
pub struct WincodeType {
    pub amount: u64,
    pub flag: bool,
    pub label: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(
    feature = "frozen-abi",
    derive(
        solana_frozen_abi_macro::AbiExample,
        solana_frozen_abi_macro::StableAbi
    ),
    frozen_abi(
        api_digest = "56VnV9y69vULx6hP4KRnE4UeM28HjCZTtbbUUE6fq68r",
        abi_digest = "Ed9z2inVXfgQrgo4aHo77Cx38hSap5ACxib5ZdfGzTyp"
    )
)]
pub struct BincodeType {
    pub amount: u64,
    pub flag: bool,
    pub label: String,
}

#[cfg(feature = "frozen-abi")]
impl solana_frozen_abi::rand::distr::Distribution<WincodeType>
    for solana_frozen_abi::rand::distr::StandardUniform
{
    fn sample<R: solana_frozen_abi::rand::Rng + ?Sized>(&self, rng: &mut R) -> WincodeType {
        WincodeType {
            amount: rng.random(),
            flag: rng.random(),
            label: format!("str{}", rng.random::<u32>()),
        }
    }
}

#[cfg(feature = "frozen-abi")]
impl solana_frozen_abi::rand::distr::Distribution<BincodeType>
    for solana_frozen_abi::rand::distr::StandardUniform
{
    fn sample<R: solana_frozen_abi::rand::Rng + ?Sized>(&self, rng: &mut R) -> BincodeType {
        BincodeType {
            amount: rng.random(),
            flag: rng.random(),
            label: format!("str{}", rng.random::<u32>()),
        }
    }
}
