use std::str::FromStr;

use crate::error::Error;

pub fn sat_to_msat(sat: &u64) -> u64 {
    sat * 1000
}

pub fn msat_to_sat(msat: &u64) -> u64 {
    msat / 1000
}

pub fn convert_base64_to_hex(hex: &str) -> Result<String, Error> {
    let base64_decoded = base64::decode(hex);
    match base64_decoded {
        Ok(e) => Ok(hex::encode(e)),
        Err(_) => Err(Error::ConversionError(String::from(
            "coudln't convert from base64 to hex",
        ))),
    }
}

pub fn parse_number<T: FromStr>(text: &str) -> Result<T, T::Err> {
    text.trim().parse::<T>()
}
