use std::ffi::CStr;

pub mod bindings;

pub mod bitmap;
pub mod canvas;
pub mod color;
pub mod data;
pub mod filter;
pub mod font;
pub mod gr_context;
pub mod image;
#[cfg(feature = "mlua")]
pub mod lua;
pub mod matrix;
pub mod paint;
pub mod paragraph;
pub mod path;
pub mod path_effect;
pub mod picture;
pub mod pixmap;
pub mod region;
pub mod rrect;
pub mod shader;
pub mod skottie;
pub mod sksg;
pub mod stream;
pub mod string;
pub mod surface;
pub mod text_blob;
pub mod typeface;
mod types;
use bindings::*;
pub use color::{Color, PMColor};
pub use types::*;
#[cfg(windows)]
#[cfg(not(feature = "disable_embedding_icudtl_dat"))]
#[cfg(feature = "static_linking")]
pub fn init() {
    use std::env;
    static icudtl: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/icudtl.dat"));

    {
        use std::sync::Mutex;

        lazy_static::lazy_static!(
            static ref MUTEX : Mutex<()> = Mutex::new(());
        );

        // Using `Once` does not work for yet unknown reasons.
        // https://github.com/rust-skia/rust-skia/issues/566

        let lock = MUTEX.lock().unwrap();
        assert!(
            unsafe { bindings::sk_icu_set_icudtl_dat(&icudtl[0] as &'static u8 as *const u8 as _) },
            "failed to init icudtl"
        );
        drop(lock);
    }

    // {
    //     use std::fs;

    //     let path = env::current_exe()
    //         .expect("Failed to resolve the current executable's path")
    //         .parent()
    //         .expect("Current executable's parent path does not point to a directory")
    //         .join("icudtl.dat");
    //     if path.exists() {
    //         return;
    //     };
    //     fs::write(path, &icudtl[..])
    //         .expect("Failed to write icudtl.dat into the current executable's directory");
    // }
}

#[cfg(not(windows))]
pub fn init() {}

pub struct SkiaWrapper<T: FfiDrop> {
    inner: *mut T,
}
impl<T: FfiDrop> Drop for SkiaWrapper<T> {
    fn drop(&mut self) {
        unsafe { crate::FfiDrop::ffi_drop(self.inner) }
    }
}
impl<T: FfiDrop> SkiaWrapper<T> {
    pub(crate) fn as_ptr(&self) -> *const T {
        self.inner as _
    }
    pub fn as_ptr_mut(&mut self) -> *mut T {
        self.inner
    }
    /// consumes struct and returns a ptr that has ownership.
    /// # Safety
    /// caller needs to call unref after being done with it
    unsafe fn into_owned_ptr(mut self) -> *mut T {
        let inner = self.as_ptr_mut();
        std::mem::forget(self);
        inner
    }
    /// takes a pointer and assumes ownership of it. panics if null ptr. For a non-panic version. use [Self::try_from_owned_ptr]
    /// # Safety
    /// the struct assumes ownership, so caller shouldn't use it after that.
    /// also make sure that you haven't accidentally called unref on it in the past.
    unsafe fn from_owned_ptr(ptr: *mut T) -> Self {
        assert!(!ptr.is_null());
        Self { inner: ptr }
    }
    /// takes a pointer and assumes ownership of it. returns None if nullptr.
    /// # Safety
    /// the struct assumes ownership, so caller shouldn't use it after that.
    /// also make sure that you haven't accidentally called unref on it in the past.
    unsafe fn try_from_owned_ptr(ptr: *mut T) -> Option<Self> {
        if ptr.is_null() {
            None
        } else {
            Some(Self::from_owned_ptr(ptr))
        }
    }
}
pub unsafe trait FfiDrop {
    unsafe fn ffi_drop(this: *mut Self);
}
pub(crate) trait SkiaOptPtrMut<T> {
    fn or_null_mut(self) -> *mut T;
}
pub(crate) trait SkiaOptPtr<T> {
    fn or_null(&self) -> *const T;
}
impl<T: FfiDrop> SkiaOptPtr<T> for Option<&SkiaWrapper<T>> {
    fn or_null(&self) -> *const T {
        self.map(|s| s.as_ptr()).unwrap_or(std::ptr::null())
    }
}
impl<T: FfiDrop> SkiaOptPtrMut<T> for Option<&mut SkiaWrapper<T>> {
    fn or_null_mut(self) -> *mut T {
        match self {
            Some(m) => m.as_ptr_mut(),
            None => std::ptr::null_mut(),
        }
    }
}

pub unsafe trait VirtualRefCounted {
    fn as_vref_ptr(&self) -> *const sk_refcnt_t;
    fn as_vref_ptr_mut(&mut self) -> *mut sk_refcnt_t;
    fn is_unique(&self) -> bool;

    fn safe_ref(&mut self) -> Self;
    fn safe_unref(self);
}
pub unsafe trait NotVirtualRefCounted {
    fn as_nvref_ptr(&self) -> *const sk_nvrefcnt_t;
    fn as_nvref_ptr_mut(&mut self) -> *mut sk_nvrefcnt_t;
    fn is_unique(&self) -> bool;
    fn safe_ref(&mut self) -> Self;
    fn safe_unref(self);
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

macro_rules! pod_struct {
    ($svis: vis $name: ident, $opaque: ident {
        $($vis: vis $field: ident: $fty: ty ,)+
    }) => {
        /*
        #[derive(Debug, Copy, Clone)]
        #[repr(C)]
        $svis struct $name {
            $($vis $field : $fty,)+
        }

        impl AsRef<$opaque> for $name {
            fn as_ref(&self) -> &$opaque {
                unsafe{std::mem::transmute(self)}
            }
        }
        impl AsMut<$opaque> for $name {
            fn as_mut(&mut self) -> &mut $opaque {
                unsafe {std::mem::transmute(self)}
            }
        }
        impl std::borrow::Borrow<$opaque> for $name {
            fn borrow(&self) -> &$opaque {
                unsafe {std::mem::transmute(self)}
            }
        }
        impl std::borrow::BorrowMut<$opaque> for $name {
            fn borrow_mut(&mut self) -> &mut $opaque {
                unsafe{std::mem::transmute(self)}
            }
        }
        */
        pub type $name = $opaque;
        #[allow(unused)]
        impl $name {
            pub(crate) fn as_ptr(&self) -> *const $opaque {
                self as * const Self as _
            }
            pub(crate) fn as_ptr_mut(&mut self) -> * mut $opaque {
                self as * mut Self as _
            }
            pub const fn into_native(&self) -> $opaque {
                $opaque {
                    $( $field : self. $field,)+
                }
            }
            $(
                paste::paste!(
                    $vis fn [<get_ $field>](&self) -> $fty {
                        self.$field
                    }
                    $vis fn [<set_ $field>](&mut self, $field: $fty) {
                        self.$field = $field;
                    }
                );
            )+
        }
        impl crate::SkiaOptPtr<$opaque> for Option<$name> {
            fn or_null(&self) -> *const $opaque {
                self.map(|s| s.as_ptr()).unwrap_or(std::ptr::null())
            }
        }
        impl crate::SkiaOptPtr<$opaque> for Option<&$name> {
            fn or_null(&self) -> *const $opaque {
                self.map(|s| s.as_ptr()).unwrap_or(std::ptr::null())
            }
        }
        impl crate::SkiaOptPtrMut<$opaque> for Option<&mut $name> {
            fn or_null_mut(self) -> *mut $opaque {
                match self {
                    Some(m) => m.as_ptr_mut(),
                    None => std::ptr::null_mut(),
                }
            }
        }
        paste::paste!(
        #[cfg(test)]
        #[test]
        fn [<$opaque _layout_tests>]() {
            assert_eq!(std::mem::size_of::<$name>(), std::mem::size_of::<$opaque>());
            assert_eq!(std::mem::align_of::<$name>(), std::mem::align_of::<$opaque>());
            // get a pointer to $name
            const UNINIT: ::std::mem::MaybeUninit<$name> =
            ::std::mem::MaybeUninit::uninit();
            let ptr = UNINIT.as_ptr();
            // get a pointer to $opaque
            const UNINIT_OPAQUE: ::std::mem::MaybeUninit<$opaque> =
            ::std::mem::MaybeUninit::uninit();
            let opaque_ptr = UNINIT_OPAQUE.as_ptr();
            // for each field, assert that the field offset (by subtracting field's address from struct's address) for all fields are same in both structs
            $(
                assert_eq!(
                    unsafe { ::std::ptr::addr_of!((*ptr).$field) as usize - ptr as usize },
                    unsafe { ::std::ptr::addr_of!((*opaque_ptr).$field) as usize - ptr as usize }
                );
            )+
        });
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
macro_rules! skia_wrapper {
    (refcnt, $name: ident, $opaque: ident,$unref: ident $(, $ref: ident)? ) => {
        crate::skia_wrapper!(shared, $name, $opaque, $unref $(, $ref)?);
        unsafe impl crate::VirtualRefCounted for $name {
            fn as_vref_ptr(&self) -> *const sk_refcnt_t {
                self.as_ptr() as _
            }
            fn as_vref_ptr_mut(&mut self) -> *mut sk_refcnt_t {
                self.as_ptr_mut() as _
            }
            fn is_unique(&self) -> bool {
                unsafe { sk_refcnt_unique(self.as_vref_ptr()) }
            }

            fn safe_ref(&mut self) -> Self {
                unsafe {
                    sk_refcnt_safe_ref(self.as_vref_ptr_mut());
                    Self::from_owned_ptr(self.inner)
                }
            }
            fn safe_unref(mut self) {
                unsafe {
                    sk_refcnt_safe_unref(self.as_vref_ptr_mut());
                }
            }
        }
    };
    (nvrefcnt, $name: ident, $opaque: ident,$unref: ident $(, $ref: ident)? ) => {
        crate::skia_wrapper!(shared, $name, $opaque, $unref $(, $ref)?);
        unsafe impl crate::NotVirtualRefCounted for $name {
            fn as_nvref_ptr(&self) -> *const sk_nvrefcnt_t {
                self.as_ptr() as _
            }
            fn as_nvref_ptr_mut(&mut self) -> *mut sk_nvrefcnt_t {
                self.as_ptr_mut() as _
            }
            fn is_unique(&self) -> bool {
                unsafe { sk_nvrefcnt_unique(self.as_nvref_ptr()) }
            }
            fn safe_ref(&mut self) -> Self {
                unsafe {
                    sk_nvrefcnt_safe_ref(self.as_nvref_ptr_mut());
                Self::from_owned_ptr(self.inner) }
            }
            fn safe_unref(mut self) {
                unsafe {
                    sk_nvrefcnt_safe_unref(self.as_nvref_ptr_mut());
                }
            }
        }
    };
    (shared, $name: ident, $opaque: ident,$unref: ident $(, $ref: ident)? ) => {
        crate::skia_wrapper!(unique, $name, $opaque, $unref);
        $(
        impl Clone for $name {
            fn clone(&self) -> Self {
                unsafe { $ref(self.inner) };
                Self { inner: self.inner }
            }
        }
        )?
        #[allow(unused)]
        impl $name {
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
    (unique, $name: ident, $opaque: ident, $del: ident) => {
        pub type $name = crate::SkiaWrapper<$opaque>;
        unsafe impl crate::FfiDrop for $opaque {
            unsafe fn ffi_drop(this: *mut Self) {
                $del(this)
            }
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
pub(crate) use skia_wrapper;
