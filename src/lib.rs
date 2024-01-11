use ckia_sys as sys;
use sys::*;

pub mod bitmap;
pub mod canvas;
pub mod color;
pub mod filter;
pub mod geometry;
pub mod image;
pub mod image_info;
pub mod paint;
pub mod pixmap;
pub mod shader;
pub mod stream;

pub use color::*;
pub use geometry::*;
pub type PngFilterFlags = sk_pngencoder_filterflags_t;
// temporary hack
#[link(name = "dl")]
#[link(name = "pthread")]
#[link(name = "fontconfig")]
#[link(name = "GL")]
#[link(name = "stdc++")]
#[link(name = "freetype")]
extern "C" {}

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
            /// # Unsafe
            /// caller needs to call unref after being done with it.

            pub(crate) unsafe fn into_owned_ptr(self) -> *mut $opaque {
                let inner = self.inner;
                std::mem::forget(self);
                inner
            }
            /// takes a pointer and assumes ownership of it. returns None if nullptr.
            /// # Unsafe
            /// the struct assumes ownership, so caller shouldn't use it after that.
            /// also make sure that you haven't accidentally called unref on it in the past.
            pub(crate) unsafe fn from_owned_ptr(ptr: *mut $opaque) -> Option<Self> {
                if ptr.is_null() {
                    None
                } else {
                    Some(Self { inner: ptr })
                }
            }
            $(
            /// takes a ptr and increments ref count. Then assumes ownership of it. returns None if nullptr.
            /// # Unsafe
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
            /// consumes struct and returns a ptr that has ownership.
            /// # Unsafe
            /// caller needs to call unref after being done with it.

            pub(crate) unsafe fn into_owned_ptr(self) -> *mut $opaque {
                let inner = self.inner;
                std::mem::forget(self);
                inner
            }
            /// takes a pointer and assumes ownership of it. returns None if nullptr.
            /// # Unsafe
            /// the struct assumes ownership, so caller shouldn't use it after that.
            /// also make sure that you haven't accidentally called unref on it in the past.
            pub(crate) unsafe fn from_owned_ptr(ptr: *mut $opaque) -> Option<Self> {
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
