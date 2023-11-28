// T has to be the size of a u64 or smaller
pub fn is_bit_set<T>(value: T, bit: u64) -> bool
where
    T: Into<u64>,
{
    let value_u64: u64 = value.into();
    value_u64 & bit == bit
}

pub fn set_bit<T>(value: T, bit: u64) -> T
where
    T: Into<u64> + From<u64>,
{
    let value_u64: u64 = value.into();
    T::from(value_u64 | bit)
}
