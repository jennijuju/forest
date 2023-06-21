#![cfg(test)]

use std::fmt::Debug;

use cid::Cid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
enum UntaggedFoo {
    Cid(Cid),
    Other,
}

#[track_caller]
fn round_trip<T, E>(
    t: T,
    serializer: impl Fn(&T) -> Result<Vec<u8>, E>,
    deserializer: impl Fn(&[u8]) -> Result<T, E>,
) where
    E: Debug,
    T: PartialEq + Debug,
{
    let serialized: Vec<u8> = serializer(&t).expect("couldn't serialize");
    let deserialized = deserializer(&serialized).expect("couldn't deserialize");
    assert_eq!(t, deserialized);
}

#[test]
fn cs_serde_cbor() {
    round_trip(
        UntaggedFoo::Cid(Cid::default()),
        cs_serde_cbor::to_vec,
        |s| cs_serde_cbor::from_slice(s),
    )
}

// Fails: "Only bytes can be deserialized into a CID"
// This is because using an untagged enum here requires `deserialize_any`, and we hit this line:
// - https://github.com/ipld/serde_ipld_dagcbor/blob/ddd7fe885e29442af36937ee26664a30c043fb4c/src/de.rs#L613-L617
//   (Note there's also this: https://github.com/ipld/libipld/blob/8478d6d66576636b9970cb3b00a232be7a88ea42/core/src/serde/de.rs#L223-L225
//   but I don't think that's what we're hitting here)
#[test]
fn fvm_ipld_encoding() {
    round_trip(
        UntaggedFoo::Cid(Cid::default()),
        fvm_ipld_encoding::to_vec,
        |s| fvm_ipld_encoding::from_slice(s),
    )
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
enum TaggedFoo {
    Cid(Cid),
    Other,
}

#[test]
fn other_cases() {
    round_trip(
        TaggedFoo::Cid(Cid::default()),
        fvm_ipld_encoding::to_vec,
        |s| fvm_ipld_encoding::from_slice(s),
    );

    round_trip(TaggedFoo::Cid(Cid::default()), cs_serde_cbor::to_vec, |s| {
        cs_serde_cbor::from_slice(s)
    });
}
