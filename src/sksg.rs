use crate::bindings::*;

use crate::{Matrix, Rect};

crate::skia_wrapper!(
    unique,
    SkSgInvalidationController,
    sksg_invalidation_controller_t,
    sksg_invalidation_controller_delete
);

impl Default for SkSgInvalidationController {
    fn default() -> Self {
        unsafe { Self::from_owned_ptr(sksg_invalidation_controller_new()) }
    }
}
impl SkSgInvalidationController {
    pub fn invalidate(&mut self, mut rect: Rect, mut matrix: Matrix) {
        unsafe {
            sksg_invalidation_controller_inval(
                self.as_ptr_mut(),
                rect.as_ptr_mut(),
                matrix.as_ptr_mut(),
            );
        }
    }
    pub fn get_bounds(&mut self) -> Rect {
        let mut rect = Rect::default();
        unsafe { sksg_invalidation_controller_get_bounds(self.as_ptr_mut(), rect.as_ptr_mut()) };
        rect
    }
    pub fn begin(&mut self) {
        unsafe { sksg_invalidation_controller_begin(self.as_ptr_mut()) }
    }
    pub fn end(&mut self) {
        unsafe { sksg_invalidation_controller_end(self.as_ptr_mut()) }
    }
    pub fn reset(&mut self) {
        unsafe { sksg_invalidation_controller_reset(self.as_ptr_mut()) }
    }
}
