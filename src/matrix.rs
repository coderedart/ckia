use std::borrow::{Borrow, BorrowMut};

use ckia_sys::*;

use crate::{Matrix, Matrix44, Point, Rect};

impl Matrix {
    pub fn get_scale_x(&self) -> f32 {
        self.0.scaleX
    }
    pub fn set_scale_x(&mut self, scale_x: f32) {
        self.0.scaleX = scale_x;
    }
    pub fn get_skew_x(&self) -> f32 {
        self.0.skewX
    }
    pub fn set_skew_x(&mut self, skew_x: f32) {
        self.0.skewX = skew_x;
    }
    pub fn get_trans_x(&self) -> f32 {
        self.0.transX
    }
    pub fn set_trans_x(&mut self, trans_x: f32) {
        self.0.transX = trans_x;
    }
    pub fn get_skew_y(&self) -> f32 {
        self.0.skewY
    }
    pub fn set_skew_y(&mut self, skew_y: f32) {
        self.0.skewY = skew_y;
    }
    pub fn get_scale_y(&self) -> f32 {
        self.0.scaleY
    }
    pub fn set_scale_y(&mut self, scale_y: f32) {
        self.0.scaleY = scale_y;
    }
    pub fn get_trans_y(&self) -> f32 {
        self.0.transY
    }
    pub fn set_trans_y(&mut self, trans_y: f32) {
        self.0.transY = trans_y;
    }
}
impl Default for Matrix {
    fn default() -> Self {
        Self::IDENTITY
    }
}
impl Matrix {
    pub const IDENTITY: Self = Self::new_identity();

    pub const fn new_identity() -> Self {
        Self(sk_matrix_t {
            scaleX: 1.0,
            skewX: 0.0,
            transX: 0.0,
            skewY: 0.0,
            scaleY: 1.0,
            transY: 0.0,
            persp0: 0.0,
            persp1: 0.0,
            persp2: 1.0,
        })
    }
    /// If a matrix maps from source to destination. Its invert maps from destination to source.
    /// returns None if it can't be inverted.
    pub fn invert(mut self) -> Option<Matrix> {
        let mut result = Self::IDENTITY;
        unsafe {
            sk_matrix_try_invert(self.as_ptr_mut() as _, result.as_ptr_mut()).then_some(result)
        }
    }
    pub fn concat(mut self, mut other: Self) -> Self {
        let mut result = Self::IDENTITY;
        unsafe {
            sk_matrix_concat(result.as_ptr_mut(), self.as_ptr_mut(), other.as_ptr_mut());
        }
        result
    }
    pub fn pre_concat(mut self, mut other: Self) -> Self {
        unsafe {
            sk_matrix_pre_concat(self.as_ptr_mut(), other.as_ptr_mut());
        }
        self
    }
    pub fn post_concat(mut self, mut other: Self) -> Self {
        unsafe {
            sk_matrix_post_concat(self.as_ptr_mut(), other.as_ptr_mut());
        }
        self
    }

    pub fn map_rect(mut self, mut src: Rect) -> Rect {
        let mut dst = Rect::default();
        unsafe {
            sk_matrix_map_rect(self.as_ptr_mut(), dst.as_ptr_mut(), src.as_ptr_mut());
        }
        dst
    }
    pub fn map_points(mut self, points: &mut [Point]) {
        unsafe {
            sk_matrix_map_points(
                self.as_ptr_mut(),
                points.as_mut_ptr() as _,
                points.as_mut_ptr() as _,
                points.len().try_into().unwrap(),
            );
        }
    }
    pub fn map_vectors(mut self, vectors: &mut [Point]) {
        unsafe {
            sk_matrix_map_vectors(
                self.as_ptr_mut(),
                vectors.as_mut_ptr() as _,
                vectors.as_mut_ptr() as _,
                vectors.len().try_into().unwrap(),
            );
        }
    }
    pub fn map_xy(mut self, x: f32, y: f32) -> Point {
        let mut point = Point::ZERO;
        unsafe {
            sk_matrix_map_xy(self.as_ptr_mut(), x, y, point.as_ptr_mut());
        }
        point
    }
    pub fn map_vector(mut self, x: f32, y: f32) -> Point {
        let mut point = Point::ZERO;
        unsafe {
            sk_matrix_map_vector(self.as_ptr_mut(), x, y, point.as_ptr_mut());
        }
        point
    }
    pub fn map_radius(mut self, radius: f32) -> f32 {
        unsafe { sk_matrix_map_radius(self.as_ptr_mut(), radius) }
    }
}

impl Default for Matrix44 {
    fn default() -> Self {
        Self(sk_matrix44_t {
            m00: 0.0,
            m01: 0.0,
            m02: 0.0,
            m03: 0.0,
            m10: 0.0,
            m11: 0.0,
            m12: 0.0,
            m13: 0.0,
            m20: 0.0,
            m21: 0.0,
            m22: 0.0,
            m23: 0.0,
            m30: 0.0,
            m31: 0.0,
            m32: 0.0,
            m33: 0.0,
        })
    }
}

// crate::pod_struct!(pub Matrix, sk_matrix_t {
//     pub scaleX: f32,
//     pub skewX: f32,
//     pub transX: f32,
//     pub skewY: f32,
//     pub scaleY: f32,
//     pub transY: f32,
//     pub persp0: f32,
//     pub persp1: f32,
//     pub persp2: f32,
// });
