use crate::rc_vec;
use alloc::vec::Vec;
use core::cell::RefCell;
use core::mem;

macro_rules! qdrop {
    (struct $name:ident(..);) => {
        #[derive(Clone, Debug)]
        struct $name<'a, T: Clone>(T, &'a RefCell<Vec<T>>);

        impl<'a, T: Clone> Drop for $name<'a, T> {
            fn drop(&mut self) {
                self.1.borrow_mut().push(self.0.clone());
            }
        }
    };
}

#[test]
pub fn single_vec_dropped() {
    qdrop! { struct A(..); }
    let q = RefCell::new(Vec::new());
    let v = rc_vec![A(1, &q), A(2, &q), A(3, &q)];
    mem::drop(v);
    assert_eq!(&*q.borrow(), &[1, 2, 3]);
}

#[test]
pub fn clone_dropped_once() {
    qdrop! { struct A(..); }
    let q = RefCell::new(Vec::new());
    let v1 = rc_vec![A(1, &q), A(2, &q), A(3, &q)];
    let v2 = v1.clone();
    mem::drop(v1);
    assert_eq!(&*q.borrow(), &[]);
    mem::drop(v2);
    assert_eq!(&*q.borrow(), &[1, 2, 3]);
}

#[test]
pub fn clone_dropped_once_with_weak() {
    qdrop! { struct A(..); }
    let q = RefCell::new(Vec::new());
    let v1 = rc_vec![A(1, &q), A(2, &q), A(3, &q)];
    let v2 = v1.clone();
    mem::drop(v1);
    assert_eq!(&*q.borrow(), &[]);
    let w = v2.downgrade();
    mem::drop(v2);
    assert_eq!(&*q.borrow(), &[1, 2, 3]);
    mem::drop(w);
    assert_eq!(&*q.borrow(), &[1, 2, 3]);
}
