use ckia_sys::*;

use crate::{
    color::ColorSpace,
    filter::{ColorFilter, ImageFilter, MaskFilter},
    path::SkiaPath,
    path_effect::PathEffect,
    shader::Shader,
    BlendMode, Color, Color4f, Matrix, PaintStyle, Rect, SkiaPointer, StrokeCap, StrokeJoin,
};

crate::skia_wrapper!(unique, Paint, sk_paint_t, sk_paint_delete);

impl Clone for Paint {
    fn clone(&self) -> Self {
        let inner = unsafe { sk_paint_clone(self.inner) };
        assert!(!inner.is_null());
        Self { inner }
    }
}

impl Default for Paint {
    fn default() -> Self {
        unsafe { Self::from_owned_ptr(sk_paint_new()) }
    }
}
impl Paint {
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
        unsafe { Shader::try_from_owned_ptr(sk_paint_get_shader(self.inner)) }
    }
    pub fn get_maskfilter(&mut self) -> Option<MaskFilter> {
        unsafe { MaskFilter::try_from_owned_ptr(sk_paint_get_maskfilter(self.inner)) }
    }
    pub fn set_colorfilter(&mut self, filter: &ColorFilter) {
        unsafe { sk_paint_set_colorfilter(self.inner, filter.inner) }
    }
    pub fn get_colorfilter(&mut self) -> Option<ColorFilter> {
        unsafe { ColorFilter::try_from_owned_ptr(sk_paint_get_colorfilter(self.inner)) }
    }
    pub fn set_image_filter(&mut self, filter: &ImageFilter) {
        unsafe { sk_paint_set_imagefilter(self.inner, filter.inner) }
    }
    pub fn get_imagefilter(&self) -> Option<ImageFilter> {
        unsafe { ImageFilter::try_from_owned_ptr(sk_paint_get_imagefilter(self.inner)) }
    }
    pub fn get_blendmode(&self) -> BlendMode {
        unsafe { sk_paint_get_blendmode(self.inner) }
    }
    pub fn get_path_effect(&mut self) -> Option<PathEffect> {
        unsafe { PathEffect::try_from_owned_ptr(sk_paint_get_path_effect(self.inner)) }
    }
    pub fn set_path_effect(&mut self, path_effect: &mut PathEffect) {
        unsafe { sk_paint_set_path_effect(self.inner, path_effect.inner) }
    }
    /// if success, dst has the result and return value is true. if not, then dst remains untouched and we return false.
    #[must_use]
    pub fn get_fill_path(
        &self,
        src: &SkiaPath,
        dst: &mut SkiaPath,
        cull_rect: &Rect,
        matrix: &Matrix,
    ) -> bool {
        unsafe {
            sk_paint_get_fill_path(
                self.inner,
                src.inner,
                dst.inner,
                cull_rect.as_ptr(),
                matrix.as_ptr(),
            )
        }
    }
}
