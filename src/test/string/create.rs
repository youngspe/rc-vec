use crate::rc_str;
use crate::string::RcString;

#[test]
pub fn create_from_macro() {
    let s = rc_str!("hello, world!");
    assert_eq!(s, "hello, world!");
}

#[test]
pub fn create_from_macro_with_args() {
    let s = rc_str!("foo: {}, bar: {}", 1, "baz");
    assert_eq!(s, "foo: 1, bar: baz");
}

#[test]
pub fn from_str() {
    let s = RcString::from("hello, world?");
    assert_eq!(s, "hello, world?");
}
