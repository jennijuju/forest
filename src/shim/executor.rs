// Copyright 2019-2023 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0, MIT

use crate::shim::econ::TokenAmount;
use fvm2::executor::ApplyRet as ApplyRet_v2;
use fvm3::executor::ApplyRet as ApplyRet_v3;
pub use fvm3::gas::GasCharge as GasChargeV3;
pub use fvm3::trace::ExecutionEvent as ExecutionEvent_v3;
use fvm_ipld_encoding::RawBytes;
use fvm_shared2::receipt::Receipt as Receipt_v2;
use fvm_shared3::error::ExitCode;
pub use fvm_shared3::receipt::Receipt as Receipt_v3;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, Debug)]
pub enum ApplyRet {
    V2(Box<ApplyRet_v2>),
    V3(Box<ApplyRet_v3>),
}

impl From<ApplyRet_v2> for ApplyRet {
    fn from(other: ApplyRet_v2) -> Self {
        ApplyRet::V2(Box::new(other))
    }
}

impl From<ApplyRet_v3> for ApplyRet {
    fn from(other: ApplyRet_v3) -> Self {
        ApplyRet::V3(Box::new(other))
    }
}

impl ApplyRet {
    pub fn failure_info(&self) -> Option<String> {
        match self {
            ApplyRet::V2(v2) => v2.failure_info.as_ref().map(|failure| failure.to_string()),
            ApplyRet::V3(v3) => v3.failure_info.as_ref().map(|failure| failure.to_string()),
        }
    }

    pub fn miner_tip(&self) -> TokenAmount {
        match self {
            ApplyRet::V2(v2) => (&v2.miner_tip).into(),
            ApplyRet::V3(v3) => (&v3.miner_tip).into(),
        }
    }

    pub fn penalty(&self) -> TokenAmount {
        match self {
            ApplyRet::V2(v2) => (&v2.penalty).into(),
            ApplyRet::V3(v3) => (&v3.penalty).into(),
        }
    }

    pub fn msg_receipt(&self) -> Receipt {
        match self {
            ApplyRet::V2(v2) => Receipt::V2(v2.msg_receipt.clone()),
            ApplyRet::V3(v3) => Receipt::V3(v3.msg_receipt.clone()),
        }
    }

    pub fn refund(&self) -> TokenAmount {
        match self {
            ApplyRet::V2(v2) => (&v2.refund).into(),
            ApplyRet::V3(v3) => (&v3.refund).into(),
        }
    }

    pub fn base_fee_burn(&self) -> TokenAmount {
        match self {
            ApplyRet::V2(v2) => (&v2.base_fee_burn).into(),
            ApplyRet::V3(v3) => (&v3.base_fee_burn).into(),
        }
    }

    pub fn over_estimation_burn(&self) -> TokenAmount {
        match self {
            ApplyRet::V2(v2) => (&v2.over_estimation_burn).into(),
            ApplyRet::V3(v3) => (&v3.over_estimation_burn).into(),
        }
    }

    pub fn exec_trace(&self) -> Vec<ExecutionEvent_v3> {
        match self {
            // We don't support fvm2 execution trace at the moment
            // Tracking issue: https://github.com/ChainSafe/forest/issues/3285
            ApplyRet::V2(_v2) => vec![],
            ApplyRet::V3(v3) => v3.exec_trace.clone(),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum Receipt {
    V2(Receipt_v2),
    V3(Receipt_v3),
}

impl Serialize for Receipt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Receipt::V2(v2) => v2.serialize(serializer),
            Receipt::V3(v3) => v3.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for Receipt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Receipt_v2::deserialize(deserializer).map(Receipt::V2)
    }
}

impl Receipt {
    pub fn exit_code(&self) -> ExitCode {
        match self {
            Receipt::V2(v2) => ExitCode::new(v2.exit_code.value()),
            Receipt::V3(v3) => v3.exit_code,
        }
    }

    pub fn return_data(&self) -> RawBytes {
        match self {
            Receipt::V2(v2) => RawBytes::from(v2.return_data.to_vec()),
            Receipt::V3(v3) => v3.return_data.clone(),
        }
    }

    pub fn gas_used(&self) -> u64 {
        match self {
            Receipt::V2(v2) => v2.gas_used as u64,
            Receipt::V3(v3) => v3.gas_used,
        }
    }
}

impl From<Receipt_v3> for Receipt {
    fn from(other: Receipt_v3) -> Self {
        Receipt::V3(other)
    }
}

#[cfg(test)]
impl quickcheck::Arbitrary for Receipt {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        #[derive(derive_quickcheck_arbitrary::Arbitrary, Clone)]
        enum Helper {
            V2 {
                exit_code: u32,
                return_data: Vec<u8>,
                gas_used: i64,
            },
            V3 {
                exit_code: u32,
                return_data: Vec<u8>,
                gas_used: u64,
                events_root: Option<::cid::Cid>,
            },
        }
        match Helper::arbitrary(g) {
            Helper::V2 {
                exit_code,
                return_data,
                gas_used,
            } => Self::V2(Receipt_v2 {
                exit_code: exit_code.into(),
                return_data: return_data.into(),
                gas_used,
            }),
            Helper::V3 {
                exit_code,
                return_data,
                gas_used,
                events_root,
            } => Self::V3(Receipt_v3 {
                exit_code: exit_code.into(),
                return_data: return_data.into(),
                gas_used,
                events_root,
            }),
        }
    }
}
