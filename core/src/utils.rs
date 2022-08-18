use std::ops::{Div, Mul};

use crate::error::Error;

pub fn sat_to_msat<T>(sat: T) -> T
where
    T: Mul<u64, Output = T>,
{
    sat * 1_000
}

pub fn msat_to_sat<T>(msat: T) -> T
where
    T: Div<u64, Output = T>,
{
    msat / 1_000
}

pub fn get_amount_msat<T>(sat: Option<T>, msat: Option<T>) -> Option<T>
where
    T: Mul<u64, Output = T> + Div<u64, Output = T>,
{
    match (sat, msat) {
        (Some(sat), _) => Some(sat_to_msat(sat)),
        (_, Some(msat)) => Some(msat),
        (None, None) => None,
    }
}

pub fn get_amount_sat<T>(sat: Option<T>, msat: Option<T>) -> Option<T>
where
    T: Mul<u64, Output = T> + Div<u64, Output = T>,
{
    match (sat, msat) {
        (Some(sat), _) => Some(sat),
        (_, Some(msat)) => Some(msat_to_sat(msat)),
        (None, None) => None,
    }
}

pub fn b64_to_hex(b64: &str) -> Result<String, Error> {
    let bytes = base64::decode(b64)?;
    Ok(hex::encode(&bytes))
}
