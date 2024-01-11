use std::ffi::CStr;

use ckia_sys::*;

crate::opaque_unique!(FileWStream, sk_wstream_filestream_t, sk_filewstream_destroy);

impl FileWStream {
    pub fn is_valid(&mut self) -> bool {
        unsafe { sk_filewstream_is_valid(self.inner) }
    }
    pub fn new(cstr: &CStr) -> Option<Self> {
        let inner = unsafe { sk_filewstream_new(cstr.as_ptr()) };
        if !inner.is_null() {
            Some(Self { inner })
        } else {
            None
        }
    }
}
unsafe impl WStream for FileWStream {
    fn borrow_wstream_mut_ptr(&mut self) -> *mut sk_wstream_t {
        self.inner as _
    }
}
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
pub trait Stream {
    fn borrow_stream_mut_ptr(&mut self) -> *mut sk_stream_t;
}
