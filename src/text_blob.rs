use ckia_sys::*;

use crate::{Rect, SkiaPointer};

crate::skia_wrapper!(
    nvrefcnt,
    TextBlob,
    sk_textblob_t,
    sk_textblob_unref,
    sk_textblob_ref
);

impl TextBlob {
    pub fn get_unique_id(&self) -> u32 {
        unsafe { sk_textblob_get_unique_id(self.as_ptr()) }
    }
    pub fn get_bounds(&self) -> Rect {
        let mut rect = Rect::default();
        unsafe { sk_textblob_get_bounds(self.as_ptr(), rect.as_ptr_mut()) };
        rect
    }

    /*
    pub fn sk_textblob_get_intercepts(
        blob: *const sk_textblob_t,
        bounds: *const f32,
        intervals: *mut f32,
        paint: *const sk_paint_t,
    ) -> ::std::os::raw::c_int;
    */
}

crate::skia_wrapper!(
    unique,
    TextBlobBuilder,
    sk_textblob_builder_t,
    sk_textblob_builder_delete
);
impl Default for TextBlobBuilder {
    fn default() -> Self {
        unsafe { Self::from_owned_ptr(sk_textblob_builder_new()) }
    }
}
impl TextBlobBuilder {
    pub fn make(&mut self) -> Option<TextBlob> {
        unsafe { TextBlob::try_from_owned_ptr(sk_textblob_builder_make(self.as_ptr_mut())) }
    }

    /*
    pub fn sk_textblob_builder_alloc_run(
        builder: *mut sk_textblob_builder_t,
        font: *const sk_font_t,
        count: ::std::os::raw::c_int,
        x: f32,
        y: f32,
        bounds: *const sk_rect_t,
        runbuffer: *mut sk_textblob_builder_runbuffer_t,
    );
    pub fn sk_textblob_builder_alloc_run_pos_h(
        builder: *mut sk_textblob_builder_t,
        font: *const sk_font_t,
        count: ::std::os::raw::c_int,
        y: f32,
        bounds: *const sk_rect_t,
        runbuffer: *mut sk_textblob_builder_runbuffer_t,
    );
    pub fn sk_textblob_builder_alloc_run_pos(
        builder: *mut sk_textblob_builder_t,
        font: *const sk_font_t,
        count: ::std::os::raw::c_int,
        bounds: *const sk_rect_t,
        runbuffer: *mut sk_textblob_builder_runbuffer_t,
    );
    pub fn sk_textblob_builder_alloc_run_rsxform(
        builder: *mut sk_textblob_builder_t,
        font: *const sk_font_t,
        count: ::std::os::raw::c_int,
        bounds: *const sk_rect_t,
        runbuffer: *mut sk_textblob_builder_runbuffer_t,
    );
    pub fn sk_textblob_builder_alloc_run_text(
        builder: *mut sk_textblob_builder_t,
        font: *const sk_font_t,
        count: ::std::os::raw::c_int,
        x: f32,
        y: f32,
        textByteCount: ::std::os::raw::c_int,
        bounds: *const sk_rect_t,
        runbuffer: *mut sk_textblob_builder_runbuffer_t,
    );
    pub fn sk_textblob_builder_alloc_run_text_pos_h(
        builder: *mut sk_textblob_builder_t,
        font: *const sk_font_t,
        count: ::std::os::raw::c_int,
        y: f32,
        textByteCount: ::std::os::raw::c_int,
        bounds: *const sk_rect_t,
        runbuffer: *mut sk_textblob_builder_runbuffer_t,
    );
    pub fn sk_textblob_builder_alloc_run_text_pos(
        builder: *mut sk_textblob_builder_t,
        font: *const sk_font_t,
        count: ::std::os::raw::c_int,
        textByteCount: ::std::os::raw::c_int,
        bounds: *const sk_rect_t,
        runbuffer: *mut sk_textblob_builder_runbuffer_t,
    );
    pub fn sk_textblob_builder_alloc_run_text_rsxform(
        builder: *mut sk_textblob_builder_t,
        font: *const sk_font_t,
        count: ::std::os::raw::c_int,
        textByteCount: ::std::os::raw::c_int,
        bounds: *const sk_rect_t,
        runbuffer: *mut sk_textblob_builder_runbuffer_t,
    ); */
}
