use crate::rc_vec;
use core::mem;

#[test]
pub fn downgrade_upgrade() {
    let v1 = rc_vec![1, 2, 3, 4];
    let w = v1.downgrade();
    let v2 = w.upgrade().unwrap();
    assert_eq!(&*v1, [1, 2, 3, 4]);
    assert_eq!(&*v2, [1, 2, 3, 4]);
}

#[test]
pub fn upgrade_should_fail() {
    let v = rc_vec![1, 2, 3, 4];
    let w = v.downgrade();
    mem::drop(v);
    assert_eq!(w.upgrade(), None);
}

#[test]
pub fn upgrade_after_clone() {
    let v1 = rc_vec![1, 2, 3, 4];
    let w = v1.downgrade();
    let v2 = v1.clone();
    mem::drop(v1);
    let v3 = w.upgrade().unwrap();
    assert_eq!(&*v2, [1, 2, 3, 4]);
    assert_eq!(&*v3, [1, 2, 3, 4]);
}


#[test]
pub fn upgrade_after_clone_dropped() {
    let v1 = rc_vec![1, 2, 3, 4];
    let w = v1.downgrade();
    let v2 = v1.clone();
    mem::drop(v2);
    let v3 = w.upgrade().unwrap();
    assert_eq!(&*v1, [1, 2, 3, 4]);
    assert_eq!(&*v3, [1, 2, 3, 4]);
}

#[test]
pub fn upgrade_after_orig_and_clone_dropped_should_fail() {
    let v1 = rc_vec![1, 2, 3, 4];
    let w = v1.downgrade();
    let v2 = v1.clone();
    mem::drop(v1);
    mem::drop(v2);
    assert_eq!(w.upgrade(), None);
}
