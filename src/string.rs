use std::ffi::CStr;

use ckia_sys::*;

crate::opaque_unique!(SkiaString, sk_string_t, sk_string_destructor);

impl SkiaString {
    pub fn new_empty() -> Self {
        unsafe { Self::from_owned_ptr(sk_string_new_empty()) }
    }
    /// since skia deals with cstrings, maybe we should check that there is no null byte inside this string?
    /// It won't cause any memory safety issues, as skia will add a null byte at the end of this string anyway
    pub fn new_with_copy(text: &str) -> Self {
        unsafe { Self::from_owned_ptr(sk_string_new_with_copy(text.as_ptr() as _, text.len())) }
    }
    pub fn len(&self) -> usize {
        unsafe { sk_string_get_size(self.inner) }
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn get_cstr(&self) -> &CStr {
        unsafe { CStr::from_ptr(sk_string_get_c_str(self.inner)) }
    }
}
