use crate::base::counter::StrongWeakCounter;
use crate::base::vec_ref::{VecMut, VecRef};
use crate::base::{BaseRcVec, StrongType, WeakType};
use core::ops::{Deref, DerefMut};
use core::ops::{Index, IndexMut};
use core::slice::SliceIndex;
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

impl<T: Copy> RcVec<T> {
    pub fn copy_from_slice(src: &[T]) -> Self {
        Self {
            base: BaseRcVec::from_vec(HeaderVec::copy_from_slice(Default::default(), src)),
        }
    }

    pub fn extend_from_slice(&mut self, src: &[T]) {
        self.base.try_make_vec_mut().unwrap().extend_from_slice(src);
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

impl<T, S: SliceIndex<[T]>> Index<S> for RcVec<T> {
    type Output = S::Output;
    fn index(&self, i: S) -> &Self::Output {
        VecRef::get_body(self.base.try_vec_ref().unwrap()).index(i)
    }
}

impl<T: Clone, S: SliceIndex<[T]>> IndexMut<S> for RcVec<T> {
    fn index_mut(&mut self, i: S) -> &mut Self::Output {
        VecMut::get_body_mut(self.base.try_make_vec_mut().unwrap()).index_mut(i)
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

#[derive(Clone)]
pub struct IntoIter<T>(header_slice::vec::IntoValuesIter<StrongWeakCounter, T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        self.0.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<T> ExactSizeIterator for IntoIter<T> {}

impl<T: Clone> IntoIterator for RcVec<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        let vec = match self.base.try_into_vec() {
            Ok(v) => v,
            Err(_) => unreachable!("somehow this RcVec couldn't be made unique"),
        };
        IntoIter(vec.into_values())
    }
}

impl<'a, T> IntoIterator for &'a RcVec<T> {
    type Item = &'a T;
    type IntoIter = core::slice::Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        (*self).deref().into_iter()
    }
}

impl<'a, T: Clone> IntoIterator for &'a mut RcVec<T> {
    type Item = &'a mut T;
    type IntoIter = core::slice::IterMut<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        (*self).deref_mut().into_iter()
    }
}

impl<T> core::iter::FromIterator<T> for RcVec<T> {
    fn from_iter<I: IntoIterator<Item = T>>(it: I) -> Self {
        Self {
            base: BaseRcVec::from_vec(it.into_iter().collect()),
        }
    }
}
