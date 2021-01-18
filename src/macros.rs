#[macro_export]
macro_rules! rc_vec {
    ($($val:expr),* $(,)?) => {{
        let mut tmp = [$($val),*];
        let v = unsafe { $crate::vec::RcVec::copy_from_ptr_unsafe(tmp.as_mut_ptr(), tmp.len()) };
        core::mem::forget(tmp);
        v
    }};
    ($val:expr; $len:expr) => {
        core::iter::repeat($val).take($len).collect::<$crate::vec::RcVec<_>>()
    };
}
