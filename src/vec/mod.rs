pub mod strong_weak;
pub mod counters;
pub mod generic_vec;

pub type AcycVec<T> = generic_vec::GenericVec<counters::AcycType, T>;
pub type RcVec<T> = generic_vec::GenericVec<counters::StrongType, T>;
