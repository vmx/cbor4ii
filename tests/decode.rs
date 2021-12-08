#![cfg(feature = "use_alloc")]

use std::convert::Infallible;
use cbor4ii::core::Value;
use cbor4ii::core::dec::{ self, Decode };


struct SliceReader<'a>(&'a [u8]);

impl<'de> dec::Read<'de> for SliceReader<'de> {
    type Error = Infallible;

    fn fill<'b>(&'b mut self, want: usize) -> Result<dec::Reference<'de, 'b>, Self::Error> {
        let len = std::cmp::min(self.0.len(), want);

        Ok(dec::Reference::Long(&self.0[..len]))
    }

    fn advance(&mut self, n: usize) {
        debug_assert!(n <= self.0.len());

        self.0 = &self.0[n..];
    }
}

#[test]
fn test_decode_value() {
    macro_rules! test {
        ( @ $bytes:expr ) => {
            let mut reader = SliceReader($bytes);
            let _ = Value::decode(&mut reader);
        };
        ( $( $bytes:expr );* $( ; )? ) => {
            $(
                test!(@ $bytes );
            )*
        }
    }

    test!{
        &[0x8a];
        &[0x7a, 0x86];
    }
}
