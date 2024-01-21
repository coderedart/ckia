use ckia_sys::*;

use crate::{color::ColorSpace, filter::ColorFilter, skia_wrapper, BlendMode, Color, Color4f};

skia_wrapper!(refcnt, Shader, sk_shader_t, sk_shader_unref, sk_shader_ref);

impl Shader {
    /*
    pub fn sk_shader_with_local_matrix(
        shader: *const sk_shader_t,
        localMatrix: *const sk_matrix_t,
    ) -> *mut sk_shader_t;
    */
    pub fn with_color_filter(&self, filter: &ColorFilter) -> Self {
        let inner = unsafe { sk_shader_with_color_filter(self.inner, filter.inner) };
        assert!(!inner.is_null());
        Self { inner }
    }
    pub fn new_empty() -> Self {
        let inner = unsafe { sk_shader_new_empty() };
        assert!(!inner.is_null());
        Self { inner }
    }
    pub fn new_color(color: Color) -> Self {
        let inner = unsafe { sk_shader_new_color(color.0) };
        assert!(!inner.is_null());
        Self { inner }
    }
    pub fn new_color4f(color: &Color4f, space: &ColorSpace) -> Self {
        let inner = unsafe { sk_shader_new_color4f(&color.0 as _, space.inner) };
        assert!(!inner.is_null());
        Self { inner }
    }
    pub fn new_blend(mode: BlendMode, dst: &Self, src: &Self) -> Self {
        let inner = unsafe { sk_shader_new_blend(mode, dst.inner, src.inner) };
        assert!(!inner.is_null());
        Self { inner }
    }
    /*
    pub fn sk_shader_new_linear_gradient(
        points: *const sk_point_t,
        colors: *const sk_color_t,
        colorPos: *const f32,
        colorCount: ::std::os::raw::c_int,
        tileMode: sk_shader_tilemode_t,
        localMatrix: *const sk_matrix_t,
    ) -> *mut sk_shader_t;
    pub fn sk_shader_new_linear_gradient_color4f(
        points: *const sk_point_t,
        colors: *const sk_color4f_t,
        colorspace: *const sk_colorspace_t,
        colorPos: *const f32,
        colorCount: ::std::os::raw::c_int,
        tileMode: sk_shader_tilemode_t,
        localMatrix: *const sk_matrix_t,
    ) -> *mut sk_shader_t;
    pub fn sk_shader_new_radial_gradient(
        center: *const sk_point_t,
        radius: f32,
        colors: *const sk_color_t,
        colorPos: *const f32,
        colorCount: ::std::os::raw::c_int,
        tileMode: sk_shader_tilemode_t,
        localMatrix: *const sk_matrix_t,
    ) -> *mut sk_shader_t;
    pub fn sk_shader_new_radial_gradient_color4f(
        center: *const sk_point_t,
        radius: f32,
        colors: *const sk_color4f_t,
        colorspace: *const sk_colorspace_t,
        colorPos: *const f32,
        colorCount: ::std::os::raw::c_int,
        tileMode: sk_shader_tilemode_t,
        localMatrix: *const sk_matrix_t,
    ) -> *mut sk_shader_t;
    pub fn sk_shader_new_sweep_gradient(
        center: *const sk_point_t,
        colors: *const sk_color_t,
        colorPos: *const f32,
        colorCount: ::std::os::raw::c_int,
        tileMode: sk_shader_tilemode_t,
        startAngle: f32,
        endAngle: f32,
        localMatrix: *const sk_matrix_t,
    ) -> *mut sk_shader_t;
    pub fn sk_shader_new_sweep_gradient_color4f(
        center: *const sk_point_t,
        colors: *const sk_color4f_t,
        colorspace: *const sk_colorspace_t,
        colorPos: *const f32,
        colorCount: ::std::os::raw::c_int,
        tileMode: sk_shader_tilemode_t,
        startAngle: f32,
        endAngle: f32,
        localMatrix: *const sk_matrix_t,
    ) -> *mut sk_shader_t;
    pub fn sk_shader_new_two_point_conical_gradient(
        start: *const sk_point_t,
        startRadius: f32,
        end: *const sk_point_t,
        endRadius: f32,
        colors: *const sk_color_t,
        colorPos: *const f32,
        colorCount: ::std::os::raw::c_int,
        tileMode: sk_shader_tilemode_t,
        localMatrix: *const sk_matrix_t,
    ) -> *mut sk_shader_t;
    pub fn sk_shader_new_two_point_conical_gradient_color4f(
        start: *const sk_point_t,
        startRadius: f32,
        end: *const sk_point_t,
        endRadius: f32,
        colors: *const sk_color4f_t,
        colorspace: *const sk_colorspace_t,
        colorPos: *const f32,
        colorCount: ::std::os::raw::c_int,
        tileMode: sk_shader_tilemode_t,
        localMatrix: *const sk_matrix_t,
    ) -> *mut sk_shader_t;
    pub fn sk_shader_new_perlin_noise_fractal_noise(
        baseFrequencyX: f32,
        baseFrequencyY: f32,
        numOctaves: ::std::os::raw::c_int,
        seed: f32,
        tileSize: *const sk_isize_t,
    ) -> *mut sk_shader_t;
    pub fn sk_shader_new_perlin_noise_turbulence(
        baseFrequencyX: f32,
        baseFrequencyY: f32,
        numOctaves: ::std::os::raw::c_int,
        seed: f32,
        tileSize: *const sk_isize_t,
    ) -> *mut sk_shader_t; */
}
