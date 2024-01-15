use std::ffi::CStr;

use ckia_sys as sys;
use sys::*;

pub mod bitmap;
pub mod canvas;
pub mod color;
pub mod data;
pub mod filter;
pub mod font;
pub mod geometry;
pub mod gr_context;
pub mod image;
pub mod image_info;
pub mod matrix;
pub mod paint;
pub mod path;
pub mod path_effect;
pub mod pixmap;
pub mod region;
pub mod rrect;
pub mod shader;
pub mod stream;
pub mod string;
pub use color::*;
pub use geometry::*;
pub type PngFilterFlags = sk_pngencoder_filterflags_t;

pub(crate) trait SkiaPointer {
    type Opaque;
    fn as_ptr(&self) -> *const Self::Opaque;
    fn as_ptr_mut(&mut self) -> *mut Self::Opaque;
}

pub(crate) unsafe trait VirtualRefCounted: SkiaPointer {
    fn as_vref_ptr(&self) -> *const sk_refcnt_t;
    fn as_vref_ptr_mut(&self) -> *mut sk_refcnt_t;
    fn is_unique(&self) -> bool {
        unsafe { sk_refcnt_unique(self.as_vref_ptr()) }
    }
    fn get_ref_count(&self) -> i32 {
        unsafe { sk_refcnt_get_ref_count(self.as_vref_ptr()) }
    }
    fn safe_ref(&mut self) {
        unsafe { sk_refcnt_safe_ref(self.as_vref_ptr_mut()) }
    }
    fn safe_unref(&mut self) {
        unsafe { sk_refcnt_safe_unref(self.as_vref_ptr_mut()) }
    }
}
pub(crate) unsafe trait NotVirtualRefCounted: SkiaPointer {
    fn as_nvref_ptr(&self) -> *const sk_nvrefcnt_t;
    fn as_nvref_ptr_mut(&self) -> *mut sk_nvrefcnt_t;
    fn is_unique(&self) -> bool {
        unsafe { sk_nvrefcnt_unique(self.as_nvref_ptr()) }
    }
    fn get_ref_count(&self) -> i32 {
        unsafe { sk_nvrefcnt_get_ref_count(self.as_nvref_ptr()) }
    }
    fn safe_ref(&mut self) {
        unsafe { sk_nvrefcnt_safe_ref(self.as_nvref_ptr_mut()) }
    }
    fn safe_unref(&mut self) {
        unsafe { sk_nvrefcnt_safe_unref(self.as_nvref_ptr_mut()) }
    }
}
pub fn colortype_get_default_8888() -> ColorType {
    unsafe { sk_colortype_get_default_8888() }
}
/// Empty struct to wrap skia version related functions
pub struct SkiaVersion;
impl SkiaVersion {
    pub fn get_milestone() -> i32 {
        unsafe { sk_version_get_milestone() }
    }
    pub fn get_increment() -> i32 {
        unsafe { sk_version_get_increment() }
    }
    pub fn get_string() -> &'static str {
        unsafe {
            let ptr = sk_version_get_string();
            assert!(!ptr.is_null());
            CStr::from_ptr(ptr as _)
                .to_str()
                .expect("failed to parse skia version string")
        }
    }
}
// temporary hack
#[link(name = "dl")]
#[link(name = "pthread")]
#[link(name = "fontconfig")]
#[link(name = "GL")]
#[link(name = "stdc++")]
#[link(name = "freetype")]
extern "C" {}
#[allow(unused)]
macro_rules! pod_struct {
    ($svis: vis $name: ident, $opaque: ident {
        $($vis: vis $field: ident: $fty: ty ,)+
    }) => {
        #[derive(Debug, Copy, Clone)]
        #[repr(transparent)]
        $svis struct $name(pub(crate) $opaque);

        impl AsRef<$opaque> for $name {
            fn as_ref(&self) -> &$opaque {
                &self.0
            }
        }
        impl AsMut<$opaque> for $name {
            fn as_mut(&mut self) -> &mut $opaque {
                &mut self.0
            }
        }
        impl Borrow<$opaque> for $name {
            fn borrow(&self) -> &$opaque {
                &self.0
            }
        }
        impl BorrowMut<$opaque> for $name {
            fn borrow_mut(&mut self) -> &mut $opaque {
                &mut self.0
            }
        }
        impl $name {
            pub(crate) fn as_ptr(&self) -> *const $opaque {
                &self.0 as _
            }
            pub(crate) fn as_ptr_mut(&mut self) -> * mut $opaque {
                &mut self.0 as _
            }
            $(
                $vis fn get_$field(&self) -> $fty {
                    self.0.$field
                }
                $vis fn set_$field(&mut self, $field: $fty) {
                    self.0.$field = $field;
                }
            )+
        }
    };
}
#[allow(unused)]
pub(crate) use pod_struct;
/// A convenience wrapper that implements some repetitive code for skia objects which are ref counted
/// usage: `opaque_shared!(StructName, struct_name, sk_struct_name_ref, sk_struct_name_unref);`
/// StructName is just the rust struct name (wrapper)
/// struct_name is the opaque C struct name (wrapped)
/// struct_name_ref is the extern "C" fn which increases ref count
/// struct_name_unref is the extern "C" fn which decrease ref count
///
/// We will basically create a #[repr(transparent)] struct that wraps the mut ptr to C struct.
/// We will implement clone and drop using ref and unref
/// Finally, we will implement functions
/// 1. `into_owned_ptr`: consumes struct and returns the inner pointer without touching the ref count. caller is responsible for decrementing the ref count to destroy it and then never using it
/// 2. `from_owned_ptr`: takes a ptr and adopts it without changing the refcount. caller needs to make sure to NOT call unref, and assume the new struct has the ownership
/// 3. `from_borrowed_ptr`: takes a ptr, calls ref and then creates a struct with it. caller still has the ownership of the original pointer
///
/// Both the above fns will only return an owned struct, if the ptr is not null. if its null, then they return None.
macro_rules! opaque_shared {
    ($name: ident, $opaque: ident,$unref: ident $(, $ref: ident)? ) => {
        #[derive(Debug)]
        #[repr(transparent)]
        pub struct $name {
            pub(crate) inner: *mut $opaque,
        }
        impl SkiaPointer for 
        $(
        impl Clone for $name {
            fn clone(&self) -> Self {
                unsafe { $ref(self.inner) };
                Self { inner: self.inner }
            }
        })?
        impl Drop for $name {
            fn drop(&mut self) {
                unsafe {
                    $unref(self.inner);
                }
            }
        }
        #[allow(unused)]
        impl $name {
            
            /// consumes struct and returns a ptr that has ownership.
            /// # Safety
            /// caller needs to call unref after being done with it.
            pub(crate) unsafe fn into_owned_ptr(self) -> *mut $opaque {
                let inner = self.inner;
                std::mem::forget(self);
                inner
            }
            /// takes a pointer and assumes ownership of it. panics if null ptr. For a non-panic version. use [Self::try_from_owned_ptr]
            /// # Safety
            /// the struct assumes ownership, so caller shouldn't use it after that.
            /// also make sure that you haven't accidentally called unref on it in the past.
            pub(crate) unsafe fn from_owned_ptr(ptr: *mut $opaque) -> Self {
                assert!(!ptr.is_null());
                Self { inner: ptr }
            }
            /// takes a pointer and assumes ownership of it. returns None if nullptr.
            /// # Safety
            /// the struct assumes ownership, so caller shouldn't use it after that.
            /// also make sure that you haven't accidentally called unref on it in the past.
            pub(crate) unsafe fn try_from_owned_ptr(ptr: *mut $opaque) -> Option<Self> {
                if ptr.is_null() {
                    None
                } else {
                    Some(Self { inner: ptr })
                }
            }
            $(
            /// takes a ptr and increments ref count. Then assumes ownership of it. returns None if nullptr.
            /// # Safety
            /// The original pointer's ownership still resides with the caller, and they must make sure to call unref on it after being done with it.
            pub(crate) unsafe fn from_borrowed_ptr(ptr: *mut $opaque) -> Option<Self> {
                if ptr.is_null() {
                    None
                } else {
                    unsafe { $ref(ptr) };
                    Some(Self { inner: ptr })
                }
            }
            )*
        }
    };
}
/// A convenience wrapper that implements some repetitive code for skia objects which are uniquely owned (eg: allocated on heap using new and delete)
/// usage: `opaque_shared!(StructName, struct_name,  sk_struct_name_delete);`
/// StructName is just the rust struct name (wrapper)
/// struct_name is the opaque C struct name (wrapped)
/// struct_name_delete is the extern "C" fn which deletes the ptr
///
/// We will basically create a #[repr(transparent)] struct that wraps the mut ptr to C struct.
/// We will implement clone and drop using ref and unref
/// Finally, we will implement functions
/// 1. `into_owned_ptr`: consumes struct and returns the inner pointer without touching the ref count. caller is responsible for decrementing the ref count to destroy it and then never using it
/// 2. `from_owned_ptr`: takes a ptr and adopts it without changing the refcount. caller needs to make sure to NOT call unref, and assume the new struct has the ownership
/// 3. `from_borrowed_ptr`: takes a ptr, calls ref and then creates a struct with it. caller still has the ownership of the original pointer
///
/// Both the above fns will only return an owned struct, if the ptr is not null. if its null, then they return None.
macro_rules! opaque_unique {
    ($name: ident, $opaque: ident, $del: ident) => {
        #[derive(Debug)]
        #[repr(transparent)]
        pub struct $name {
            pub(crate) inner: *mut $opaque,
        }
        impl Drop for $name {
            fn drop(&mut self) {
                unsafe {
                    $del(self.inner);
                }
            }
        }
        #[allow(unused)]
        impl $name {
            pub(crate) fn as_ptr(&self) -> *const $opaque {
                self.inner as _
            }
            pub(crate) fn as_ptr_mut(&mut self) -> *mut $opaque {
                self.inner
            }
            /// consumes struct and returns a ptr that has ownership.
            /// # Safety
            /// caller needs to call unref after being done with it.

            pub(crate) unsafe fn into_owned_ptr(self) -> *mut $opaque {
                let inner = self.inner;
                std::mem::forget(self);
                inner
            }
            /// takes a pointer and assumes ownership of it. panics if null ptr. For a non-panic version. use [Self::try_from_owned_ptr]
            /// # Safety
            /// the struct assumes ownership, so caller shouldn't use it after that.
            /// also make sure that you haven't accidentally called unref on it in the past.
            pub(crate) unsafe fn from_owned_ptr(ptr: *mut $opaque) -> Self {
                assert!(!ptr.is_null());
                Self { inner: ptr }
            }
            /// takes a pointer and assumes ownership of it. returns None if nullptr.
            /// # Safety
            /// the struct assumes ownership, so caller shouldn't use it after that.
            /// also make sure that you haven't accidentally called unref on it in the past.
            pub(crate) unsafe fn try_from_owned_ptr(ptr: *mut $opaque) -> Option<Self> {
                if ptr.is_null() {
                    None
                } else {
                    Some(Self { inner: ptr })
                }
            }
        }
    };
}
pub(crate) use opaque_shared;

pub(crate) use opaque_unique;
