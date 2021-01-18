use crate::rc_vec;
use crate::vec::RcVec;

#[test]
pub fn create_from_macro_list() {
    let v = rc_vec![1, 2, 3, 4, 5];
    assert_eq!(&*v, [1, 2, 3, 4, 5]);
}

#[test]
pub fn create_from_macro_count() {
    let v = rc_vec![7; 10];
    assert_eq!(&*v, [7, 7, 7, 7, 7, 7, 7, 7, 7, 7]);
}

#[test]
pub fn create_new() {
    let v = RcVec::<i32>::new();
    assert_eq!(&*v, [])
}
