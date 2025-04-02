use super::bit_array::BitArray;
pub trait Mantissa<const N:usize> {
    fn to_array(&self)->BitArray<N> where [(); N - 1]:;
}
