pub fn sat_to_msat(sat: &u64) -> u64 {
    sat * 1000
}

pub fn msat_to_sat(msat: &u64) -> u64 {
    msat / 1000
}
