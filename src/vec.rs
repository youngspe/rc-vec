use crate::base::counter::StrongWeakCounter;
use crate::base::vec_ref::{VecMut, VecRef};
use crate::base::{BaseRcVec, StrongType, WeakType};
use core::cmp::Ordering;
use core::fmt;
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

    pub unsafe fn copy_from_ptr_unsafe(ptr: *mut T, len: usize) -> Self {
        let v = HeaderVec::copy_from_ptr_unsafe(Default::default(), ptr, len);
        Self {
            base: BaseRcVec::from_vec(v),
        }
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

impl<T: fmt::Debug> fmt::Debug for RcVec<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let slice: &[T] = self;
        fmt::Debug::fmt(slice, f)
    }
}

impl<T: PartialEq> PartialEq for RcVec<T> {
    fn eq(&self, rhs: &Self) -> bool {
        let s1: &[T] = self;
        let s2: &[T] = rhs;
        s1 == s2
    }
}

impl<T: Eq> Eq for RcVec<T> {}

impl<T: PartialOrd> PartialOrd for RcVec<T> {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        let s1: &[T] = self;
        let s2: &[T] = rhs;
        s1.partial_cmp(s2)
    }
}

impl<T: Ord> Ord for RcVec<T> {
    fn cmp(&self, rhs: &Self) -> Ordering {
        let s1: &[T] = self;
        let s2: &[T] = rhs;
        s1.cmp(s2)
    }
}
