use std::mem::transmute;

use ckia_sys::*;

use crate::{color::Color, pixmap::PixMap, skia_wrapper, IRect, ImageInfo, SkiaPointer};

skia_wrapper!(unique, BitMap, sk_bitmap_t, sk_bitmap_destructor);

impl Default for BitMap {
    fn default() -> Self {
        unsafe { Self::from_owned_ptr(sk_bitmap_new()) }
    }
}
impl BitMap {
    pub fn get_info(&mut self) -> ImageInfo {
        let mut info = ImageInfo::default();
        unsafe {
            sk_bitmap_get_info(self.inner, info.as_ptr_mut());
        }
        info
    }
    pub fn get_pixels(&mut self) -> &mut [u8] {
        let mut length = 0usize;
        unsafe {
            let ptr_to_pixels = sk_bitmap_get_pixels(self.inner, &mut length as _);
            std::slice::from_raw_parts_mut(ptr_to_pixels as _, length)
        }
    }
    pub fn get_row_bytes(&mut self) -> usize {
        unsafe { sk_bitmap_get_row_bytes(self.inner) }
    }
    pub fn get_byte_count(&mut self) -> usize {
        unsafe { sk_bitmap_get_byte_count(self.inner) }
    }
    pub fn reset(&mut self) {
        unsafe {
            sk_bitmap_reset(self.inner);
        }
    }
    pub fn is_null(&mut self) -> bool {
        unsafe { sk_bitmap_is_null(self.inner) }
    }
    pub fn is_immutable(&mut self) -> bool {
        unsafe { sk_bitmap_is_immutable(self.inner) }
    }
    pub fn set_immutable(&mut self) {
        unsafe { sk_bitmap_set_immutable(self.inner) }
    }
    pub fn erase(&mut self, color: Color) {
        unsafe {
            sk_bitmap_erase(self.inner, color.0);
        }
    }
    pub fn erase_rect(&mut self, color: Color, rect: &mut IRect) {
        unsafe {
            sk_bitmap_erase_rect(self.inner, color.0, rect.as_ptr_mut());
        }
    }
    pub fn get_addr_8(&mut self, x: i32, y: i32) -> Option<&mut u8> {
        unsafe { sk_bitmap_get_addr_8(self.inner, x, y).as_mut() }
    }
    pub fn get_addr_16(&mut self, x: i32, y: i32) -> Option<&mut u16> {
        unsafe { sk_bitmap_get_addr_16(self.inner, x, y).as_mut() }
    }
    pub fn get_addr_32(&mut self, x: i32, y: i32) -> Option<&mut u32> {
        unsafe { sk_bitmap_get_addr_32(self.inner, x, y).as_mut() }
    }
    pub fn get_addr(&mut self, x: i32, y: i32) -> Option<&mut std::ffi::c_void> {
        unsafe { sk_bitmap_get_addr(self.inner, x, y).as_mut() }
    }
    pub fn get_pixel_color(&mut self, x: i32, y: i32) -> Color {
        unsafe { Color(sk_bitmap_get_pixel_color(self.inner, x, y)) }
    }
    pub fn ready_to_draw(&mut self) -> bool {
        unsafe { sk_bitmap_ready_to_draw(self.inner) }
    }
    /// will panic if the length of `colors` is not width x height.
    pub fn get_pixel_colors(&mut self) -> Vec<Color> {
        let info = self.get_info();
        let mut colors = Vec::with_capacity(info.get_width() as usize * info.get_height() as usize);
        unsafe {
            sk_bitmap_get_pixel_colors(self.inner, colors.as_mut_ptr() as _);
        }
        colors
    }
    #[must_use]
    pub fn try_alloc_pixels(&mut self, requested_info: &ImageInfo, row_bytes: usize) -> bool {
        assert!(
            requested_info.get_width() as usize * requested_info.bytes_per_pixel() as usize
                <= row_bytes
                || row_bytes == 0,
            "invalid row bytes value"
        );
        unsafe { sk_bitmap_try_alloc_pixels(self.inner, requested_info.as_ptr(), row_bytes) }
    }
    pub fn swap(&mut self, other: &mut Self) {
        unsafe {
            sk_bitmap_swap(self.inner, other.inner);
        }
    }
    pub fn notify_pixels_changed(&mut self) {
        unsafe { sk_bitmap_notify_pixels_changed(self.inner) }
    }
    /*
    pub fn sk_bitmap_get_pixel_colors(cbitmap: *mut sk_bitmap_t, colors: *mut sk_color_t);
    pub fn sk_bitmap_install_pixels(
        cbitmap: *mut sk_bitmap_t,
        cinfo: *const sk_imageinfo_t,
        pixels: *mut ::std::os::raw::c_void,
        rowBytes: usize,
        releaseProc: sk_bitmap_release_proc,
        context: *mut ::std::os::raw::c_void,
    ) -> bool;
    pub fn sk_bitmap_install_pixels_with_pixmap(
        cbitmap: *mut sk_bitmap_t,
        cpixmap: *const sk_pixmap_t,
    ) -> bool;
    pub fn sk_bitmap_try_alloc_pixels_with_flags(
        cbitmap: *mut sk_bitmap_t,
        requestedInfo: *const sk_imageinfo_t,
        flags: u32,
    ) -> bool;
    pub fn sk_bitmap_set_pixels(cbitmap: *mut sk_bitmap_t, pixels: *mut ::std::os::raw::c_void);
     */
    /// We consume by self here because we need to change the lifetime of the pixmap (and by extension its type)
    /// If we succeed, we will return the pixmap with lifetime attached to this bitmap.
    /// If we fail, we will just return the pixmap with the previous lifetime, as pixmap won't be touched in the failure case
    pub fn peek_pixels<'a, 'b, T>(
        &mut self,
        pixmap: PixMap<'b, T>,
    ) -> Result<PixMap<'a, Self>, PixMap<'b, T>> {
        unsafe {
            if sk_bitmap_peek_pixels(self.inner, pixmap.inner) {
                Ok(transmute(pixmap))
            } else {
                Err(pixmap)
            }
        }
    }
    /*
    pub fn sk_bitmap_extract_subset(
        cbitmap: *mut sk_bitmap_t,
        dst: *mut sk_bitmap_t,
        subset: *mut sk_irect_t,
    ) -> bool;
    pub fn sk_bitmap_extract_alpha(
        cbitmap: *mut sk_bitmap_t,
        dst: *mut sk_bitmap_t,
        paint: *const sk_paint_t,
        offset: *mut sk_ipoint_t,
    ) -> bool;
    pub fn sk_bitmap_make_shader(
        cbitmap: *mut sk_bitmap_t,
        tmx: sk_shader_tilemode_t,
        tmy: sk_shader_tilemode_t,
        sampling: *mut sk_sampling_options_t,
        cmatrix: *const sk_matrix_t,
    ) -> *mut sk_shader_t;

     */
}
