use core::marker::PhantomData;
use core::mem;
use core::mem::MaybeUninit;
use core::ops::{Deref, DerefMut};
use core::ptr::NonNull;
use header_slice::pair::Pair;
use header_slice::HeaderVec;

#[derive(Debug)]
pub struct HeaderVecParts<H, T> {
    pub ptr: NonNull<Pair<H, MaybeUninit<T>>>,
    pub len: usize,
    pub cap: usize,
}

impl<H, T> HeaderVecParts<H, T> {
    pub fn from_vec(src: HeaderVec<H, T>) -> Self {
        let (ptr, len, cap) = src.into_raw_parts();
        Self { ptr, len, cap }
    }
    pub unsafe fn into_vec(self) -> HeaderVec<H, T> {
        let Self { ptr, len, cap } = self;
        HeaderVec::from_raw_parts(ptr, len, cap)
    }
}

impl<H, T> Clone for HeaderVecParts<H, T> {
    fn clone(&self) -> Self {
        Self {
            ptr: self.ptr,
            len: self.len,
            cap: self.cap,
        }
    }
}
impl<H, T> Copy for HeaderVecParts<H, T> {}

pub struct VecRef<'a, H: 'a, T: 'a> {
    inner: MaybeUninit<HeaderVec<H, T>>,
    // make sure this struct can't outlive the data it's borrowing
    _lt: PhantomData<&'a ()>,
}

pub struct VecMut<'a, H: 'a, T: 'a> {
    src: &'a mut HeaderVecParts<H, T>,
    inner: MaybeUninit<HeaderVec<H, T>>,
}

impl<'a, H, T> VecRef<'a, H, T> {
    /// SAFETY: Promise that no mutable references to the vector will be created while this instance
    /// exists.
    pub unsafe fn new(src: &'a HeaderVecParts<H, T>) -> Self {
        let inner = MaybeUninit::new(src.into_vec());
        Self {
            inner,
            _lt: PhantomData,
        }
    }

    pub fn get_body(this: Self) -> &'a [T] {
        // SAFETY: The contract when creating this struct promises that there are no mutable
        // references to the vector and it will not be dropped for the lifetime 'a
        let ptr = this.inner.as_ptr();
        unsafe { &(*ptr).body }
    }

    pub(super) fn get_head(this: &Self) -> &'a H {
        // SAFETY: The contract when creating this struct promises that there are no mutable
        // references to the vector and it will not be dropped for the lifetime 'a
        let ptr = this.inner.as_ptr();
        unsafe { &(*ptr).head }
    }
}

impl<'a, H, T> VecMut<'a, H, T> {
    /// SAFETY: Promise that no other references to the vector will be created while this instance
    /// exists.
    pub unsafe fn new(src: &'a mut HeaderVecParts<H, T>) -> Self {
        let inner = MaybeUninit::new(src.into_vec());
        VecMut { src, inner }
    }

    /// Deallocates the vector referenced by this struct without dropping its contents.
    /// Make sure the vector will not be reconstructed from parts ever again.
    pub unsafe fn dealloc_vector(this: Self) {
        let inner = mem::transmute_copy::<_, HeaderVec<H, T>>(&this.inner);
        inner.dealloc_without_dropping();
    }

    pub fn get_body_mut(mut this: Self) -> &'a mut [T] {
        // SAFETY: The contract when creating this struct promises that there are no other
        // references to the data and it will not be dropped for the lifetime 'a
        let ptr = this.inner.as_mut_ptr();
        unsafe { &mut (*ptr).body }
    }
}

impl<'a, H, T> Drop for VecMut<'a, H, T> {
    fn drop(&mut self) {
        // Save any changes that were made
        let (ptr, len, cap) = self.as_raw_parts();
        *self.src = HeaderVecParts { ptr, len, cap };
    }
}

impl<'a, H, T> Deref for VecRef<'a, H, T> {
    type Target = HeaderVec<H, T>;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.inner.as_ptr() }
    }
}

impl<'a, H, T> Deref for VecMut<'a, H, T> {
    type Target = HeaderVec<H, T>;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.inner.as_ptr() }
    }
}

impl<'a, H, T> DerefMut for VecMut<'a, H, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.inner.as_mut_ptr() }
    }
}
