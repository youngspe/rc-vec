use crate::string::RcString;
use alloc::string::String;
use alloc::vec::Vec;
use core::iter::FromIterator;

fn base_str_iter_empty_test<S>()
where
    RcString: FromIterator<S>,
{
    let src = core::iter::empty::<S>();
    let s: RcString = src.collect();
    assert_eq!(s, "");
}

fn base_str_iter_test<S>()
where
    &'static str: Into<S>,
    RcString: FromIterator<S>,
{
    let s: RcString = vec!["foo", "bar", "baz"]
        .into_iter()
        .map(<&str as Into<S>>::into)
        .collect();
    assert_eq!(s, "foobarbaz");
}

fn base_str_iter_ref_test<S>()
where
    &'static str: Into<S>,
    for<'a> RcString: FromIterator<&'a S>,
{
    let v: Vec<S> = vec!["foo", "bar", "baz"]
        .into_iter()
        .map(<&str as Into<S>>::into)
        .collect();
    let s: RcString = v.iter().collect();
    assert_eq!(s, "foobarbaz");
}

#[test]
pub fn from_str_iter_empty() {
    base_str_iter_empty_test::<&str>();
}

#[test]
pub fn from_str_iter() {
    base_str_iter_test::<&str>();
}

#[test]
pub fn from_self_iter_empty() {
    base_str_iter_empty_test::<RcString>();
}

#[test]
pub fn from_self_iter() {
    base_str_iter_test::<RcString>();
}

#[test]
pub fn from_self_ref_iter_empty() {
    base_str_iter_empty_test::<&RcString>();
}

#[test]
pub fn from_self_ref_iter() {
    base_str_iter_ref_test::<RcString>();
}

#[test]
pub fn from_string_iter_empty() {
    base_str_iter_empty_test::<String>();
}

#[test]
pub fn from_string_iter() {
    base_str_iter_test::<String>();
}

#[test]
pub fn from_string_ref_iter_empty() {
    base_str_iter_empty_test::<&String>();
}

#[test]
pub fn from_string_ref_iter() {
    base_str_iter_ref_test::<String>();
}
