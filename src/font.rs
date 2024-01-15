use ckia_sys::*;

crate::opaque_unique!(Font, sk_font_t, sk_font_delete);

impl Default for Font {
    fn default() -> Self {
        unsafe { Self::from_owned_ptr(sk_font_new()) }
    }
}
impl Font {
    /*
    pub fn sk_font_new_with_values(
        typeface: *mut sk_typeface_t,
        size: f32,
        scaleX: f32,
        skewX: f32,
    ) -> *mut sk_font_t;
    pub fn sk_font_delete(font: *mut sk_font_t);
    pub fn sk_font_is_force_auto_hinting(font: *const sk_font_t) -> bool;
    pub fn sk_font_set_force_auto_hinting(font: *mut sk_font_t, value: bool);
    pub fn sk_font_is_embedded_bitmaps(font: *const sk_font_t) -> bool;
    pub fn sk_font_set_embedded_bitmaps(font: *mut sk_font_t, value: bool);
    pub fn sk_font_is_subpixel(font: *const sk_font_t) -> bool;
    pub fn sk_font_set_subpixel(font: *mut sk_font_t, value: bool);
    pub fn sk_font_is_linear_metrics(font: *const sk_font_t) -> bool;
    pub fn sk_font_set_linear_metrics(font: *mut sk_font_t, value: bool);
    pub fn sk_font_is_embolden(font: *const sk_font_t) -> bool;
    pub fn sk_font_set_embolden(font: *mut sk_font_t, value: bool);
    pub fn sk_font_is_baseline_snap(font: *const sk_font_t) -> bool;
    pub fn sk_font_set_baseline_snap(font: *mut sk_font_t, value: bool);
    pub fn sk_font_get_edging(font: *const sk_font_t) -> sk_font_edging_t;
    pub fn sk_font_set_edging(font: *mut sk_font_t, value: sk_font_edging_t);
    pub fn sk_font_get_hinting(font: *const sk_font_t) -> sk_font_hinting_t;
    pub fn sk_font_set_hinting(font: *mut sk_font_t, value: sk_font_hinting_t);
    pub fn sk_font_get_typeface(font: *const sk_font_t) -> *mut sk_typeface_t;
    pub fn sk_font_set_typeface(font: *mut sk_font_t, value: *mut sk_typeface_t);
    pub fn sk_font_get_size(font: *const sk_font_t) -> f32;
    pub fn sk_font_set_size(font: *mut sk_font_t, value: f32);
    pub fn sk_font_get_scale_x(font: *const sk_font_t) -> f32;
    pub fn sk_font_set_scale_x(font: *mut sk_font_t, value: f32);
    pub fn sk_font_get_skew_x(font: *const sk_font_t) -> f32;
    pub fn sk_font_set_skew_x(font: *mut sk_font_t, value: f32);
    pub fn sk_font_text_to_glyphs(
        font: *const sk_font_t,
        text: *const ::std::os::raw::c_void,
        byteLength: usize,
        encoding: sk_text_encoding_t,
        glyphs: *mut u16,
        maxGlyphCount: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn sk_font_unichar_to_glyph(font: *const sk_font_t, uni: i32) -> u16;
    pub fn sk_font_unichars_to_glyphs(
        font: *const sk_font_t,
        uni: *const i32,
        count: ::std::os::raw::c_int,
        glyphs: *mut u16,
    );
    pub fn sk_font_measure_text(
        font: *const sk_font_t,
        text: *const ::std::os::raw::c_void,
        byteLength: usize,
        encoding: sk_text_encoding_t,
        bounds: *mut sk_rect_t,
        paint: *const sk_paint_t,
    ) -> f32;
    pub fn sk_font_measure_text_no_return(
        font: *const sk_font_t,
        text: *const ::std::os::raw::c_void,
        byteLength: usize,
        encoding: sk_text_encoding_t,
        bounds: *mut sk_rect_t,
        paint: *const sk_paint_t,
        measuredWidth: *mut f32,
    );
    pub fn sk_font_break_text(
        font: *const sk_font_t,
        text: *const ::std::os::raw::c_void,
        byteLength: usize,
        encoding: sk_text_encoding_t,
        maxWidth: f32,
        measuredWidth: *mut f32,
        paint: *const sk_paint_t,
    ) -> usize;
    pub fn sk_font_get_widths_bounds(
        font: *const sk_font_t,
        glyphs: *const u16,
        count: ::std::os::raw::c_int,
        widths: *mut f32,
        bounds: *mut sk_rect_t,
        paint: *const sk_paint_t,
    );
    pub fn sk_font_get_pos(
        font: *const sk_font_t,
        glyphs: *const u16,
        count: ::std::os::raw::c_int,
        pos: *mut sk_point_t,
        origin: *mut sk_point_t,
    );
    pub fn sk_font_get_xpos(
        font: *const sk_font_t,
        glyphs: *const u16,
        count: ::std::os::raw::c_int,
        xpos: *mut f32,
        origin: f32,
    );
    pub fn sk_font_get_path(font: *const sk_font_t, glyph: u16, path: *mut sk_path_t) -> bool;
    pub fn sk_font_get_paths(
        font: *const sk_font_t,
        glyphs: *mut u16,
        count: ::std::os::raw::c_int,
        glyphPathProc: sk_glyph_path_proc,
        context: *mut ::std::os::raw::c_void,
    );
    pub fn sk_font_get_metrics(font: *const sk_font_t, metrics: *mut sk_fontmetrics_t) -> f32;
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
