// Copyright 2019-2023 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0, MIT

pub mod json {
    use crate::shim::executor::TraceGasCharge;

    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    use std::borrow::Cow;

    /// Wrapper for serializing and de-serializing a `TraceGasCharge` from JSON.
    #[derive(Deserialize, Serialize, Debug)]
    #[serde(transparent)]
    pub struct TraceGasChargeJson(#[serde(with = "self")] pub TraceGasCharge);

    /// Wrapper for serializing a `TraceGasCharge` reference to JSON.
    #[derive(Serialize)]
    #[serde(transparent)]
    pub struct TraceGasChargeJsonRef<'a>(#[serde(with = "self")] pub &'a TraceGasCharge);

    impl From<TraceGasChargeJson> for TraceGasCharge {
        fn from(wrapper: TraceGasChargeJson) -> Self {
            wrapper.0
        }
    }

    impl From<TraceGasCharge> for TraceGasChargeJson {
        fn from(wrapper: TraceGasCharge) -> Self {
            TraceGasChargeJson(wrapper)
        }
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    struct JsonHelper {
        pub name: Cow<'static, str>,
        #[serde(rename = "tg")]
        pub total_gas: u64,
        #[serde(rename = "cg")]
        pub compute_gas: u64,
        #[serde(rename = "sg")]
        pub other_gas: u64,
        #[serde(rename = "tt")]
        pub duration_nanos: u64,
    }

    pub fn serialize<S>(gc: &TraceGasCharge, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        JsonHelper {
            name: gc.name.clone(),
            total_gas: gc.total_gas,
            compute_gas: gc.compute_gas,
            other_gas: gc.other_gas,
            duration_nanos: gc.duration_nanos,
        }
        .serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<TraceGasCharge, D::Error>
    where
        D: Deserializer<'de>,
    {
        let gc: JsonHelper = Deserialize::deserialize(deserializer)?;
        Ok(TraceGasCharge {
            name: gc.name.clone(),
            total_gas: gc.total_gas,
            compute_gas: gc.compute_gas,
            other_gas: gc.other_gas,
            duration_nanos: gc.duration_nanos,
        })
    }

    pub mod vec {
        use crate::utils::json::GoVecVisitor;
        use serde::ser::SerializeSeq;

        use super::*;

        pub fn serialize<S>(m: &[TraceGasCharge], serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut seq = serializer.serialize_seq(Some(m.len()))?;
            for e in m {
                seq.serialize_element(&TraceGasChargeJsonRef(e))?;
            }
            seq.end()
        }

        pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<TraceGasCharge>, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_any(GoVecVisitor::<TraceGasCharge, TraceGasChargeJson>::new())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::shim::executor::TraceGasCharge;
    use quickcheck_macros::quickcheck;

    use super::*;

    impl quickcheck::Arbitrary for TraceGasCharge {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            Self {
                name: String::arbitrary(g).into(),
                total_gas: u64::arbitrary(g),
                compute_gas: u64::arbitrary(g),
                other_gas: u64::arbitrary(g),
                duration_nanos: u64::arbitrary(g),
            }
        }
    }

    #[quickcheck]
    fn trace_gas_charge_roundtrip(gc: TraceGasCharge) {
        let serialized = crate::to_string_with!(&gc, json::serialize);
        let parsed: TraceGasCharge = crate::from_str_with!(&serialized, json::deserialize);
        assert_eq!(gc, parsed);
    }
}