use ckia_sys::*;

use crate::{
    color::ColorSpace, filter::ColorFilter, skia_wrapper, BlendMode, Color, Color4f, Matrix, Point,
    ShaderTileMode, SkiaPointer,
};

skia_wrapper!(refcnt, Shader, sk_shader_t, sk_shader_unref, sk_shader_ref);

impl Shader {
    /*
    pub fn sk_shader_with_local_matrix(
        shader: *const sk_shader_t,
        localMatrix: *const sk_matrix_t,
    ) -> *mut sk_shader_t;
    */
    pub fn with_color_filter(&self, filter: &ColorFilter) -> Self {
        unsafe { Self::from_owned_ptr(sk_shader_with_color_filter(self.inner, filter.inner)) }
    }
    pub fn new_empty() -> Self {
        unsafe { Self::from_owned_ptr(sk_shader_new_empty()) }
    }
    pub fn new_color(color: Color) -> Self {
        unsafe { Self::from_owned_ptr(sk_shader_new_color(color.0)) }
    }
    pub fn new_color4f(color: &Color4f, space: &ColorSpace) -> Self {
        unsafe { Self::from_owned_ptr(sk_shader_new_color4f(color.as_ptr(), space.inner)) }
    }
    pub fn new_blend(mode: BlendMode, dst: &Self, src: &Self) -> Self {
        unsafe { Self::from_owned_ptr(sk_shader_new_blend(mode, dst.inner, src.inner)) }
    }
    pub fn new_linear_gradient(
        points: &[Point; 2],
        colors: &[Color],
        color_positions: Option<&[f32]>,
        tile_mode: ShaderTileMode,
        matrix: Option<Matrix>,
    ) -> Self {
        if let Some(color_positions) = color_positions {
            assert_eq!(colors.len(), color_positions.len());
        }
        unsafe {
            Self::from_owned_ptr(sk_shader_new_linear_gradient(
                points.as_ptr() as _,
                colors.as_ptr() as _,
                color_positions
                    .map(|c| c.as_ptr())
                    .unwrap_or(std::ptr::null()),
                colors.len() as _,
                tile_mode,
                matrix.map(|m| m.as_ptr()).unwrap_or(std::ptr::null()),
            ))
        }
    }
    pub fn new_radial_gradient(
        center: Point,
        radius: f32,
        colors: &[Color],
        color_positions: Option<&[f32]>,
        tile_mode: ShaderTileMode,
        matrix: Option<Matrix>,
    ) -> Self {
        if let Some(color_positions) = color_positions {
            assert_eq!(colors.len(), color_positions.len());
        }
        unsafe {
            Self::from_owned_ptr(sk_shader_new_radial_gradient(
                center.as_ptr(),
                radius,
                colors.as_ptr() as _,
                color_positions
                    .map(|c| c.as_ptr())
                    .unwrap_or(std::ptr::null()),
                colors.len() as _,
                tile_mode,
                matrix.map(|m| m.as_ptr()).unwrap_or(std::ptr::null()),
            ))
        }
    }
    /*
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
