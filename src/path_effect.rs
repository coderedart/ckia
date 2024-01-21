use ckia_sys::*;

use crate::{path::SkiaPath, Matrix, PathEffect1DStyle, PathEffectTrimMode, SkiaPointer};

crate::skia_wrapper!(refcnt, PathEffect, sk_path_effect_t, sk_path_effect_unref);

impl PathEffect {
    pub fn create_compose(&self, inner: &Self) -> Self {
        unsafe { Self::from_owned_ptr(sk_path_effect_create_compose(self.inner, inner.inner)) }
    }
    pub fn create_sum(&self, second: &Self) -> Self {
        unsafe { Self::from_owned_ptr(sk_path_effect_create_sum(self.inner, second.inner)) }
    }
    pub fn discrete(seg_length: f32, deviation: f32, seed_assist: u32) -> Self {
        unsafe {
            Self::from_owned_ptr(sk_path_effect_create_discrete(
                seg_length,
                deviation,
                seed_assist,
            ))
        }
    }
    pub fn create_corner(radius: f32) -> Self {
        unsafe { Self::from_owned_ptr(sk_path_effect_create_corner(radius)) }
    }
    pub fn create_1d_path(
        path: &SkiaPath,
        advance: f32,
        phase: f32,
        style: PathEffect1DStyle,
    ) -> Self {
        unsafe {
            Self::from_owned_ptr(sk_path_effect_create_1d_path(
                path.inner, advance, phase, style,
            ))
        }
    }
    pub fn create_2d_line(width: f32, matrix: &Matrix) -> Self {
        unsafe { Self::from_owned_ptr(sk_path_effect_create_2d_line(width, matrix.as_ptr())) }
    }
    pub fn create_2d_path(matrix: &Matrix, path: &SkiaPath) -> Self {
        unsafe { Self::from_owned_ptr(sk_path_effect_create_2d_path(matrix.as_ptr(), path.inner)) }
    }
    pub fn create_dash(intervals: &[f32], phase: f32) -> Self {
        assert!(intervals.len() >= 2 && intervals.len() % 2 == 0);
        unsafe {
            Self::from_owned_ptr(sk_path_effect_create_dash(
                intervals.as_ptr(),
                intervals.len() as _,
                phase,
            ))
        }
    }
    pub fn create_trim(start: f32, stop: f32, mode: PathEffectTrimMode) -> Self {
        unsafe { Self::from_owned_ptr(sk_path_effect_create_trim(start, stop, mode)) }
    }
}
