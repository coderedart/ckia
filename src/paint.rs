use ckia_sys::*;

use crate::{filter::MaskFilter, shader::Shader, BlendMode, Color, Color4f, ColorSpace};

pub type PaintStyle = sk_paint_style_t;
pub type StrokeCap = sk_stroke_cap_t;
pub type StrokeJoin = sk_stroke_join_t;
pub type BlurStyle = sk_blurstyle_t;

crate::opaque_unique!(Paint, sk_paint_t, sk_paint_delete);

impl Clone for Paint {
    fn clone(&self) -> Self {
        let inner = unsafe { sk_paint_clone(self.inner) };
        assert!(!inner.is_null());
        Self { inner }
    }
}

impl Paint {
    pub fn new() -> Self {
        let inner = unsafe { sk_paint_new() };
        assert!(!inner.is_null());
        Self { inner }
    }
    pub fn reset(&mut self) {
        unsafe {
            sk_paint_reset(self.inner);
        }
    }
    pub fn is_antialias(&self) -> bool {
        unsafe { sk_paint_is_antialias(self.inner) }
    }
    pub fn set_antialias(&mut self, antialias: bool) {
        unsafe {
            sk_paint_set_antialias(self.inner, antialias);
        }
    }
    pub fn get_color(&self) -> Color {
        unsafe { Color(sk_paint_get_color(self.inner)) }
    }
    pub fn get_color4f(&self) -> Color4f {
        let mut color = Color4f::default();
        unsafe {
            sk_paint_get_color4f(self.inner, &mut color.0 as _);
        }
        color
    }
    pub fn set_color(&mut self, color: Color) {
        unsafe { sk_paint_set_color(self.inner, color.0) }
    }
    pub fn set_color4f(&mut self, mut color: Color4f, space: &ColorSpace) {
        unsafe {
            sk_paint_set_color4f(self.inner, &mut color.0 as _, space.inner);
        }
    }
    pub fn get_style(&self) -> PaintStyle {
        unsafe { sk_paint_get_style(self.inner) }
    }
    pub fn set_style(&mut self, style: PaintStyle) {
        unsafe { sk_paint_set_style(self.inner, style) }
    }
    pub fn get_stroke_width(&self) -> f32 {
        unsafe { sk_paint_get_stroke_width(self.inner) }
    }
    pub fn set_stroke_width(&mut self, width: f32) {
        unsafe { sk_paint_set_stroke_width(self.inner, width) }
    }
    pub fn get_stroke_miter(&self) -> f32 {
        unsafe { sk_paint_get_stroke_miter(self.inner) }
    }
    pub fn set_stroke_miter(&mut self, width: f32) {
        unsafe { sk_paint_set_stroke_miter(self.inner, width) }
    }
    pub fn get_stroke_cap(&self) -> StrokeCap {
        unsafe { sk_paint_get_stroke_cap(self.inner) }
    }
    pub fn set_stroke_cap(&mut self, cap: StrokeCap) {
        unsafe { sk_paint_set_stroke_cap(self.inner, cap) }
    }
    pub fn get_stroke_join(&self) -> StrokeJoin {
        unsafe { sk_paint_get_stroke_join(self.inner) }
    }
    pub fn set_stroke_join(&mut self, stroke_join: StrokeJoin) {
        unsafe { sk_paint_set_stroke_join(self.inner, stroke_join) }
    }
    pub fn set_shader(&mut self, shader: &Shader) {
        unsafe {
            sk_paint_set_shader(self.inner, shader.inner);
        }
    }
    pub fn set_maskfilter(&mut self, filter: &MaskFilter) {
        unsafe {
            sk_paint_set_maskfilter(self.inner, filter.inner);
        }
    }
    pub fn set_blendmode(&mut self, mode: BlendMode) {
        unsafe {
            sk_paint_set_blendmode(self.inner, mode);
        }
    }
    pub fn is_dither(&self) -> bool {
        unsafe { sk_paint_is_dither(self.inner) }
    }
    pub fn set_dither(&mut self, dither: bool) {
        unsafe {
            sk_paint_set_dither(self.inner, dither);
        }
    }
    pub fn get_shader(&mut self) -> Option<Shader> {
        unsafe { Shader::from_owned_ptr(sk_paint_get_shader(self.inner)) }
    }
    pub fn get_maskfilter(&mut self) -> Option<MaskFilter> {
        unsafe { MaskFilter::from_owned_ptr(sk_paint_get_maskfilter(self.inner)) }
    }
    /*
       pub fn sk_paint_get_maskfilter(arg1: *mut sk_paint_t) -> *mut sk_maskfilter_t;
       pub fn sk_paint_set_colorfilter(arg1: *mut sk_paint_t, arg2: *mut sk_colorfilter_t);
       pub fn sk_paint_get_colorfilter(arg1: *mut sk_paint_t) -> *mut sk_colorfilter_t;
       pub fn sk_paint_set_imagefilter(arg1: *mut sk_paint_t, arg2: *mut sk_imagefilter_t);
       pub fn sk_paint_get_imagefilter(arg1: *mut sk_paint_t) -> *mut sk_imagefilter_t;
       pub fn sk_paint_get_blendmode(arg1: *mut sk_paint_t) -> sk_blendmode_t;
       pub fn sk_paint_get_path_effect(cpaint: *mut sk_paint_t) -> *mut sk_path_effect_t;
       pub fn sk_paint_set_path_effect(cpaint: *mut sk_paint_t, effect: *mut sk_path_effect_t);
       pub fn sk_paint_get_fill_path(
           cpaint: *const sk_paint_t,
           src: *const sk_path_t,
           dst: *mut sk_path_t,
           cullRect: *const sk_rect_t,
           cmatrix: *const sk_matrix_t,
       ) -> bool;
    */
}
