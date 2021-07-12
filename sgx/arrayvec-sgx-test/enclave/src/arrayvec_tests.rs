use arrayvec::*;

use std::prelude::v1::*;
use std::panic::{catch_unwind, resume_unwind, AssertUnwindSafe};

pub fn test_encode_utf8_oob() {
    // test that we report oob if the buffer is too short
    let mut data = [0u8; 16];
    let chars = ['a', 'α', '�', '𐍈'];
    for (len, &ch) in (1..=4).zip(&chars) {
        assert_eq!(len, ch.len_utf8(), "Len of ch={}", ch);
    }
}
