pub mod vec_ref;

use header_slice::HeaderVec;
use vec_ref::{HeaderVecParts, VecMut, VecRef};

pub trait Counter: Default + Clone {}

pub struct BaseRcVec<V: VecType, T> {
    parts: HeaderVecParts<V::Counter, T>,
}

pub unsafe trait VecType {
    type Counter: Counter;
    fn incr(counter: &Self::Counter);
    fn decr(counter: &Self::Counter);
    fn can_take(counter: &Self::Counter) -> bool;
    fn can_get_ref(counter: &Self::Counter) -> bool;
    fn can_get_mut(counter: &Self::Counter) -> bool;
    fn should_drop_entire_vector(counter: &Self::Counter) -> bool;
    fn should_drop_contents(counter: &Self::Counter) -> bool;
    fn can_create(counter: &Self::Counter) -> bool;
}

impl<V: VecType, T> BaseRcVec<V, T> {
    unsafe fn from_parts(parts: HeaderVecParts<V::Counter, T>) -> Self {
        let this = Self { parts };
        V::incr(this.counter());
        this
    }

    pub fn from_vec(mut src: HeaderVec<V::Counter, T>) -> Self {
        src.head = Default::default();
        unsafe { Self::from_parts(HeaderVecParts::from_vec(src)) }
    }

    unsafe fn unsafe_vec_ref(&self) -> VecRef<V::Counter, T> {
        VecRef::new(&self.parts)
    }

    unsafe fn unsafe_vec_mut(&mut self) -> VecMut<V::Counter, T> {
        let vr = VecMut::new(&mut self.parts);
        vr
    }

    fn counter(&self) -> &V::Counter {
        // SAFETY: at least the counter must exist if this instance exists
        unsafe { VecRef::get_head(&self.unsafe_vec_ref()) }
    }

    pub fn try_vec_ref(&self) -> Option<VecRef<V::Counter, T>> {
        let vr = unsafe { self.unsafe_vec_ref() };
        if V::can_get_ref(&vr.head) {
            Some(vr)
        } else {
            None
        }
    }

    pub fn _try_vec_mut(&mut self) -> Option<VecMut<V::Counter, T>> {
        if V::can_get_mut(self.counter()) {
            Some(unsafe { self.unsafe_vec_mut() })
        } else {
            None
        }
    }

    pub fn try_convert<V2: VecType<Counter = V::Counter>>(&self) -> Option<BaseRcVec<V2, T>> {
        if V2::can_create(self.counter()) {
            Some(unsafe { BaseRcVec::from_parts(self.parts) })
        } else {
            None
        }
    }
}

impl<V: VecType, T: Clone> BaseRcVec<V, T> {
    pub fn try_deep_clone(&self) -> Option<Self> {
        let new_vec = self.try_vec_ref()?.clone();
        Some(Self::from_vec(new_vec))
    }

    fn try_make_unique(&mut self) -> bool {
        if !V::can_get_mut(self.counter()) {
            *self = match self.try_deep_clone() {
                Some(x) => x,
                None => return false,
            }
        }
        return true;
    }

    pub fn try_make_vec_mut(&mut self) -> Option<VecMut<V::Counter, T>> {
        if !self.try_make_unique() {
            return None;
        }
        Some(unsafe { self.unsafe_vec_mut() })
    }

    pub fn try_into_vec(mut self) -> Result<HeaderVec<V::Counter, T>, Self> {
        if !self.try_make_unique() {
            return Err(self);
        }

        Ok(unsafe { self.parts.into_vec() })
    }
}

impl<V: VecType, T> Drop for BaseRcVec<V, T> {
    fn drop(&mut self) {
        unsafe {
            V::decr(self.counter());

            if V::should_drop_contents(self.counter()) {
                self.unsafe_vec_mut().clear_in_place();
            }
            if V::should_drop_entire_vector(self.counter()) {
                VecMut::dealloc_vector(self.unsafe_vec_mut())
            }
        }
    }
}

impl<V: VecType, T> Clone for BaseRcVec<V, T> {
    fn clone(&self) -> Self {
        unsafe { Self::from_parts(self.parts) }
    }
}
