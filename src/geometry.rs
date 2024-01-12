use std::borrow::{Borrow, BorrowMut};

use ckia_sys::*;

pub type PointMode = sk_point_mode_t;

#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct Rect(pub(crate) sk_rect_t);

impl AsRef<sk_rect_t> for Rect {
    fn as_ref(&self) -> &sk_rect_t {
        &self.0
    }
}
impl AsMut<sk_rect_t> for Rect {
    fn as_mut(&mut self) -> &mut sk_rect_t {
        &mut self.0
    }
}
impl Borrow<sk_rect_t> for Rect {
    fn borrow(&self) -> &sk_rect_t {
        &self.0
    }
}
impl BorrowMut<sk_rect_t> for Rect {
    fn borrow_mut(&mut self) -> &mut sk_rect_t {
        &mut self.0
    }
}
impl Rect {
    #[allow(unused)]
    pub(crate) fn as_ptr(&self) -> *const sk_rect_t {
        &self.0 as _
    }
    pub(crate) fn as_ptr_mut(&mut self) -> *mut sk_rect_t {
        &mut self.0 as _
    }
    pub fn get_left(&self) -> f32 {
        self.0.left
    }
    pub fn set_left(&mut self, left: f32) {
        self.0.left = left;
    }
    pub fn get_top(&self) -> f32 {
        self.0.top
    }
    pub fn set_top(&mut self, top: f32) {
        self.0.top = top;
    }
    pub fn get_right(&self) -> f32 {
        self.0.right
    }
    pub fn set_right(&mut self, right: f32) {
        self.0.right = right;
    }
    pub fn get_bottom(&self) -> f32 {
        self.0.bottom
    }
    pub fn set_bottom(&mut self, bottom: f32) {
        self.0.bottom = bottom;
    }
}
impl Default for Rect {
    fn default() -> Self {
        Self::ZERO
    }
}
impl Rect {
    pub const ZERO: Self = Self::new(0.0, 0.0, 0.0, 0.0);
    pub const INF: Self = Self::new(0.0, 0.0, f32::INFINITY, f32::INFINITY);
    pub const MAX: Self = Self::new(f32::MAX, f32::MAX, f32::MAX, f32::MAX);
    pub const fn new(left: f32, top: f32, right: f32, bottom: f32) -> Self {
        Self(sk_rect_t {
            left,
            top,
            right,
            bottom,
        })
    }
}
#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct IRect(pub(crate) sk_irect_t);

impl AsRef<sk_irect_t> for IRect {
    fn as_ref(&self) -> &sk_irect_t {
        &self.0
    }
}
impl AsMut<sk_irect_t> for IRect {
    fn as_mut(&mut self) -> &mut sk_irect_t {
        &mut self.0
    }
}
impl Borrow<sk_irect_t> for IRect {
    fn borrow(&self) -> &sk_irect_t {
        &self.0
    }
}
impl BorrowMut<sk_irect_t> for IRect {
    fn borrow_mut(&mut self) -> &mut sk_irect_t {
        &mut self.0
    }
}
impl IRect {
    #[allow(unused)]
    pub(crate) fn as_ptr(&self) -> *const sk_irect_t {
        &self.0 as _
    }
    #[allow(unused)]
    pub(crate) fn as_ptr_mut(&mut self) -> *mut sk_irect_t {
        &mut self.0 as _
    }
    pub fn get_left(&self) -> i32 {
        self.0.left
    }
    pub fn set_left(&mut self, left: i32) {
        self.0.left = left;
    }
    pub fn get_top(&self) -> i32 {
        self.0.top
    }
    pub fn set_top(&mut self, top: i32) {
        self.0.top = top;
    }
    pub fn get_right(&self) -> i32 {
        self.0.right
    }
    pub fn set_right(&mut self, right: i32) {
        self.0.right = right;
    }
    pub fn get_bottom(&self) -> i32 {
        self.0.bottom
    }
    pub fn set_bottom(&mut self, bottom: i32) {
        self.0.bottom = bottom;
    }
}
impl Default for IRect {
    fn default() -> Self {
        Self::ZERO
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
#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct Point(pub(crate) sk_point_t);

impl AsRef<sk_point_t> for Point {
    fn as_ref(&self) -> &sk_point_t {
        &self.0
    }
}
impl AsMut<sk_point_t> for Point {
    fn as_mut(&mut self) -> &mut sk_point_t {
        &mut self.0
    }
}
impl Borrow<sk_point_t> for Point {
    fn borrow(&self) -> &sk_point_t {
        &self.0
    }
}
impl BorrowMut<sk_point_t> for Point {
    fn borrow_mut(&mut self) -> &mut sk_point_t {
        &mut self.0
    }
}
#[allow(unused)]
impl Point {
    pub(crate) fn as_ptr(&self) -> *const sk_point_t {
        &self.0 as _
    }
    pub(crate) fn as_ptr_mut(&mut self) -> *mut sk_point_t {
        &mut self.0 as _
    }
    fn get_x(&self) -> f32 {
        self.0.x
    }
    fn set_x(&mut self, x: f32) {
        self.0.x = x;
    }
    fn get_y(&self) -> f32 {
        self.0.y
    }
    fn set_y(&mut self, y: f32) {
        self.0.y = y;
    }
}
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

#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct Vector(pub(crate) sk_vector_t);

impl AsRef<sk_vector_t> for Vector {
    fn as_ref(&self) -> &sk_vector_t {
        &self.0
    }
}
impl AsMut<sk_vector_t> for Vector {
    fn as_mut(&mut self) -> &mut sk_vector_t {
        &mut self.0
    }
}
impl Borrow<sk_vector_t> for Vector {
    fn borrow(&self) -> &sk_vector_t {
        &self.0
    }
}
impl BorrowMut<sk_vector_t> for Vector {
    fn borrow_mut(&mut self) -> &mut sk_vector_t {
        &mut self.0
    }
}
#[allow(unused)]
impl Vector {
    pub(crate) fn as_ptr(&self) -> *const sk_vector_t {
        &self.0 as _
    }
    pub(crate) fn as_ptr_mut(&mut self) -> *mut sk_vector_t {
        &mut self.0 as _
    }
    fn get_x(&self) -> f32 {
        self.0.x
    }
    fn set_x(&mut self, x: f32) {
        self.0.x = x;
    }
    fn get_y(&self) -> f32 {
        self.0.y
    }
    fn set_y(&mut self, y: f32) {
        self.0.y = y;
    }
}
impl Default for Vector {
    fn default() -> Self {
        Self(sk_vector_t { x: 0.0, y: 0.0 })
    }
}
impl Vector {
    pub const ZERO: Self = Self::new(0.0, 0.0);
    pub const ONE: Self = Self::new(1.0, 1.0);
    pub const X: Self = Self::new(1.0, 0.0);
    pub const Y: Self = Self::new(0.0, 1.0);
    pub const INF: Self = Self::new(f32::INFINITY, f32::INFINITY);

    pub const fn new(x: f32, y: f32) -> Self {
        Self(sk_vector_t { x, y })
    }
}
