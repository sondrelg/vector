use crate::Value;
use bytes::Bytes;
use chrono::{DateTime, NaiveDateTime, Utc};
use quickcheck::{Arbitrary, Gen};
use std::collections::BTreeMap;

const MAX_ARRAY_SIZE: usize = 4;
const MAX_MAP_SIZE: usize = 4;
const MAX_F64_SIZE: f64 = 1_000_000.0;

fn datetime(g: &mut Gen) -> DateTime<Utc> {
    // `chrono` documents that there is an out-of-range for both second and
    // nanosecond values but doesn't actually document what the valid ranges
    // are. We just sort of arbitrarily restrict things.
    let secs = i64::arbitrary(g) % 32_000;
    let nanosecs = u32::arbitrary(g) % 32_000;
    DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(secs, nanosecs), Utc)
}

impl Arbitrary for Value {
    fn arbitrary(g: &mut Gen) -> Self {
        // Quickcheck can't derive Arbitrary for enums, see
        // https://github.com/BurntSushi/quickcheck/issues/98.  The magical
        // constant here are the number of fields in `Value`. Because the field
        // total is a power of two we, happily, don't introduce a bias into the
        // field picking.
        match u8::arbitrary(g) % 8 {
            0 => {
                let bytes: Vec<u8> = Vec::arbitrary(g);
                Self::Bytes(Bytes::from(bytes))
            }
            1 => Self::Integer(i64::arbitrary(g)),
            2 => {
                let mut f = f64::arbitrary(g) % MAX_F64_SIZE;
                if f.is_nan() {
                    f = 0.0;
                }
                Self::from(f)
            }
            3 => Self::Boolean(bool::arbitrary(g)),
            4 => Self::Timestamp(datetime(g)),
            5 => {
                let mut gen = Gen::new(MAX_MAP_SIZE);
                Self::Map(BTreeMap::arbitrary(&mut gen))
            }
            6 => {
                let mut gen = Gen::new(MAX_ARRAY_SIZE);
                Self::Array(Vec::arbitrary(&mut gen))
            }
            7 => Self::Null,
            _ => unreachable!(),
        }
    }
}
