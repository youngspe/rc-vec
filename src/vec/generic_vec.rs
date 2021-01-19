use crate::base::vec_ref::{VecMut, VecRef};
use crate::base::{BaseRcVec, VecType};
use core::cmp::Ordering;
use core::fmt;
use core::ops::{Deref, DerefMut};
use core::ops::{Index, IndexMut};
use core::slice::SliceIndex;
use header_slice::HeaderVec;

pub struct GenericVec<V: VecType, T> {
    pub(super) base: BaseRcVec<V, T>,
}

impl<V: VecType, T> GenericVec<V, T> {
    pub fn new() -> Self {
        Self {
            base: BaseRcVec::from_vec(HeaderVec::default()),
        }
    }

    pub fn with_capacity(cap: usize) -> Self {
        Self {
            base: BaseRcVec::from_vec(HeaderVec::with_capacity(Default::default(), cap)),
        }
    }
}

impl<V: VecType, T: Clone> GenericVec<V, T> {
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

impl<V: VecType, T: Copy> GenericVec<V, T> {
    pub fn copy_from_slice(src: &[T]) -> Self {
        Self {
            base: BaseRcVec::from_vec(HeaderVec::copy_from_slice(Default::default(), src)),
        }
    }

    pub fn extend_from_slice(&mut self, src: &[T]) {
        self.base.try_make_vec_mut().unwrap().extend_from_slice(src);
    }
}

impl<V: VecType, T> Deref for GenericVec<V, T> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        VecRef::get_body(self.base.try_vec_ref().unwrap())
    }
}

impl<V: VecType, T: Clone> DerefMut for GenericVec<V, T> {
    fn deref_mut(&mut self) -> &mut [T] {
        VecMut::get_body_mut(self.base.try_make_vec_mut().unwrap())
    }
}

impl<V: VecType, T, S: SliceIndex<[T]>> Index<S> for GenericVec<V, T> {
    type Output = S::Output;
    fn index(&self, i: S) -> &Self::Output {
        VecRef::get_body(self.base.try_vec_ref().unwrap()).index(i)
    }
}

impl<V: VecType, T: Clone, S: SliceIndex<[T]>> IndexMut<S> for GenericVec<V, T> {
    fn index_mut(&mut self, i: S) -> &mut Self::Output {
        VecMut::get_body_mut(self.base.try_make_vec_mut().unwrap()).index_mut(i)
    }
}

impl<V: VecType, T> Clone for GenericVec<V, T> {
    fn clone(&self) -> Self {
        Self {
            base: self.base.clone(),
        }
    }
}

impl<V: VecType, T> Default for GenericVec<V, T> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone)]
pub struct IntoIter<V: VecType, T>(header_slice::vec::IntoValuesIter<V::Counter, T>);

impl<V: VecType, T> Iterator for IntoIter<V, T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        self.0.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<V: VecType, T> ExactSizeIterator for IntoIter<V, T> {}

impl<V: VecType, T: Clone> IntoIterator for GenericVec<V, T> {
    type Item = T;
    type IntoIter = IntoIter<V, T>;
    fn into_iter(self) -> Self::IntoIter {
        let vec = match self.base.try_into_vec() {
            Ok(v) => v,
            Err(_) => unreachable!("somehow this StrongVec couldn't be made unique"),
        };
        IntoIter(vec.into_values())
    }
}

impl<'a, V: VecType, T> IntoIterator for &'a GenericVec<V, T> {
    type Item = &'a T;
    type IntoIter = core::slice::Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        (*self).deref().into_iter()
    }
}

impl<'a, V: VecType, T: Clone> IntoIterator for &'a mut GenericVec<V, T> {
    type Item = &'a mut T;
    type IntoIter = core::slice::IterMut<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        (*self).deref_mut().into_iter()
    }
}

impl<V: VecType, T> core::iter::FromIterator<T> for GenericVec<V, T> {
    fn from_iter<I: IntoIterator<Item = T>>(it: I) -> Self {
        Self {
            base: BaseRcVec::from_vec(it.into_iter().collect()),
        }
    }
}

impl<V: VecType, T: fmt::Debug> fmt::Debug for GenericVec<V, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let slice: &[T] = self;
        fmt::Debug::fmt(slice, f)
    }
}

impl<V: VecType, T: PartialEq> PartialEq for GenericVec<V, T> {
    fn eq(&self, rhs: &Self) -> bool {
        let s1: &[T] = self;
        let s2: &[T] = rhs;
        s1 == s2
    }
}

impl<V: VecType, T: Eq> Eq for GenericVec<V, T> {}

impl<V: VecType, T: PartialOrd> PartialOrd for GenericVec<V, T> {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        let s1: &[T] = self;
        let s2: &[T] = rhs;
        s1.partial_cmp(s2)
    }
}

impl<V: VecType, T: Ord> Ord for GenericVec<V, T> {
    fn cmp(&self, rhs: &Self) -> Ordering {
        let s1: &[T] = self;
        let s2: &[T] = rhs;
        s1.cmp(s2)
    }
}
