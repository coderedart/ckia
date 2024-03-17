use std::{ffi::CStr, marker::PhantomData, mem::transmute};

use crate::bindings::*;

use crate::data::SkiaData;

crate::skia_wrapper!(
    unique,
    DynamicMemoryStream,
    sk_wstream_dynamicmemorystream_t,
    sk_dynamicmemorywstream_destroy
);
impl Default for DynamicMemoryStream {
    fn default() -> Self {
        unsafe { Self::from_owned_ptr(sk_dynamicmemorywstream_new()) }
    }
}
impl DynamicMemoryStream {
    pub fn detach_as_stream(&mut self) -> StreamAsset {
        unsafe { StreamAsset::from_owned_ptr(sk_dynamicmemorywstream_detach_as_stream(self.inner)) }
    }
    pub fn detach_as_data(&mut self) -> SkiaData {
        unsafe { SkiaData::from_owned_ptr(sk_dynamicmemorywstream_detach_as_data(self.inner)) }
    }
    /// dst needs to be atleast `Self::bytes_written` long or we panic
    pub fn copy_to(&mut self, dst: &mut [u8]) {
        assert!(dst.len() >= self.bytes_written());
    }

    /// returns true on success. if false, then there must be an error.
    #[must_use]
    pub fn write_to(&mut self, dst: &mut impl WStream) -> bool {
        unsafe { sk_dynamicmemorywstream_write_to_stream(self.inner, dst.borrow_wstream_mut_ptr()) }
    }
}
unsafe impl WStream for DynamicMemoryStream {
    fn borrow_wstream_mut_ptr(&mut self) -> *mut sk_wstream_t {
        self.inner as _
    }
}
crate::skia_wrapper!(
    unique,
    StreamAsset,
    sk_stream_asset_t,
    sk_stream_asset_destroy
);
unsafe impl Stream for StreamAsset {
    fn borrow_stream_mut_ptr(&mut self) -> *mut sk_stream_t {
        self.inner as _
    }
}
crate::skia_wrapper!(
    unique,
    FileStream,
    sk_stream_filestream_t,
    sk_filestream_destroy
);

impl FileStream {
    pub fn is_valid(&mut self) -> bool {
        unsafe { sk_filestream_is_valid(self.inner) }
    }
    pub fn new(cstr: &CStr) -> Option<Self> {
        unsafe { Self::try_from_owned_ptr(sk_filestream_new(cstr.as_ptr())) }
    }
}
unsafe impl Stream for FileStream {
    fn borrow_stream_mut_ptr(&mut self) -> *mut sk_stream_t {
        self.inner as _
    }
}
crate::skia_wrapper!(
    unique,
    FileWStream,
    sk_wstream_filestream_t,
    sk_filewstream_destroy
);

impl FileWStream {
    pub fn is_valid(&mut self) -> bool {
        unsafe { sk_filewstream_is_valid(self.inner) }
    }
    pub fn new(cstr: &CStr) -> Option<Self> {
        unsafe { Self::try_from_owned_ptr(sk_filewstream_new(cstr.as_ptr())) }
    }
}
unsafe impl WStream for FileWStream {
    fn borrow_wstream_mut_ptr(&mut self) -> *mut sk_wstream_t {
        self.inner as _
    }
}
#[derive(Debug)]
#[repr(transparent)]
pub struct MemoryStream<'a> {
    pub(crate) inner: *mut sk_stream_memorystream_t,
    phantom: PhantomData<&'a [u8]>,
}
impl<'a> Drop for MemoryStream<'a> {
    fn drop(&mut self) {
        unsafe {
            sk_memorystream_destroy(self.inner);
        }
    }
}
#[allow(unused)]
impl<'a> MemoryStream<'a> {
    #[doc = r" consumes struct and returns a ptr that has ownership."]
    #[doc = r" # Safety"]
    #[doc = r" caller needs to call unref after being done with it."]
    pub(crate) unsafe fn into_owned_ptr(self) -> *mut sk_stream_memorystream_t {
        let inner = self.inner;
        std::mem::forget(self);
        inner
    }
    #[doc = r" takes a pointer and assumes ownership of it. returns None if nullptr."]
    #[doc = r" # Safety"]
    #[doc = r" the struct assumes ownership, so caller shouldn't use it after that."]
    #[doc = r" also make sure that you haven't accidentally called unref on it in the past."]
    pub(crate) unsafe fn from_owned_ptr(ptr: *mut sk_stream_memorystream_t) -> Option<Self> {
        if ptr.is_null() {
            None
        } else {
            Some(Self {
                inner: ptr,
                phantom: PhantomData,
            })
        }
    }
}

impl MemoryStream<'static> {
    pub fn new() -> Option<Self> {
        unsafe { Self::from_owned_ptr(sk_memorystream_new()) }
    }
    pub fn new_with_len(len: usize) -> Option<Self> {
        unsafe { Self::from_owned_ptr(sk_memorystream_new_with_length(len)) }
    }
    /// copies and owns the data
    pub fn new_with_copied_data(data: &[u8]) -> Option<Self> {
        let inner = unsafe { sk_memorystream_new_with_data(data.as_ptr() as _, data.len(), true) };
        if !inner.is_null() {
            Some(Self {
                inner,
                phantom: PhantomData,
            })
        } else {
            None
        }
    }
    /// copies and owns its data
    pub fn set_memory_with_copied_data(self, data: &[u8]) -> Self {
        unsafe {
            sk_memorystream_set_memory(self.inner, data.as_ptr() as _, data.len(), false);
            transmute(self)
        }
    }
}
impl<'a, 'b> MemoryStream<'a> {
    /// useful for static data or temporary data that you can guarantee will live for the duration of this struct
    pub fn new_with_borrowed_data(data: &'a [u8]) -> Option<Self> {
        let inner = unsafe { sk_memorystream_new_with_data(data.as_ptr() as _, data.len(), false) };
        if !inner.is_null() {
            Some(Self {
                inner,
                phantom: PhantomData,
            })
        } else {
            None
        }
    }
    pub fn new_with_skia_data(data: &SkiaData) -> Option<Self> {
        unsafe { Self::from_owned_ptr(sk_memorystream_new_with_skdata(data.as_ptr() as _)) }
    }
    /// borrows the slice and doesn't own any data.
    pub fn set_memory_with_borrowed_data(self, data: &'b [u8]) -> MemoryStream<'b> {
        unsafe {
            sk_memorystream_set_memory(self.inner, data.as_ptr() as _, data.len(), false);
            transmute(self)
        }
    }
}
unsafe impl<'a> Stream for MemoryStream<'a> {
    fn borrow_stream_mut_ptr(&mut self) -> *mut sk_stream_t {
        self.inner as _
    }
}
/// # Safety
/// Do NOT implement this outside of this crate. All of the default impl methods in this crate basically take a mut pointer and send it across FFI boundary where methods are dispatched using vtables
pub unsafe trait WStream {
    fn borrow_wstream_mut_ptr(&mut self) -> *mut sk_wstream_t;
    fn write(&mut self, buffer: &[u8]) -> bool {
        let wstream = self.borrow_wstream_mut_ptr();
        unsafe { sk_wstream_write(wstream, buffer.as_ptr() as _, buffer.len()) }
    }
    fn newline(&mut self) -> bool {
        let wstream = self.borrow_wstream_mut_ptr();
        unsafe { sk_wstream_newline(wstream) }
    }
    fn flush(&mut self) {
        let wstream = self.borrow_wstream_mut_ptr();
        unsafe { sk_wstream_flush(wstream) }
    }
    fn bytes_written(&mut self) -> usize {
        let wstream = self.borrow_wstream_mut_ptr();
        unsafe { sk_wstream_bytes_written(wstream) }
    }
    fn write_8(&mut self, value: u8) -> bool {
        let wstream = self.borrow_wstream_mut_ptr();
        unsafe { sk_wstream_write_8(wstream, value) }
    }
    fn write_16(&mut self, value: u16) -> bool {
        let wstream = self.borrow_wstream_mut_ptr();
        unsafe { sk_wstream_write_16(wstream, value) }
    }
    fn write_32(&mut self, value: u32) -> bool {
        let wstream = self.borrow_wstream_mut_ptr();
        unsafe { sk_wstream_write_32(wstream, value) }
    }
    fn write_text(&mut self, cstr: &CStr) -> bool {
        let wstream = self.borrow_wstream_mut_ptr();
        unsafe { sk_wstream_write_text(wstream, cstr.as_ptr()) }
    }
    fn write_dec_as_text(&mut self, value: i32) -> bool {
        let wstream = self.borrow_wstream_mut_ptr();
        unsafe { sk_wstream_write_dec_as_text(wstream, value) }
    }
    fn write_bigdec_as_text(&mut self, value: i64, min_digits: i32) -> bool {
        let wstream = self.borrow_wstream_mut_ptr();
        unsafe { sk_wstream_write_bigdec_as_text(wstream, value, min_digits) }
    }
    fn write_hex_as_text(&mut self, value: u32, min_digits: i32) -> bool {
        let wstream = self.borrow_wstream_mut_ptr();
        unsafe { sk_wstream_write_hex_as_text(wstream, value, min_digits) }
    }
    fn write_scalar_as_text(&mut self, value: f32) -> bool {
        let wstream = self.borrow_wstream_mut_ptr();
        unsafe { sk_wstream_write_scalar_as_text(wstream, value) }
    }
    fn write_bool(&mut self, value: bool) -> bool {
        let wstream = self.borrow_wstream_mut_ptr();
        unsafe { sk_wstream_write_bool(wstream, value) }
    }
    fn write_scalar(&mut self, value: f32) -> bool {
        let wstream = self.borrow_wstream_mut_ptr();
        unsafe { sk_wstream_write_scalar(wstream, value) }
    }
    fn write_packed_uint(&mut self, value: usize) -> bool {
        let wstream = self.borrow_wstream_mut_ptr();
        unsafe { sk_wstream_write_packed_uint(wstream, value) }
    }
    fn write_stream(&mut self, input: &mut impl Stream, length: usize) -> bool {
        let wstream = self.borrow_wstream_mut_ptr();
        unsafe { sk_wstream_write_stream(wstream, input.borrow_stream_mut_ptr(), length) }
    }
    fn get_size_of_packed_uint(&mut self, value: usize) -> i32 {
        unsafe { sk_wstream_get_size_of_packed_uint(value) }
    }
}

/// # Safety
/// Do NOT impl this out of crate. All of the default implemented methods take raw pointer, send it across FFI boundary, so that the relevant method can be dispatched using vtables.
pub unsafe trait Stream {
    fn borrow_stream_mut_ptr(&mut self) -> *mut sk_stream_t;
    fn read(&mut self, buffer: &mut [u8]) -> usize {
        let stream = self.borrow_stream_mut_ptr();
        unsafe { sk_stream_read(stream, buffer.as_mut_ptr() as _, buffer.len()) }
    }
    fn peek(&mut self, buffer: &mut [u8]) -> usize {
        let stream = self.borrow_stream_mut_ptr();
        unsafe { sk_stream_peek(stream, buffer.as_mut_ptr() as _, buffer.len()) }
    }

    fn skip(&mut self, size: usize) -> usize {
        let stream = self.borrow_stream_mut_ptr();
        unsafe { sk_stream_skip(stream, size) }
    }
    fn is_at_end(&mut self) -> bool {
        let stream = self.borrow_stream_mut_ptr();
        unsafe { sk_stream_is_at_end(stream) }
    }
    fn read_i8(&mut self) -> Option<i8> {
        let mut value: i8 = 0;
        let stream = self.borrow_stream_mut_ptr();
        unsafe { sk_stream_read_s8(stream, &mut value as _).then_some(value) }
    }
    fn read_i16(&mut self) -> Option<i16> {
        let mut value: i16 = 0;
        let stream = self.borrow_stream_mut_ptr();
        unsafe { sk_stream_read_s16(stream, &mut value as _).then_some(value) }
    }
    fn read_i32(&mut self) -> Option<i32> {
        let mut value: i32 = 0;
        let stream = self.borrow_stream_mut_ptr();
        unsafe { sk_stream_read_s32(stream, &mut value as _).then_some(value) }
    }
    fn read_u8(&mut self) -> Option<u8> {
        let mut value: u8 = 0;
        let stream = self.borrow_stream_mut_ptr();
        unsafe { sk_stream_read_u8(stream, &mut value as _).then_some(value) }
    }
    fn read_u16(&mut self) -> Option<u16> {
        let mut value: u16 = 0;
        let stream = self.borrow_stream_mut_ptr();
        unsafe { sk_stream_read_u16(stream, &mut value as _).then_some(value) }
    }
    fn read_u32(&mut self) -> Option<u32> {
        let mut value: u32 = 0;
        let stream = self.borrow_stream_mut_ptr();
        unsafe { sk_stream_read_u32(stream, &mut value as _).then_some(value) }
    }

    fn read_bool(&mut self) -> Option<bool> {
        let mut value: bool = false;
        let stream = self.borrow_stream_mut_ptr();
        unsafe { sk_stream_read_bool(stream, &mut value).then_some(value) }
    }
    fn rewind(&mut self) -> bool {
        let stream = self.borrow_stream_mut_ptr();
        unsafe { sk_stream_rewind(stream) }
    }
    fn has_position(&mut self) -> bool {
        let stream = self.borrow_stream_mut_ptr();
        unsafe { sk_stream_has_position(stream) }
    }
    fn has_length(&mut self) -> bool {
        let stream = self.borrow_stream_mut_ptr();
        unsafe { sk_stream_has_length(stream) }
    }

    fn get_position(&mut self) -> usize {
        let stream = self.borrow_stream_mut_ptr();
        unsafe { sk_stream_get_position(stream) }
    }
    fn get_length(&mut self) -> usize {
        let stream = self.borrow_stream_mut_ptr();
        unsafe { sk_stream_get_length(stream) }
    }

    fn seek(&mut self, value: usize) -> bool {
        let stream = self.borrow_stream_mut_ptr();
        unsafe { sk_stream_seek(stream, value) }
    }
    fn move_to_offset(&mut self, value: i64) -> bool {
        let stream = self.borrow_stream_mut_ptr();
        unsafe { sk_stream_move(stream, value) }
    }
    fn get_memory_base(&mut self) -> *const std::ffi::c_void {
        let stream = self.borrow_stream_mut_ptr();
        unsafe { sk_stream_get_memory_base(stream) }
    }
    fn fork(&mut self) -> *mut sk_stream_t {
        let stream = self.borrow_stream_mut_ptr();
        unsafe { sk_stream_fork(stream) }
    }
    fn duplicate(&mut self) -> *mut sk_stream_t {
        let stream = self.borrow_stream_mut_ptr();
        unsafe { sk_stream_duplicate(stream) }
    }
}
