use crate::rc_str;

#[test]
pub fn add_str() {
    let s = rc_str!("foo") + "bar";
    assert_eq!(s, "foobar");
}

#[test]
pub fn add_self() {
    let s = rc_str!("foo") + rc_str!("bar");
    assert_eq!(s, "foobar");
}

#[test]
pub fn add_assign_str() {
    let mut s = rc_str!("foo");
    s += "bar";
    assert_eq!(s, "foobar");
}

#[test]
pub fn add_assign_self() {
    let mut s = rc_str!("foo");
    s += rc_str!("bar");
    assert_eq!(s, "foobar");
}

#[test]
pub fn add_str_cloned() {
    let s1 = rc_str!("foo");
    let s2 = s1.clone() + "bar";
    assert_eq!(s1, "foo");
    assert_eq!(s2, "foobar");
}

#[test]
pub fn add_assign_str_cloned() {
    let s1 = rc_str!("foo");
    let mut s2 = s1.clone();
    s2 += "bar";
    assert_eq!(s1, "foo");
    assert_eq!(s2, "foobar");
}

#[test]
pub fn add_assign_str_orig() {
    let mut s1 = rc_str!("foo");
    let s2 = s1.clone();
    s1 += "bar";
    assert_eq!(s1, "foobar");
    assert_eq!(s2, "foo");
}

#[test]
pub fn sort_strs() {
    let mut arr = [
        rc_str!("foo"),
        rc_str!("baz"),
        rc_str!("qux"),
        rc_str!("bar"),
        rc_str!(""),
        rc_str!("food"),
    ];
    arr.sort();
    let expected = [
        rc_str!(""),
        rc_str!("bar"),
        rc_str!("baz"),
        rc_str!("foo"),
        rc_str!("food"),
        rc_str!("qux"),
    ];

    assert_eq!(arr, expected);
}

#[test]
pub fn debug() {
    let s = rc_str!("foo\nbar");
    assert_eq!(format!("{:?}", s), "\"foo\\nbar\"")
}

#[test]
pub fn display() {
    let s = rc_str!("foo\nbar");
    assert_eq!(format!("{}", s), "foo\nbar")
}
