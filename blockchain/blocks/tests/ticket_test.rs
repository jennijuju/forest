// Copyright 2019-2022 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0, MIT

use forest_blocks::*;
use forest_test_utils::construct_ticket;
use fvm_ipld_encoding::{from_slice, to_vec};

// From Lotus
const TICKET: [u8; 99] = [
    0x81, 0x58, 0x60, 0x96, 0x64, 0x49, 0x2f, 0x30, 0xe9, 0xb9, 0x50, 0x3b, 0x71, 0x41, 0x0b, 0x1d,
    0x38, 0x2e, 0x2b, 0xd4, 0x85, 0x7f, 0xe2, 0x15, 0x39, 0xac, 0x92, 0x1b, 0xcb, 0x7f, 0xd0, 0x86,
    0xd5, 0x78, 0x71, 0xe6, 0xdd, 0x5c, 0x31, 0xcd, 0x23, 0x61, 0x8b, 0x52, 0x52, 0xb6, 0x2c, 0x7b,
    0x44, 0x4c, 0x3a, 0x02, 0x9b, 0xba, 0xad, 0xc2, 0x50, 0x57, 0x56, 0x81, 0x06, 0x47, 0x77, 0xf6,
    0x04, 0x06, 0xc4, 0xff, 0x00, 0x6f, 0x38, 0xfc, 0x61, 0x71, 0xfe, 0x45, 0xd4, 0x83, 0xe5, 0x15,
    0x79, 0xd0, 0xe2, 0x47, 0x8b, 0x7e, 0x5f, 0xde, 0x2c, 0x51, 0xd2, 0xe8, 0x64, 0x63, 0xaf, 0x86,
    0xd3, 0xcb, 0xd5,
];

#[test]
fn encode_ticket() {
    let ticket = construct_ticket();
    // Encode Ticket
    let encoded_ticket = to_vec(&ticket).unwrap();
    assert_eq!(&TICKET[..], &encoded_ticket[..]);
}

#[test]
fn decode_ticket() {
    let ticket = construct_ticket();
    // Decode Ticket
    let decoded_ticket: Ticket = from_slice(&TICKET).unwrap();
    assert_eq!(ticket, decoded_ticket);
}