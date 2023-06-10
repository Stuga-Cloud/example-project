use bigdecimal::{BigDecimal, ToPrimitive};
use serde::Serializer;

pub fn serialize_bigdecimal<S>(value: &BigDecimal, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let rounded = value.round(2);
    let as_f64 = rounded.to_f64().unwrap_or(0.0);
    serializer.serialize_f64(as_f64)
}
