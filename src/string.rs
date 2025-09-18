use std::ffi::CStr;

use crate::{bindings::*, OwnerShip, SkiaPtr};

crate::skia_wrapper!(unique, SkiaString, sk_string_t, sk_string_destructor);

impl<O: OwnerShip> SkiaStringGen<O> {
    pub fn len(&self) -> usize {
        unsafe { sk_string_get_size(self.as_ptr()) }
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn get_cstr(&self) -> &CStr {
        unsafe { CStr::from_ptr(sk_string_get_c_str(self.as_ptr())) }
    }
    pub fn new_empty() -> SkiaString {
        unsafe { Self::from_owned_ptr(sk_string_new_empty()) }
    }
    /// since skia deals with cstrings, maybe we should check that there is no null byte inside this string?
    /// It won't cause any memory safety issues, as skia will add a null byte at the end of this string anyway
    pub fn new_with_copy(text: &str) -> SkiaString {
        unsafe { Self::from_owned_ptr(sk_string_new_with_copy(text.as_ptr() as _, text.len())) }
    }
}
