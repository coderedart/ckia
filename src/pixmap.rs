use std::{marker::PhantomData, mem::transmute};

use crate::bindings::*;

use crate::{stream::WStream, ImageInfo, PngEncoderFilterFlags};

#[repr(transparent)]
pub struct PixMap<'a, T = ()> {
    pub(crate) inner: *mut sk_pixmap_t,
    phantom: PhantomData<&'a T>,
}
impl<'a, T> Drop for PixMap<'a, T> {
    fn drop(&mut self) {
        unsafe { sk_pixmap_destructor(self.inner) }
    }
}
impl Default for PixMap<'static, ()> {
    fn default() -> Self {
        let inner = unsafe { sk_pixmap_new() };
        assert!(!inner.is_null());
        PixMap {
            inner,
            phantom: PhantomData,
        }
    }
}
impl<'a, T> PixMap<'a, T> {
    pub fn new_with_params(
        info: &'a ImageInfo,
        buffer: &'a mut [u8],
        row_bytes: usize,
    ) -> PixMap<'a, &'a mut [u8]> {
        let inner = unsafe {
            sk_pixmap_new_with_params(info.as_ptr(), buffer.as_mut_ptr() as _, row_bytes)
        };
        assert!(!inner.is_null());
        PixMap {
            inner,
            phantom: PhantomData,
        }
    }
    pub fn reset(self) -> PixMap<'static, ()> {
        unsafe {
            sk_pixmap_reset(self.inner);
            std::mem::transmute(self)
        }
    }
    pub fn reset_with_params(
        self,
        info: &ImageInfo,
        buffer: &'a mut [u8],
        row_bytes: usize,
    ) -> PixMap<'a, &'a mut [u8]> {
        unsafe {
            sk_pixmap_reset_with_params(
                self.inner,
                info.as_ptr(),
                buffer.as_mut_ptr() as _,
                row_bytes,
            );
            transmute(self)
        }
    }

    /// returns true on success.
    #[must_use]
    pub fn encode_png(
        &self,
        stream: &mut impl WStream,
        png_filter_flags: Option<PngEncoderFilterFlags>,
        z_lib_level: Option<i32>,
    ) -> bool {
        let options = sk_pngencoder_options_t {
            fFilterFlags: png_filter_flags
                .unwrap_or(PngEncoderFilterFlags::ALL_SK_PNGENCODER_FILTER_FLAGS),
            fZLibLevel: z_lib_level.unwrap_or(6),
            fComments: std::ptr::null_mut(),
            fICCProfile: std::ptr::null(),
            fICCProfileDescription: std::ptr::null(),
        };
        unsafe { sk_pngencoder_encode(stream.borrow_wstream_mut_ptr(), self.inner, &options as _) }
    }
}
/*


pub fn sk_pixmap_set_colorspace(cpixmap: *mut sk_pixmap_t, colorspace: *mut sk_colorspace_t);
pub fn sk_pixmap_extract_subset(
    cpixmap: *const sk_pixmap_t,
    result: *mut sk_pixmap_t,
    subset: *const sk_irect_t,
) -> bool;
pub fn sk_pixmap_get_info(cpixmap: *const sk_pixmap_t, cinfo: *mut sk_imageinfo_t);
pub fn sk_pixmap_get_row_bytes(cpixmap: *const sk_pixmap_t) -> usize;
pub fn sk_pixmap_get_colorspace(cpixmap: *const sk_pixmap_t) -> *mut sk_colorspace_t;
pub fn sk_pixmap_compute_is_opaque(cpixmap: *const sk_pixmap_t) -> bool;
pub fn sk_pixmap_get_pixel_color(
    cpixmap: *const sk_pixmap_t,
    x: ::std::os::raw::c_int,
    y: ::std::os::raw::c_int,
) -> sk_color_t;
pub fn sk_pixmap_get_pixel_color4f(
    cpixmap: *const sk_pixmap_t,
    x: ::std::os::raw::c_int,
    y: ::std::os::raw::c_int,
    color: *mut sk_color4f_t,
);
pub fn sk_pixmap_get_pixel_alphaf(
    cpixmap: *const sk_pixmap_t,
    x: ::std::os::raw::c_int,
    y: ::std::os::raw::c_int,
) -> f32;
pub fn sk_pixmap_get_writable_addr(cpixmap: *const sk_pixmap_t) -> *mut ::std::os::raw::c_void;
pub fn sk_pixmap_get_writeable_addr_with_xy(
    cpixmap: *const sk_pixmap_t,
    x: ::std::os::raw::c_int,
    y: ::std::os::raw::c_int,
) -> *mut ::std::os::raw::c_void;
pub fn sk_pixmap_read_pixels(
    cpixmap: *const sk_pixmap_t,
    dstInfo: *const sk_imageinfo_t,
    dstPixels: *mut ::std::os::raw::c_void,
    dstRowBytes: usize,
    srcX: ::std::os::raw::c_int,
    srcY: ::std::os::raw::c_int,
) -> bool;
pub fn sk_pixmap_scale_pixels(
    cpixmap: *const sk_pixmap_t,
    dst: *const sk_pixmap_t,
    sampling: *const sk_sampling_options_t,
) -> bool;
pub fn sk_pixmap_erase_color(
    cpixmap: *const sk_pixmap_t,
    color: sk_color_t,
    subset: *const sk_irect_t,
) -> bool;
pub fn sk_pixmap_erase_color4f(
    cpixmap: *const sk_pixmap_t,
    color: *const sk_color4f_t,
    subset: *const sk_irect_t,
) -> bool;
pub fn sk_webpencoder_encode(
    dst: *mut sk_wstream_t,
    src: *const sk_pixmap_t,
    options: *const sk_webpencoder_options_t,
) -> bool;
pub fn sk_jpegencoder_encode(
    dst: *mut sk_wstream_t,
    src: *const sk_pixmap_t,
    options: *const sk_jpegencoder_options_t,
) -> bool;
*/
