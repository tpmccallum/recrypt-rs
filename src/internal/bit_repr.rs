use crate::internal::fp::fr_256::Fr256;
use gridiron::fp_256::Fp256;

/// A bit representation of a numeric value
/// CT: It seems like exposing the ContsantBool might be better if we need to actually decision based on a bit
/// However, the constant time recomendation for double and add is to instead look at many bits at once to save 
/// adds. We might need to change this based on what we find there.
pub trait BitRepr {
    fn to_bits(&self) -> Vec<u8>;
}

impl BitRepr for Fp256 {
    fn to_bits(&self) -> Vec<u8> {
        (*self).iter_bit().map(|x| x.0 as u8).collect()
    }
}

impl BitRepr for Fr256 {
    fn to_bits(&self) -> Vec<u8> {
        (*self).iter_bit().map(|x| x.0 as u8).collect()
    }
}
