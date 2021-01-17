use crate::base::counter::StrongWeakCounter;
use crate::base::vec_ref::{VecMut, VecRef};
use crate::base::{BaseRcVec, StrongType, WeakType};
use core::ops::{Deref, DerefMut};
use header_slice::HeaderVec;

pub struct RcVec<T> {
    base: BaseRcVec<StrongWeakCounter, StrongType, T>,
}

impl<T> RcVec<T> {
    pub fn new() -> Self {
        Self {
            base: BaseRcVec::from_vec(HeaderVec::default()),
        }
    }

    pub fn downgrade(&self) -> WeakVec<T> {
        WeakVec {
            base: self.base.try_convert().unwrap(),
        }
    }
}

impl<T: Clone> RcVec<T> {
    pub fn push(&mut self, val: T) {
        self.base.try_make_vec_mut().unwrap().push(val);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.base.try_make_vec_mut().unwrap().pop()
    }

    pub fn clear(&mut self) {
        self.base.try_make_vec_mut().unwrap().clear();
    }

    pub fn insert(&mut self, index: usize, val: T) {
        self.base.try_make_vec_mut().unwrap().insert(index, val);
    }

    pub fn remove(&mut self, index: usize) -> Option<T> {
        self.base.try_make_vec_mut().unwrap().remove(index)
    }
}

impl<T> Deref for RcVec<T> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        VecRef::get_body(self.base.try_vec_ref().unwrap())
    }
}

impl<T: Clone> DerefMut for RcVec<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        VecMut::get_body_mut(self.base.try_make_vec_mut().unwrap())
    }
}

impl<T> Clone for RcVec<T> {
    fn clone(&self) -> Self {
        Self {
            base: self.base.clone(),
        }
    }
}

impl<T> Default for RcVec<T> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct WeakVec<T> {
    base: BaseRcVec<StrongWeakCounter, WeakType, T>,
}

impl<T> WeakVec<T> {
    pub fn upgrade(&self) -> Option<RcVec<T>> {
        Some(RcVec {
            base: self.base.try_convert()?,
        })
    }
}

impl<T> Clone for WeakVec<T> {
    fn clone(&self) -> Self {
        Self {
            base: self.base.clone(),
        }
    }
}
