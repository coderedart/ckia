use ckia_sys::*;

use crate::{paint::BlurStyle, shader::Shader, opaque_shared};

opaque_shared!(MaskFilter, sk_maskfilter_t, sk_maskfilter_unref, sk_maskfilter_ref);

impl MaskFilter {
    pub fn new_blur(blur: BlurStyle, sigma: f32) -> Self {
        let inner = unsafe { sk_maskfilter_new_blur(blur, sigma) };
        assert!(!inner.is_null());
        Self { inner }
    }
    pub fn new_blur_with_flags(blur: BlurStyle, sigma: f32, respect_c_t_m: bool) -> Self {
        let inner = unsafe { sk_maskfilter_new_blur_with_flags(blur, sigma, respect_c_t_m) };
        assert!(!inner.is_null());
        Self { inner }
    }
    pub fn new_table(table: &[u8]) -> Self {
        assert!(table.len() >= 256);
        let inner = unsafe { sk_maskfilter_new_table(table.as_ptr()) };
        assert!(!inner.is_null());
        Self { inner }
    }
    pub fn new_gamma(gamma: f32) -> Self {
        let inner = unsafe { sk_maskfilter_new_gamma(gamma) };
        assert!(!inner.is_null());
        Self { inner }
    }
    pub fn new_clip(min: u8, max: u8) -> Self {
        let inner = unsafe { sk_maskfilter_new_clip(min, max) };
        assert!(!inner.is_null());
        Self { inner }
    }
    pub fn new_shader(shader: &Shader) -> Self {
        let inner = unsafe { sk_maskfilter_new_shader(shader.inner) };
        assert!(!inner.is_null());
        Self { inner }
    }
}

opaque_shared!(ColorFilter, sk_colorfilter_t, sk_colorfilter_unref);
impl ColorFilter {

    /*
    pub fn sk_colorfilter_new_mode(c: sk_color_t, mode: sk_blendmode_t) -> *mut sk_colorfilter_t;
    pub fn sk_colorfilter_new_lighting(mul: sk_color_t, add: sk_color_t) -> *mut sk_colorfilter_t;
    pub fn sk_colorfilter_new_compose(
        outer: *mut sk_colorfilter_t,
        inner: *mut sk_colorfilter_t,
    ) -> *mut sk_colorfilter_t;
    pub fn sk_colorfilter_new_color_matrix(array: *const f32) -> *mut sk_colorfilter_t;
    pub fn sk_colorfilter_new_luma_color() -> *mut sk_colorfilter_t;
    pub fn sk_colorfilter_new_high_contrast(
        config: *const sk_highcontrastconfig_t,
    ) -> *mut sk_colorfilter_t;
    pub fn sk_colorfilter_new_table(table: *const u8) -> *mut sk_colorfilter_t;
    pub fn sk_colorfilter_new_table_argb(
        tableA: *const u8,
        tableR: *const u8,
        tableG: *const u8,
        tableB: *const u8,
    ) -> *mut sk_colorfilter_t; */
}