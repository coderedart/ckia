use crate::{bindings::*, MutabilityMarker, OwnerShip, SkiaPtr, SkiaPtrMut};

use crate::{path::SkiaPath, Matrix, PathEffect1DStyle, PathEffectTrimMode};

crate::skia_wrapper!(refcnt, PathEffect, sk_path_effect_t, sk_path_effect_unref);

impl<O: OwnerShip + MutabilityMarker> PathEffectGen<O> {
    pub fn create_compose(&mut self, inner: &mut Self) -> PathEffect {
        unsafe {
            Self::from_owned_ptr(sk_path_effect_create_compose(
                self.as_ptr_mut(),
                inner.as_ptr_mut(),
            ))
        }
    }
    pub fn create_sum(&mut self, second: &mut Self) -> PathEffect {
        unsafe {
            Self::from_owned_ptr(sk_path_effect_create_sum(
                self.as_ptr_mut(),
                second.as_ptr_mut(),
            ))
        }
    }
}
impl<O: OwnerShip> PathEffectGen<O> {
    pub fn discrete(seg_length: f32, deviation: f32, seed_assist: u32) -> PathEffect {
        unsafe {
            Self::from_owned_ptr(sk_path_effect_create_discrete(
                seg_length,
                deviation,
                seed_assist,
            ))
        }
    }
    pub fn create_corner(radius: f32) -> PathEffect {
        unsafe { Self::from_owned_ptr(sk_path_effect_create_corner(radius)) }
    }
    pub fn create_1d_path(
        path: &SkiaPath,
        advance: f32,
        phase: f32,
        style: PathEffect1DStyle,
    ) -> PathEffect {
        unsafe {
            Self::from_owned_ptr(sk_path_effect_create_1d_path(
                path.inner, advance, phase, style,
            ))
        }
    }
    pub fn create_2d_line(width: f32, matrix: &Matrix) -> PathEffect {
        unsafe { Self::from_owned_ptr(sk_path_effect_create_2d_line(width, matrix.as_ptr())) }
    }
    pub fn create_2d_path(matrix: &Matrix, path: &SkiaPath) -> PathEffect {
        unsafe { Self::from_owned_ptr(sk_path_effect_create_2d_path(matrix.as_ptr(), path.inner)) }
    }
    pub fn create_dash(intervals: &[f32], phase: f32) -> PathEffect {
        assert!(intervals.len() >= 2 && intervals.len() % 2 == 0);
        unsafe {
            Self::from_owned_ptr(sk_path_effect_create_dash(
                intervals.as_ptr(),
                intervals.len() as _,
                phase,
            ))
        }
    }
    pub fn create_trim(start: f32, stop: f32, mode: PathEffectTrimMode) -> PathEffect {
        unsafe { Self::from_owned_ptr(sk_path_effect_create_trim(start, stop, mode)) }
    }
}
