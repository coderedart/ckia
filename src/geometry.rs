use ckia_sys::*;

pub type PointMode = sk_point_mode_t;

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct IRect(pub(crate) sk_irect_t);
impl Default for IRect {
    fn default() -> Self {
        Self(sk_irect_t {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
        })
    }
}
impl IRect {
    pub const ZERO: Self = Self::new(0, 0, 0, 0);
    pub const MAX: Self = Self::new(i32::MAX, i32::MAX, i32::MAX, i32::MAX);
    pub const fn new(left: i32, top: i32, right: i32, bottom: i32) -> Self {
        Self(sk_irect_t {
            left,
            top,
            right,
            bottom,
        })
    }
}
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct Point(pub(crate) sk_point_t);
impl Default for Point {
    fn default() -> Self {
        Self(sk_point_t { x: 0.0, y: 0.0 })
    }
}
impl Point {
    pub const ZERO: Self = Self::new(0.0, 0.0);
    pub const ONE: Self = Self::new(1.0, 1.0);
    pub const X: Self = Self::new(1.0, 0.0);
    pub const Y: Self = Self::new(0.0, 1.0);
    pub const INF: Self = Self::new(f32::INFINITY, f32::INFINITY);

    pub const fn new(x: f32, y: f32) -> Self {
        Self(sk_point_t { x, y })
    }
}
