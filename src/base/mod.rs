pub mod counter;
pub mod vec_ref;

use core::convert::Infallible;
use core::marker::PhantomData;
use counter::Counter;
use header_slice::HeaderVec;
use vec_ref::{HeaderVecParts, VecMut, VecRef};

pub struct BaseRcVec<H: Counter, V: VecType<H>, T> {
    parts: HeaderVecParts<H, T>,
    _v: PhantomData<V>,
}

pub unsafe trait VecType<C: Counter> {
    unsafe fn incr(counter: &C);
    unsafe fn decr(counter: &C);
    fn can_take(counter: &C) -> bool;
    fn can_get_ref(counter: &C) -> bool;
    fn can_get_mut(counter: &C) -> bool;
    fn should_drop_entire_vector(counter: &C) -> bool;
    fn should_drop_contents(counter: &C) -> bool;
    fn can_create(counter: &C) -> bool;
}

pub struct StrongType(Infallible);

unsafe impl<C: Counter> VecType<C> for StrongType {
    unsafe fn incr(counter: &C) {
        counter.incr_strong();
    }
    unsafe fn decr(counter: &C) {
        counter.decr_strong();
    }
    fn can_take(counter: &C) -> bool {
        counter.unique_strong()
    }
    fn can_get_ref(counter: &C) -> bool {
        counter.valid_strong()
    }
    fn can_get_mut(counter: &C) -> bool {
        counter.unique_weak()
    }
    fn should_drop_entire_vector(counter: &C) -> bool {
        !counter.valid_weak()
    }
    fn should_drop_contents(counter: &C) -> bool {
        !counter.valid_strong()
    }
    fn can_create(counter: &C) -> bool {
        counter.valid_strong()
    }
}

pub struct WeakType(Infallible);

unsafe impl<C: Counter> VecType<C> for WeakType {
    unsafe fn incr(counter: &C) {
        counter.incr_weak();
    }
    unsafe fn decr(counter: &C) {
        counter.decr_weak();
    }
    fn can_take(_: &C) -> bool {
        false
    }
    fn can_get_ref(_: &C) -> bool {
        false
    }
    fn can_get_mut(_: &C) -> bool {
        false
    }
    fn should_drop_entire_vector(counter: &C) -> bool {
        !counter.valid_weak()
    }
    fn should_drop_contents(_: &C) -> bool {
        false
    }
    fn can_create(_: &C) -> bool {
        true
    }
}

impl<H: Counter, V: VecType<H>, T> BaseRcVec<H, V, T> {
    unsafe fn from_parts(parts: HeaderVecParts<H, T>) -> Self {
        let this = Self {
            parts,
            _v: PhantomData,
        };
        V::incr(this.counter());
        this
    }

    pub fn from_vec(mut src: HeaderVec<H, T>) -> Self {
        src.head = H::default();
        unsafe { Self::from_parts(HeaderVecParts::from_vec(src)) }
    }

    unsafe fn unsafe_vec_ref(&self) -> VecRef<H, T> {
        VecRef::new(&self.parts)
    }

    unsafe fn unsafe_vec_mut(&mut self) -> VecMut<H, T> {
        let vr = VecMut::new(&mut self.parts);
        debug_assert!(vr.head.valid_strong());
        vr
    }

    fn counter(&self) -> &H {
        // SAFETY: at least the counter must exist if this instance exists
        unsafe { VecRef::get_head(&self.unsafe_vec_ref()) }
    }

    pub fn try_vec_ref(&self) -> Option<VecRef<H, T>> {
        let vr = unsafe { self.unsafe_vec_ref() };
        if V::can_get_ref(&vr.head) {
            Some(vr)
        } else {
            None
        }
    }

    pub fn _try_vec_mut(&mut self) -> Option<VecMut<H, T>> {
        if V::can_get_mut(self.counter()) {
            Some(unsafe { self.unsafe_vec_mut() })
        } else {
            None
        }
    }

    pub fn try_convert<V2: VecType<H>>(&self) -> Option<BaseRcVec<H, V2, T>> {
        if V2::can_create(self.counter()) {
            Some(unsafe { BaseRcVec::from_parts(self.parts) })
        } else {
            None
        }
    }
}

impl<H: Counter, V: VecType<H>, T: Clone> BaseRcVec<H, V, T> {
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

    pub fn try_make_vec_mut(&mut self) -> Option<VecMut<H, T>> {
        if !self.try_make_unique() {
            return None;
        }
        Some(unsafe { self.unsafe_vec_mut() })
    }

    pub fn try_into_vec(mut self) -> Result<HeaderVec<H, T>, Self> {
        if !self.try_make_unique() {
            return Err(self);
        }

        Ok(unsafe { self.parts.into_vec() })
    }
}

impl<H: Counter, V: VecType<H>, T> Drop for BaseRcVec<H, V, T> {
    fn drop(&mut self) {
        let counter = self.counter();
        unsafe {
            V::decr(counter);

            if V::should_drop_entire_vector(counter) {
                VecMut::drop_entire_vector(self.unsafe_vec_mut())
            } else if V::should_drop_contents(counter) {
                self.unsafe_vec_mut().clear_in_place()
            }
        }
    }
}

impl<H: Counter, V: VecType<H>, T> Clone for BaseRcVec<H, V, T> {
    fn clone(&self) -> Self {
        unsafe { Self::from_parts(self.parts) }
    }
}
