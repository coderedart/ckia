use std::marker::PhantomData;

use ckia_sys::*;

use crate::{path::SkiaPath, IRect, RegionOp, SkiaPointer};

crate::skia_wrapper!(unique, Region, sk_region_t, sk_region_delete);
impl Default for Region {
    fn default() -> Self {
        unsafe { Self::from_owned_ptr(sk_region_new()) }
    }
}
impl Region {
    pub fn is_empty(&self) -> bool {
        unsafe { sk_region_is_empty(self.as_ptr()) }
    }
    pub fn is_rect(&self) -> bool {
        unsafe { sk_region_is_rect(self.as_ptr()) }
    }
    pub fn is_complex(&self) -> bool {
        unsafe { sk_region_is_complex(self.as_ptr()) }
    }
    pub fn get_bounds(&self) -> IRect {
        let mut rect = IRect::default();
        unsafe { sk_region_get_bounds(self.as_ptr(), rect.as_ptr_mut()) };
        rect
    }
    /// returns true if not empty and boundary appended to path. if false, path is unmodified
    pub fn get_boundary_path(&self, path: &mut SkiaPath) -> bool {
        unsafe { sk_region_get_boundary_path(self.as_ptr(), path.as_ptr_mut()) }
    }
    pub fn set_empty(&mut self) {
        unsafe { sk_region_set_empty(self.as_ptr_mut()) };
    }
    pub fn set_rect(&mut self, rect: &IRect) {
        unsafe { sk_region_set_rect(self.as_ptr_mut(), rect.as_ptr()) };
    }
    pub fn set_rects(&mut self, rects: &[IRect]) {
        unsafe { sk_region_set_rects(self.as_ptr_mut(), rects.as_ptr() as _, rects.len() as _) };
    }
    pub fn set_region(&mut self, region: &Self) -> bool {
        unsafe { sk_region_set_region(self.as_ptr_mut(), region.as_ptr()) }
    }
    /// returns true if resulting region is not empty
    pub fn set_path(&mut self, path: &SkiaPath, clip: &Self) -> bool {
        unsafe { sk_region_set_path(self.as_ptr_mut(), path.as_ptr(), clip.as_ptr()) }
    }
    #[must_use]
    pub fn intersects_rect(&self, rect: &IRect) -> bool {
        unsafe { sk_region_intersects_rect(self.as_ptr(), rect.as_ptr()) }
    }
    #[must_use]
    pub fn intersects(&self, other: &Region) -> bool {
        unsafe { sk_region_intersects(self.as_ptr(), other.as_ptr()) }
    }
    #[must_use]
    pub fn contains_point(&self, x: i32, y: i32) -> bool {
        unsafe { sk_region_contains_point(self.as_ptr(), x, y) }
    }
    #[must_use]
    pub fn contains_rect(&self, rect: &IRect) -> bool {
        unsafe { sk_region_contains_rect(self.as_ptr(), rect.as_ptr()) }
    }
    #[must_use]
    pub fn contains(&self, region: &Self) -> bool {
        unsafe { sk_region_contains(self.as_ptr(), region.as_ptr()) }
    }
    #[must_use]
    pub fn quick_contains(&self, rect: &IRect) -> bool {
        unsafe { sk_region_quick_contains(self.as_ptr(), rect.as_ptr()) }
    }
    #[must_use]
    pub fn quick_reject_rect(&self, rect: &IRect) -> bool {
        unsafe { sk_region_quick_reject_rect(self.as_ptr(), rect.as_ptr()) }
    }
    #[must_use]
    pub fn quick_reject(&self, region: &Self) -> bool {
        unsafe { sk_region_quick_reject(self.as_ptr(), region.as_ptr()) }
    }
    pub fn translate(&mut self, x: i32, y: i32) {
        unsafe { sk_region_translate(self.as_ptr_mut(), x, y) }
    }
    /// returns false if result is empty
    pub fn op_rect(&mut self, rect: &IRect, op: RegionOp) -> bool {
        unsafe { sk_region_op_rect(self.as_ptr_mut(), rect.as_ptr(), op) }
    }
    /// returns false if result is empty
    pub fn op(&mut self, region: &Region, op: RegionOp) -> bool {
        unsafe { sk_region_op(self.as_ptr_mut(), region.as_ptr(), op) }
    }
    /// Returns sequence of [IRect], sorted along y-axis, then x-axis, that make
    /// up [Region].
    pub fn iter_regions(&self) -> RegionIterator<'_> {
        unsafe {
            let inner = sk_region_iterator_new(self.as_ptr());
            assert!(!inner.is_null());
            RegionIterator {
                inner,
                phantom: PhantomData,
                already_done: false,
            }
        }
    }
    pub fn cliperator(&self, clip: &IRect) -> RegionCliperator<'_> {
        unsafe {
            let inner = sk_region_cliperator_new(self.as_ptr(), clip.as_ptr());
            assert!(!inner.is_null());

            RegionCliperator {
                inner,
                phantom: PhantomData,
                already_done: false,
            }
        }
    }
    pub fn spanerator(&self, y: i32, left: i32, right: i32) -> RegionSpanerator<'_> {
        unsafe {
            let inner = sk_region_spanerator_new(self.as_ptr(), y, left, right);
            assert!(!inner.is_null());
            RegionSpanerator {
                inner,
                phantom: PhantomData,
            }
        }
    }
}
/// Returns sequence of [IRect], sorted along y-axis, then x-axis, that make
/// up [Region].
pub struct RegionIterator<'a> {
    inner: *mut sk_region_iterator_t,
    already_done: bool,
    phantom: PhantomData<&'a Region>,
}
impl<'a> Drop for RegionIterator<'a> {
    fn drop(&mut self) {
        unsafe { sk_region_iterator_delete(self.inner) }
    }
}
impl<'a> RegionIterator<'a> {
    /// sets the iterator to point to the first [IRect] again.
    pub fn rewind(&mut self) -> bool {
        unsafe { sk_region_iterator_rewind(self.inner) }
    }
    fn is_done(&self) -> bool {
        unsafe { sk_region_iterator_done(self.inner) }
    }
    fn advance(&mut self) {
        unsafe { sk_region_iterator_next(self.inner) }
    }
    fn get_rect(&self) -> IRect {
        let mut rect = IRect::default();
        unsafe { sk_region_iterator_rect(self.inner, rect.as_ptr_mut()) };
        rect
    }
}
impl<'a> Iterator for RegionIterator<'a> {
    type Item = IRect;

    fn next(&mut self) -> Option<Self::Item> {
        if self.already_done {
            return None;
        }
        self.already_done = self.is_done();
        let rect = self.get_rect();
        self.advance();
        Some(rect)
    }
}

pub struct RegionCliperator<'a> {
    inner: *mut sk_region_cliperator_t,
    already_done: bool,
    phantom: PhantomData<&'a Region>,
}
impl<'a> Drop for RegionCliperator<'a> {
    fn drop(&mut self) {
        unsafe { sk_region_cliperator_delete(self.inner) }
    }
}
impl<'a> RegionCliperator<'a> {
    pub fn cliperator_done(&mut self) -> bool {
        unsafe { sk_region_cliperator_done(self.inner) }
    }
    pub fn cliperator_next(&mut self) {
        unsafe { sk_region_cliperator_next(self.inner) }
    }
    pub fn cliperator_rect(&self) -> IRect {
        let mut rect = IRect::default();
        unsafe { sk_region_cliperator_rect(self.inner, rect.as_ptr_mut()) };
        rect
    }
}
impl<'a> Iterator for RegionCliperator<'a> {
    type Item = IRect;

    fn next(&mut self) -> Option<Self::Item> {
        if self.already_done {
            return None;
        }
        self.already_done = self.cliperator_done();
        let rect = self.cliperator_rect();
        self.cliperator_next();
        Some(rect)
    }
}
pub struct RegionSpanerator<'a> {
    inner: *mut sk_region_spanerator_t,
    phantom: PhantomData<&'a Region>,
}
impl<'a> Drop for RegionSpanerator<'a> {
    fn drop(&mut self) {
        unsafe { sk_region_spanerator_delete(self.inner) }
    }
}
impl<'a> Iterator for RegionSpanerator<'a> {
    type Item = (i32, i32);
    /// returns (left, right) representing span start and end if interval is found
    fn next(&mut self) -> Option<Self::Item> {
        let mut left = 0;
        let mut right = 0;
        unsafe {
            sk_region_spanerator_next(self.inner, &mut left as _, &mut right as _)
                .then_some((left, right))
        }
    }
}
