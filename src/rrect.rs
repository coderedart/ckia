use crate::{bindings::*, MutabilityMarker, OwnerShip, SkiaPtr, SkiaPtrMut};

use crate::{Matrix, RRectCorner, RRectType, Rect, Vector};

crate::skia_wrapper!(unique, RRect, sk_rrect_t, sk_rrect_delete);
impl Clone for RRect {
    fn clone(&self) -> Self {
        unsafe { Self::from_owned_ptr(sk_rrect_new_copy(self.as_ptr())) }
    }
}
impl Default for RRect {
    fn default() -> Self {
        unsafe { Self::from_owned_ptr(sk_rrect_new()) }
    }
}
impl<O: OwnerShip> RRectGen<O> {
    pub fn get_type(&self) -> RRectType {
        unsafe { sk_rrect_get_type(self.as_ptr()) }
    }
    pub fn get_rect(&self) -> Rect {
        let mut rect = Rect::ZERO;
        unsafe { sk_rrect_get_rect(self.as_ptr(), rect.as_ptr_mut()) }
        rect
    }
    pub fn get_radii(&self, corner: RRectCorner) -> Vector {
        let mut vector = Vector::default();
        unsafe {
            sk_rrect_get_radii(self.as_ptr(), corner, vector.as_ptr_mut());
        }
        vector
    }
    pub fn get_width(&self) -> f32 {
        unsafe { sk_rrect_get_width(self.as_ptr()) }
    }

    pub fn get_height(&self) -> f32 {
        unsafe { sk_rrect_get_height(self.as_ptr()) }
    }
    pub fn contains(&self, rect: &Rect) -> bool {
        unsafe { sk_rrect_contains(self.as_ptr(), rect.as_ptr()) }
    }
    pub fn is_valid(&self) -> bool {
        unsafe { sk_rrect_is_valid(self.as_ptr()) }
    }
}
impl<O: OwnerShip + MutabilityMarker> RRectGen<O> {
    pub fn set_empty(&mut self) {
        unsafe { sk_rrect_set_empty(self.as_ptr_mut()) }
    }
    pub fn set_rect(&mut self, rect: &Rect) {
        unsafe { sk_rrect_set_rect(self.as_ptr_mut(), rect.as_ptr()) }
    }
    pub fn set_oval(&mut self, rect: &Rect) {
        unsafe { sk_rrect_set_oval(self.as_ptr_mut(), rect.as_ptr()) }
    }
    pub fn set_rect_xy(&mut self, rect: &Rect, x_rad: f32, y_rad: f32) {
        unsafe { sk_rrect_set_rect_xy(self.as_ptr_mut(), rect.as_ptr(), x_rad, y_rad) }
    }
    pub fn set_nine_patch(
        &mut self,
        rect: &Rect,
        left_rad: f32,
        top_rad: f32,
        right_rad: f32,
        bottom_rad: f32,
    ) {
        unsafe {
            sk_rrect_set_nine_patch(
                self.as_ptr_mut(),
                rect.as_ptr(),
                left_rad,
                top_rad,
                right_rad,
                bottom_rad,
            )
        }
    }
    /// radius for 4 different corners
    pub fn set_rect_radii(&mut self, rect: &Rect, radii: &[Vector; 4]) {
        unsafe { sk_rrect_set_rect_radii(self.as_ptr_mut(), rect.as_ptr(), radii.as_ptr()) }
    }
    pub fn inset(&mut self, dx: f32, dy: f32) {
        unsafe { sk_rrect_inset(self.as_ptr_mut(), dx, dy) }
    }
    pub fn outset(&mut self, dx: f32, dy: f32) {
        unsafe { sk_rrect_outset(self.as_ptr_mut(), dx, dy) }
    }
    pub fn offset(&mut self, dx: f32, dy: f32) {
        unsafe { sk_rrect_offset(self.as_ptr_mut(), dx, dy) }
    }
    pub fn transform(&mut self, matrix: &Matrix, dest: &mut Self) -> bool {
        unsafe { sk_rrect_transform(self.as_ptr_mut(), matrix.as_ptr(), dest.inner) }
    }
}
