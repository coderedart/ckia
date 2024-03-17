use std::ptr::{null, null_mut};

use crate::bindings::*;
use crate::{
    paint::Paint, path::SkiaPath, typeface::Typeface, FontEdging, FontHinting, FontMetrics, Point,
    Rect, TextEncoding,
};

crate::skia_wrapper!(unique, Font, sk_font_t, sk_font_delete);

impl Default for Font {
    fn default() -> Self {
        unsafe { Self::from_owned_ptr(sk_font_new()) }
    }
}
impl Font {
    pub fn new_with_values(
        tf: &mut Typeface,
        size: f32,
        scale_x: f32,
        skew_x: f32,
    ) -> Option<Self> {
        unsafe {
            Self::try_from_owned_ptr(sk_font_new_with_values(
                tf.as_ptr_mut(),
                size,
                scale_x,
                skew_x,
            ))
        }
    }
    pub fn is_force_auto_hinting(&self) -> bool {
        unsafe { sk_font_is_force_auto_hinting(self.as_ptr()) }
    }
    pub fn set_force_auto_hinting(&mut self, value: bool) {
        unsafe { sk_font_set_force_auto_hinting(self.as_ptr_mut(), value) }
    }
    pub fn is_embedded_bitmaps(&self) -> bool {
        unsafe { sk_font_is_embedded_bitmaps(self.as_ptr()) }
    }
    pub fn set_embedded_bitmaps(&mut self, value: bool) {
        unsafe { sk_font_set_embedded_bitmaps(self.as_ptr_mut(), value) }
    }
    pub fn is_subpixel(&self) -> bool {
        unsafe { sk_font_is_subpixel(self.as_ptr()) }
    }
    pub fn set_subpixel(&mut self, value: bool) {
        unsafe { sk_font_set_subpixel(self.as_ptr_mut(), value) }
    }
    pub fn is_linear_metrics(&self) -> bool {
        unsafe { sk_font_is_linear_metrics(self.as_ptr()) }
    }
    pub fn set_linear_metrics(&mut self, value: bool) {
        unsafe { sk_font_set_linear_metrics(self.as_ptr_mut(), value) }
    }
    pub fn is_embolden(&self) -> bool {
        unsafe { sk_font_is_embolden(self.as_ptr()) }
    }
    pub fn set_embolden(&mut self, value: bool) {
        unsafe { sk_font_set_embolden(self.as_ptr_mut(), value) }
    }
    pub fn is_baseline_snap(&self) -> bool {
        unsafe { sk_font_is_baseline_snap(self.as_ptr()) }
    }
    pub fn set_baseline_snap(&mut self, value: bool) {
        unsafe { sk_font_set_baseline_snap(self.as_ptr_mut(), value) }
    }
    pub fn get_edging(&self) -> FontEdging {
        unsafe { sk_font_get_edging(self.as_ptr()) }
    }
    pub fn set_edging(&mut self, value: FontEdging) {
        unsafe { sk_font_set_edging(self.as_ptr_mut(), value) }
    }
    pub fn get_hinting(&self) -> FontHinting {
        unsafe { sk_font_get_hinting(self.as_ptr()) }
    }
    pub fn set_hinting(&mut self, value: FontHinting) {
        unsafe { sk_font_set_hinting(self.as_ptr_mut(), value) }
    }
    pub fn get_typeface(&self) -> Typeface {
        unsafe { Typeface::from_owned_ptr(sk_font_get_typeface(self.as_ptr())) }
    }
    pub fn set_typeface(&mut self, value: &mut Typeface) {
        unsafe { sk_font_set_typeface(self.as_ptr_mut(), value.as_ptr_mut()) }
    }
    pub fn get_size(&self) -> f32 {
        unsafe { sk_font_get_size(self.as_ptr()) }
    }
    pub fn set_size(&mut self, value: f32) {
        unsafe { sk_font_set_size(self.as_ptr_mut(), value) }
    }
    pub fn get_scale_x(&self) -> f32 {
        unsafe { sk_font_get_scale_x(self.as_ptr()) }
    }
    pub fn set_scale_x(&mut self, value: f32) {
        unsafe { sk_font_set_scale_x(self.as_ptr_mut(), value) }
    }
    pub fn get_skew_x(&self) -> f32 {
        unsafe { sk_font_get_skew_x(self.as_ptr()) }
    }
    pub fn set_skew_x(&mut self, value: f32) {
        unsafe { sk_font_set_skew_x(self.as_ptr_mut(), value) }
    }
    /// fills the glyphs of unicode codepoints of the `text`
    /// returns the number of glyphs written into the slice.
    /// If the glyphs won't fit, then it will return the total size *needed* and will NOT write any glyphs.
    /// you must call again with a new slice that has enough space
    pub fn text_to_glyphs(&self, text: &str, glyphs: &mut [u16]) -> i32 {
        unsafe {
            sk_font_text_to_glyphs(
                self.as_ptr(),
                text.as_ptr() as _,
                text.len(),
                TextEncoding::UTF8_SK_TEXT_ENCODING,
                glyphs.as_mut_ptr(),
                glyphs.len() as _,
            )
        }
    }
    pub fn unichar_to_glyphs(&self, uni: i32) -> u16 {
        unsafe { sk_font_unichar_to_glyph(self.as_ptr(), uni) }
    }
    pub fn unichars_to_glyphs(&self, uni: &[i32], glyphs: &mut [u16]) {
        assert!(uni.len() <= glyphs.len());
        unsafe {
            sk_font_unichars_to_glyphs(
                self.as_ptr(),
                uni.as_ptr(),
                uni.len() as _,
                glyphs.as_mut_ptr(),
            )
        }
    }

    /// measures text. if rect is Some, then it will also measure the "bounding rect" of the rect. will be set to empty if text is empty.
    /// If Paint is Some, it will also consider the impact of path effect, stroke width etc.. on the text's size.
    /// The return value is simply the "width" -> how much you need to advance after drawing this text. -> the combination of all the "widths" of characters added.
    pub fn measure_text(
        &self,
        text: &str,
        bounds: Option<&mut Rect>,
        paint: Option<&Paint>,
    ) -> f32 {
        unsafe {
            sk_font_measure_text(
                self.as_ptr(),
                text.as_ptr() as _,
                text.len(),
                TextEncoding::UTF8_SK_TEXT_ENCODING,
                bounds
                    .map(|r| r.as_ptr_mut())
                    .unwrap_or(std::ptr::null_mut()),
                paint.map(|p| p.as_ptr()).unwrap_or(std::ptr::null()),
            )
        }
    }
    /* pointless to implement a duplicate?
    pub fn sk_font_measure_text_no_return(
        font: *const sk_font_t,
        text: *const ::std::os::raw::c_void,
        byteLength: usize,
        encoding: sk_text_encoding_t,
        bounds: *mut sk_rect_t,
        paint: *const sk_paint_t,
        measuredWidth: *mut f32,
    );
    */
    /// gets the widths/bounds of each glyph in the slice and writes them to the mut slices (if not None).
    /// if paint is Some, it will be used in calculation of the widths/bounds.
    /// the output slices MUST be as big as glyphs slice
    pub fn get_widths_bounds(
        &self,
        glyphs: &[u16],
        widths: Option<&mut [f32]>,
        bounds: Option<&mut [Rect]>,
        paint: Option<&Paint>,
    ) {
        if let Some(bounds) = bounds.as_ref() {
            assert!(glyphs.len() <= bounds.len())
        }
        if let Some(widths) = widths.as_ref() {
            assert!(glyphs.len() <= widths.len())
        }
        unsafe {
            sk_font_get_widths_bounds(
                self.as_ptr(),
                glyphs.as_ptr(),
                glyphs.len() as _,
                widths.map(|w| w.as_mut_ptr()).unwrap_or(null_mut()),
                bounds.map(|b| b.as_mut_ptr() as _).unwrap_or(null_mut()),
                paint.map(Paint::as_ptr).unwrap_or(null()),
            )
        }
    }
    /// size of pos must be atleast the size of glyphs
    pub fn get_pos(&self, glyphs: &[u16], pos: &mut [Point], mut origin: Point) {
        assert!(glyphs.len() <= pos.len());
        unsafe {
            sk_font_get_pos(
                self.as_ptr(),
                glyphs.as_ptr(),
                glyphs.len() as _,
                pos.as_mut_ptr() as _,
                origin.as_ptr_mut(),
            )
        }
    }
    /// size of xpos must be atleast the size of glyphs
    pub fn get_xpos(&self, glyphs: &[u16], xpos: &mut [f32], origin: f32) {
        assert!(glyphs.len() <= xpos.len());
        unsafe {
            sk_font_get_xpos(
                self.as_ptr(),
                glyphs.as_ptr(),
                glyphs.len() as _,
                xpos.as_mut_ptr() as _,
                origin,
            )
        }
    }
    ///
    /// # Safety
    /// If you pass in None, then we return the size of vec needed.
    /// If you pass in Some, then we will fill as much as we can and still return the vec size needed.
    /// So, it is upto the user to actually ensure that the returned usize is less than or equal to vec size.
    /// So that they are not missing any textboxes
    pub unsafe fn get_intercepts(
        &mut self,
        glyphs: &[u16],
        pos: &mut [Point],
        top: f32,
        bottom: f32,
        paint: Option<&Paint>,
        vec: Option<&mut Vec<f32>>,
    ) -> usize {
        let len = vec.as_ref().map(|v| v.len()).unwrap_or_default();
        unsafe {
            sk_font_get_intercepts(
                self.as_ptr_mut(),
                glyphs.as_ptr(),
                glyphs.len() as _,
                pos.as_ptr() as *const sk_point_t,
                top,
                bottom,
                paint.map(Paint::as_ptr).unwrap_or(null()),
                vec.map(|v| v as *mut Vec<f32> as *mut f32)
                    .unwrap_or(std::ptr::null_mut()),
                len,
            )
        }
    }
    /// If the glyph can be represented by a path, we will set path to the glyph outline and return true.
    /// If the glyph is a bitmap, then we return false.
    pub fn get_path(&self, glyph: u16, path: &mut SkiaPath) -> bool {
        unsafe { sk_font_get_path(self.as_ptr(), glyph, path.as_ptr_mut()) }
    }

    /*
    pub fn sk_font_get_paths(
        font: *const sk_font_t,
        glyphs: *mut u16,
        count: ::std::os::raw::c_int,
        glyphPathProc: sk_glyph_path_proc,
        context: *mut ::std::os::raw::c_void,
    ); */
    /// returns recommended spacing between lines
    /// If metrics is Some, then it sets font metrics too.
    /// doesn't account for "paint" variables like path effect.
    pub fn get_metrics(&self, metrics: Option<&mut FontMetrics>) -> f32 {
        unsafe {
            sk_font_get_metrics(
                self.as_ptr(),
                metrics.map(|m| m.as_ptr_mut()).unwrap_or(null_mut()),
            )
        }
    }
    /*
    pub fn sk_text_utils_get_path(
        text: *const ::std::os::raw::c_void,
        length: usize,
        encoding: sk_text_encoding_t,
        x: f32,
        y: f32,
        font: *const sk_font_t,
        path: *mut sk_path_t,
    );
    pub fn sk_text_utils_get_pos_path(
        text: *const ::std::os::raw::c_void,
        length: usize,
        encoding: sk_text_encoding_t,
        pos: *const sk_point_t,
        font: *const sk_font_t,
        path: *mut sk_path_t,
    );
     */
}
