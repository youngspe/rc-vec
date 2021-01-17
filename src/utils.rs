/// Use with care -- only if you're certain the reference is valid for `'b`
pub unsafe fn coerce_lifetime<'a, 'b: 'a, T: ?Sized>(orig: &'a T) -> &'b T {
    &*(orig as *const T)
}
/// Use with care -- only if you're certain the reference is valid and unique for `'b`
pub unsafe fn coerce_lifetime_mut<'a, 'b: 'a, T: ?Sized>(orig: &'a mut T) -> &'b mut T {
    &mut *(orig as *const T as *mut T)
}
