use std::ffi::CStr;

use crate::bindings::*;

use crate::stream::Stream;
crate::skia_wrapper!(nvrefcnt, SkiaData, sk_data_t, sk_data_unref, sk_data_ref);
impl Default for SkiaData {
    fn default() -> Self {
        unsafe { Self::from_owned_ptr(sk_data_new_empty()) }
    }
}
impl SkiaData {
    pub fn new_with_copy(data: &[u8]) -> Self {
        unsafe { Self::from_owned_ptr(sk_data_new_with_copy(data.as_ptr() as _, data.len())) }
    }
    pub fn new_subset(data: &Self, offset: usize, len: usize) -> Self {
        unsafe { Self::from_owned_ptr(sk_data_new_subset(data.as_ptr(), offset, len)) }
    }
    pub fn get_size(&self) -> usize {
        unsafe { sk_data_get_size(self.as_ptr()) }
    }
    pub fn get_bytes(&self) -> &[u8] {
        let size = self.get_size();
        unsafe { std::slice::from_raw_parts(sk_data_get_bytes(self.as_ptr()), size) }
    }
    pub fn new_from_file(path: &CStr) -> Self {
        unsafe { Self::from_owned_ptr(sk_data_new_from_file(path.as_ptr())) }
    }
    pub fn new_from_stream(stream: &mut impl Stream, len: usize) -> Self {
        unsafe {
            Self::from_owned_ptr(sk_data_new_from_stream(stream.borrow_stream_mut_ptr(), len))
        }
    }
    /*    pub fn sk_data_new_with_proc(
        ptr: *const ::std::os::raw::c_void,
        length: usize,
        proc_: sk_data_release_proc,
        ctx: *mut ::std::os::raw::c_void,
    ) -> *mut sk_data_t; */
    pub fn new_uninitialized(len: usize) -> Self {
        unsafe { Self::from_owned_ptr(sk_data_new_uninitialized(len)) }
    }
}

#[cfg(test)]
mod test {
    use crate::NotVirtualRefCounted;

    use super::SkiaData;

    #[test]
    pub fn test_unique() {
        let mut data = SkiaData::new_uninitialized(128);
        assert!(
            data.is_unique(),
            "skia data is not unique after just being initialized"
        );
        let d = data.safe_ref();
        assert!(
            !data.is_unique(),
            "skia data is unique after we just incremented the ref count"
        );
        d.safe_unref();
        assert!(data.is_unique(), "skia data is not unique after safe unref");
    }
}
