use crate::bindings::*;
use crate::{SkiaPtr, SkiaPtrMut};

use crate::{
    color::ColorSpace,
    filter::{ColorFilter, ImageFilter, MaskFilter},
    path::SkiaPath,
    path_effect::PathEffect,
    shader::Shader,
    BlendMode, Color, Color4f, Matrix, PaintStyle, Rect, SkiaOptPtrMut, StrokeCap, StrokeJoin,
};

crate::skia_wrapper!(unique, Paint, sk_paint_t, sk_paint_delete);

impl Clone for Paint {
    fn clone(&self) -> Self {
        unsafe { Self::from_owned_ptr(sk_paint_clone(self.inner)) }
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
            sk_paint_reset(self.as_ptr_mut());
        }
    }
    pub fn is_antialias(&self) -> bool {
        unsafe { sk_paint_is_antialias(self.as_ptr()) }
    }
    pub fn set_antialias(&mut self, antialias: bool) {
        unsafe {
            sk_paint_set_antialias(self.as_ptr_mut(), antialias);
        }
    }
    pub fn get_alpha_f(&self) -> f32 {
        unsafe { sk_paint_get_alpha_f(self.as_ptr()) }
    }
    pub fn set_alpha_f(&mut self, alpha: f32) {
        unsafe { sk_paint_set_alpha_f(self.as_ptr_mut(), alpha) }
    }
    pub fn get_color(&self) -> Color {
        unsafe { Color(sk_paint_get_color(self.as_ptr())) }
    }
    pub fn get_color4f(&self) -> Color4f {
        let mut color = Color4f::default();
        unsafe {
            sk_paint_get_color4f(self.as_ptr(), color.as_ptr_mut());
        }
        color
    }
    pub fn set_color(&mut self, color: Color) {
        unsafe { sk_paint_set_color(self.as_ptr_mut(), color.0) }
    }
    pub fn set_color4f(&mut self, mut color: Color4f, space: &ColorSpace) {
        unsafe {
            sk_paint_set_color4f(self.as_ptr_mut(), color.as_ptr_mut(), space.inner);
        }
    }
    pub fn get_style(&self) -> PaintStyle {
        unsafe { sk_paint_get_style(self.as_ptr()) }
    }
    pub fn set_style(&mut self, style: PaintStyle) {
        unsafe { sk_paint_set_style(self.as_ptr_mut(), style) }
    }
    pub fn get_stroke_width(&self) -> f32 {
        unsafe { sk_paint_get_stroke_width(self.as_ptr()) }
    }
    pub fn set_stroke_width(&mut self, width: f32) {
        unsafe { sk_paint_set_stroke_width(self.as_ptr_mut(), width) }
    }
    pub fn get_stroke_miter(&self) -> f32 {
        unsafe { sk_paint_get_stroke_miter(self.as_ptr()) }
    }
    pub fn set_stroke_miter(&mut self, width: f32) {
        unsafe { sk_paint_set_stroke_miter(self.as_ptr_mut(), width) }
    }
    pub fn get_stroke_cap(&self) -> StrokeCap {
        unsafe { sk_paint_get_stroke_cap(self.as_ptr()) }
    }
    pub fn set_stroke_cap(&mut self, cap: StrokeCap) {
        unsafe { sk_paint_set_stroke_cap(self.as_ptr_mut(), cap) }
    }
    pub fn get_stroke_join(&self) -> StrokeJoin {
        unsafe { sk_paint_get_stroke_join(self.as_ptr()) }
    }
    pub fn set_stroke_join(&mut self, stroke_join: StrokeJoin) {
        unsafe { sk_paint_set_stroke_join(self.as_ptr_mut(), stroke_join) }
    }
    pub fn set_shader(&mut self, shader: Option<&mut Shader>) {
        unsafe {
            sk_paint_set_shader(self.as_ptr_mut(), shader.or_null_mut());
        }
    }
    pub fn set_maskfilter(&mut self, filter: Option<&mut MaskFilter>) {
        unsafe {
            sk_paint_set_maskfilter(self.as_ptr_mut(), filter.or_null_mut());
        }
    }
    pub fn set_blendmode(&mut self, mode: BlendMode) {
        unsafe {
            sk_paint_set_blendmode(self.as_ptr_mut(), mode);
        }
    }
    pub fn is_dither(&self) -> bool {
        unsafe { sk_paint_is_dither(self.as_ptr()) }
    }
    pub fn set_dither(&mut self, dither: bool) {
        unsafe {
            sk_paint_set_dither(self.as_ptr_mut(), dither);
        }
    }
    pub fn get_shader(&mut self) -> Option<Shader> {
        unsafe { Shader::try_from_owned_ptr(sk_paint_get_shader(self.as_ptr_mut())) }
    }
    pub fn get_maskfilter(&mut self) -> Option<MaskFilter> {
        unsafe { MaskFilter::try_from_owned_ptr(sk_paint_get_maskfilter(self.as_ptr_mut())) }
    }
    pub fn set_colorfilter(&mut self, filter: Option<&mut ColorFilter>) {
        unsafe { sk_paint_set_colorfilter(self.as_ptr_mut(), filter.or_null_mut()) }
    }
    pub fn get_colorfilter(&mut self) -> Option<ColorFilter> {
        unsafe { ColorFilter::try_from_owned_ptr(sk_paint_get_colorfilter(self.as_ptr_mut())) }
    }
    pub fn set_image_filter(&mut self, filter: Option<&mut ImageFilter>) {
        unsafe { sk_paint_set_imagefilter(self.as_ptr_mut(), filter.or_null_mut()) }
    }
    pub fn get_imagefilter(&mut self) -> Option<ImageFilter> {
        unsafe { ImageFilter::try_from_owned_ptr(sk_paint_get_imagefilter(self.as_ptr_mut())) }
    }
    pub fn get_blendmode(&mut self) -> BlendMode {
        unsafe { sk_paint_get_blendmode(self.as_ptr_mut()) }
    }
    pub fn get_path_effect(&mut self) -> Option<PathEffect> {
        unsafe { PathEffect::try_from_owned_ptr(sk_paint_get_path_effect(self.as_ptr_mut())) }
    }
    pub fn set_path_effect(&mut self, path_effect: Option<&mut PathEffect>) {
        unsafe { sk_paint_set_path_effect(self.as_ptr_mut(), path_effect.or_null_mut()) }
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
                self.as_ptr(),
                src.inner,
                dst.inner,
                cull_rect.as_ptr(),
                matrix.as_ptr(),
            )
        }
    }
}
