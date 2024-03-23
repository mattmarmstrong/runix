// T has to be the size of a usize or smaller
pub fn is_bit_set<T>(value: T, bit: usize) -> bool
where
    T: Into<usize>,
{
    let value_usize: usize = value.into();
    value_usize & bit == bit
}

pub fn set_bit<T>(value: T, bit: usize) -> T
where
    T: Into<usize> + From<usize>,
{
    let value_usize: usize = value.into();
    T::from(value_usize | bit)
}
