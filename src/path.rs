use std::{ffi::CStr, marker::PhantomData};

use crate::bindings::*;

use crate::{
    rrect::RRect, string::SkiaString, Matrix, PathAddMode, PathArcSize, PathDirection,
    PathFillType, PathMeasureMatrixflags, PathOp, PathVerb, Point, Rect, Vector,
};

crate::skia_wrapper!(unique, SkiaPath, sk_path_t, sk_path_delete);
impl Clone for SkiaPath {
    fn clone(&self) -> Self {
        unsafe { Self::from_owned_ptr(sk_path_clone(self.as_ptr())) }
    }
}
impl Default for SkiaPath {
    fn default() -> Self {
        unsafe { Self::from_owned_ptr(sk_path_new()) }
    }
}
impl SkiaPath {
    pub fn move_to(&mut self, x: f32, y: f32) {
        unsafe { sk_path_move_to(self.inner, x, y) }
    }
    pub fn line_to(&mut self, x: f32, y: f32) {
        unsafe { sk_path_line_to(self.inner, x, y) }
    }
    pub fn quad_to(&mut self, x0: f32, y0: f32, x1: f32, y1: f32) {
        unsafe { sk_path_quad_to(self.inner, x0, y0, x1, y1) }
    }
    pub fn conic_to(&mut self, x0: f32, y0: f32, x1: f32, y1: f32, w: f32) {
        unsafe { sk_path_conic_to(self.inner, x0, y0, x1, y1, w) }
    }
    pub fn cubic_to(&mut self, x0: f32, y0: f32, x1: f32, y1: f32, x2: f32, y2: f32) {
        unsafe {
            sk_path_cubic_to(self.inner, x0, y0, x1, y1, x2, y2);
        }
    }
    pub fn arc_to(
        &mut self,
        rx: f32,
        ry: f32,
        x_axis_rotate: f32,
        large_arc: PathArcSize,
        sweep: PathDirection,
        x: f32,
        y: f32,
    ) {
        unsafe { sk_path_arc_to(self.inner, rx, ry, x_axis_rotate, large_arc, sweep, x, y) }
    }
    pub fn rarc_to(
        &mut self,
        rx: f32,
        ry: f32,
        x_axis_rotate: f32,
        large_arc: PathArcSize,
        sweep: PathDirection,
        x: f32,
        y: f32,
    ) {
        unsafe { sk_path_rarc_to(self.inner, rx, ry, x_axis_rotate, large_arc, sweep, x, y) }
    }
    pub fn arc_to_with_oval(
        &mut self,
        oval: &Rect,
        start_angle: f32,
        sweep_angle: f32,
        force_move_to: bool,
    ) {
        unsafe {
            sk_path_arc_to_with_oval(
                self.inner,
                oval.as_ptr(),
                start_angle,
                sweep_angle,
                force_move_to,
            )
        }
    }
    pub fn arc_to_with_points(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, radius: f32) {
        unsafe {
            sk_path_arc_to_with_points(self.inner, x1, y1, x2, y2, radius);
        }
    }
    pub fn close(&mut self) {
        unsafe { sk_path_close(self.inner) }
    }
    pub fn add_rect(&mut self, rect: &Rect, dir: PathDirection) {
        unsafe { sk_path_add_rect(self.inner, rect.as_ptr(), dir) }
    }
    pub fn add_rrect(&mut self, rect: &RRect, dir: PathDirection) {
        unsafe {
            sk_path_add_rrect(self.inner, rect.inner, dir);
        }
    }
    pub fn add_rrect_start(&mut self, rect: &RRect, dir: PathDirection, start: u32) {
        unsafe {
            sk_path_add_rrect_start(self.inner, rect.inner, dir, start);
        }
    }
    pub fn add_rounded_rect(&mut self, rect: &Rect, rx: f32, ry: f32, dir: PathDirection) {
        unsafe { sk_path_add_rounded_rect(self.inner, rect.as_ptr(), rx, ry, dir) }
    }
    pub fn add_oval(&mut self, rect: &Rect, dir: PathDirection) {
        unsafe { sk_path_add_oval(self.inner, rect.as_ptr(), dir) }
    }
    pub fn add_circle(&mut self, x: f32, y: f32, radius: f32, dir: PathDirection) {
        unsafe { sk_path_add_circle(self.inner, x, y, radius, dir) }
    }
    pub fn get_bounds(&self) -> Rect {
        let mut rect = Rect::ZERO;
        unsafe { sk_path_get_bounds(self.inner, rect.as_ptr_mut()) }
        rect
    }
    pub fn compute_tight_bounds(&self) -> Rect {
        let mut rect = Rect::ZERO;
        unsafe { sk_path_compute_tight_bounds(self.inner, rect.as_ptr_mut()) }
        rect
    }
    pub fn rmove_to(&mut self, dx: f32, dy: f32) {
        unsafe { sk_path_rmove_to(self.inner, dx, dy) }
    }
    pub fn rline_to(&mut self, dx: f32, dy: f32) {
        unsafe { sk_path_rline_to(self.inner, dx, dy) }
    }
    pub fn rquad_to(&mut self, dx0: f32, dy0: f32, dx1: f32, dy1: f32) {
        unsafe { sk_path_rquad_to(self.inner, dx0, dy0, dx1, dy1) }
    }
    pub fn rconic_to(&mut self, dx0: f32, dy0: f32, dx1: f32, dy1: f32, w: f32) {
        unsafe { sk_path_rconic_to(self.inner, dx0, dy0, dx1, dy1, w) }
    }
    pub fn rcubic_to(&mut self, dx0: f32, dy0: f32, dx1: f32, dy1: f32, dx2: f32, dy2: f32) {
        unsafe { sk_path_rcubic_to(self.inner, dx0, dy0, dx1, dy1, dx2, dy2) }
    }
    pub fn add_rect_start(&mut self, rect: &Rect, dir: PathDirection, start_index: u32) {
        unsafe { sk_path_add_rect_start(self.inner, rect.as_ptr(), dir, start_index) }
    }
    pub fn add_arc(&mut self, rect: &Rect, start_angle: f32, sweep_angle: f32) {
        unsafe { sk_path_add_arc(self.inner, rect.as_ptr(), start_angle, sweep_angle) }
    }
    pub fn get_filltype(&mut self) -> PathFillType {
        unsafe { sk_path_get_filltype(self.inner) }
    }
    pub fn set_filltype(&mut self, filltype: PathFillType) {
        unsafe { sk_path_set_filltype(self.inner, filltype) }
    }
    pub fn transform(&mut self, matrix: &Matrix) {
        unsafe { sk_path_transform(self.inner, matrix.as_ptr()) }
    }
    /// use [Self::transform] to transform self.
    /// This function is to make a new copy that is transformed by the matrix.
    pub fn transform_to_dest(&self, matrix: &Matrix, dest: &mut Self) {
        unsafe { sk_path_transform_to_dest(self.inner, matrix.as_ptr(), dest.inner) }
    }
    pub fn add_path_offset(&mut self, other: &mut Self, dx: f32, dy: f32, add_mode: PathAddMode) {
        unsafe {
            sk_path_add_path_offset(self.inner, other.inner, dx, dy, add_mode);
        }
    }
    pub fn add_path_matrix(&mut self, other: &mut Self, mut matrix: Matrix, add_mode: PathAddMode) {
        unsafe {
            sk_path_add_path_matrix(self.inner, other.inner, matrix.as_ptr_mut(), add_mode);
        }
    }
    pub fn add_path(&mut self, other: &mut Self, add_mode: PathAddMode) {
        unsafe {
            sk_path_add_path(self.inner, other.inner, add_mode);
        }
    }
    pub fn add_path_reverse(&mut self, other: &mut Self) {
        unsafe { sk_path_add_path_reverse(self.inner, other.inner) }
    }
    pub fn reset(&mut self) {
        unsafe { sk_path_reset(self.inner) }
    }
    pub fn rewind(&mut self) {
        unsafe { sk_path_rewind(self.inner) }
    }
    pub fn count_points(&self) -> i32 {
        unsafe { sk_path_count_points(self.inner) }
    }
    pub fn count_verbs(&self) -> i32 {
        unsafe { sk_path_count_verbs(self.inner) }
    }
    pub fn get_point(&self, index: i32) -> Point {
        let mut point = Point::ZERO;
        unsafe { sk_path_get_point(self.inner, index, point.as_ptr_mut()) }
        point
    }
    pub fn get_points(&self, points: &mut [Point]) -> i32 {
        unsafe { sk_path_get_points(self.inner, points.as_mut_ptr() as _, points.len() as _) }
    }
    pub fn contains(&self, x: f32, y: f32) -> bool {
        unsafe { sk_path_contains(self.inner, x, y) }
    }
    /// returns true on success and self contains the new parsed contents. otherwise, self is unchanged
    #[must_use]
    pub fn parse_svg_string(&mut self, str: &CStr) -> bool {
        unsafe { sk_path_parse_svg_string(self.inner, str.as_ptr() as _) }
    }
    pub fn to_svg_string(&self, str: &mut SkiaString) {
        unsafe { sk_path_to_svg_string(self.inner, str.inner) }
    }
    pub fn get_last_point(&self) -> Option<Point> {
        let mut point = Point::default();
        unsafe { sk_path_get_last_point(self.inner, point.as_ptr_mut()).then_some(point) }
    }
    /// I don't know what this does, but the slice must be atleast `1 + (2 * (1 << pow2))`  long. or we panic.
    pub fn convert_to_quads(
        start: Point,
        control: Point,
        end: Point,
        weight: f32,
        points: &mut [Point],
        pow2: i32,
    ) -> i32 {
        assert!(points.len() >= (1 + 2 * (1usize << pow2 as usize)));
        unsafe {
            sk_path_convert_conic_to_quads(
                start.as_ptr(),
                control.as_ptr(),
                end.as_ptr(),
                weight,
                points.as_mut_ptr() as _,
                pow2,
            )
        }
    }
    pub fn add_poly(&mut self, points: &[Point], close: bool) {
        unsafe { sk_path_add_poly(self.inner, points.as_ptr() as _, points.len() as _, close) }
    }
    pub fn get_segment_masks(&mut self) -> u32 {
        unsafe { sk_path_get_segment_masks(self.inner) }
    }
    /// returns bounds if this is oval. none of it this is not oval
    pub fn is_oval(&mut self) -> Option<Rect> {
        let mut bounds = Rect::default();
        unsafe { sk_path_is_oval(self.inner, bounds.as_ptr_mut()).then_some(bounds) }
    }
    /// if this is representable as rrect, it will return true, otherwise false
    /// If bounds is Some and if it returns true, then it filled the bounds variable with path's bounds.
    /// if bounds is None, it won't write bounds even if true. bounds is completely ignored in the false case
    pub fn is_rrect(&mut self, bounds: Option<&mut RRect>) -> bool {
        unsafe {
            sk_path_is_rrect(
                self.inner,
                bounds.map(|b| b.inner).unwrap_or(std::ptr::null_mut()),
            )
        }
    }
    /// if line, returns start and end points. otherwise, none.
    pub fn is_line(&mut self) -> Option<[Point; 2]> {
        let mut line = [Point::ZERO; 2];
        unsafe { sk_path_is_line(self.inner, line.as_mut_ptr() as _).then_some(line) }
    }
    /// if rect, returns rect + if path is closed + PathDirection. otherwise None.
    pub fn is_rect(&mut self) -> Option<(Rect, bool, PathDirection)> {
        let mut rect = Rect::ZERO;
        let mut closed = false;
        let mut path_direction = PathDirection::CW_SK_PATH_DIRECTION;
        unsafe {
            sk_path_is_rect(
                self.inner,
                rect.as_ptr_mut(),
                &mut closed as _,
                &mut path_direction as _,
            )
            .then_some((rect, closed, path_direction))
        }
    }
    pub fn is_convex(&self) -> bool {
        unsafe { sk_path_is_convex(self.inner) }
    }
    pub fn iter(&mut self, force_close: bool) -> PathIterator {
        unsafe {
            PathIterator {
                inner: sk_path_create_iter(
                    self.inner,
                    force_close.then_some(1).unwrap_or_default(), // convert bool into i32
                ),
                phantom: PhantomData,
            }
        }
    }
    pub fn raw_iter(&mut self) -> RawPathIterator {
        unsafe {
            RawPathIterator {
                inner: sk_path_create_rawiter(self.inner),
                phantom: PhantomData,
            }
        }
    }
    /// returns true on success. if false, then result is untouched.
    #[must_use]
    pub fn op(&self, other: &Self, op: PathOp, result: &mut SkiaPath) -> bool {
        unsafe { sk_pathop_op(self.inner, other.inner, op, result.inner) }
    }
    /// returns true on success. if false, then result is unmodified.
    #[must_use]
    pub fn simplify(&self, result: &mut SkiaPath) -> bool {
        unsafe { sk_pathop_simplify(self.inner, result.inner) }
    }
    pub fn tight_bounds(&self) -> Option<Rect> {
        let mut result = Rect::default();
        unsafe { sk_pathop_tight_bounds(self.inner, result.as_ptr_mut()).then_some(result) }
    }

    /// returns true on success. if false, then result is unmodified.
    #[must_use]
    pub fn as_winding(&self, result: SkiaPath) -> bool {
        unsafe { sk_pathop_as_winding(self.inner, result.inner) }
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct PathIterator<'a> {
    pub(crate) inner: *mut sk_path_iterator_t,
    pub phantom: PhantomData<&'a SkiaPath>,
}
impl<'a> Drop for PathIterator<'a> {
    fn drop(&mut self) {
        unsafe {
            sk_path_iter_destroy(self.inner);
        }
    }
}

impl<'a> PathIterator<'a> {
    pub fn next(&mut self, points: &mut [Point; 4]) -> PathVerb {
        unsafe { sk_path_iter_next(self.inner, points.as_mut_ptr() as _) }
    }
    pub fn conic_weight(&mut self) -> f32 {
        unsafe { sk_path_iter_conic_weight(self.inner) }
    }
    pub fn is_close_line(&mut self) -> bool {
        unsafe {
            sk_path_iter_is_close_line(self.inner) != 0 // convert int to bool
        }
    }
    pub fn is_closed_counter(&mut self) -> bool {
        unsafe {
            sk_path_iter_is_closed_contour(self.inner) != 0 // int to bool
        }
    }
}
#[derive(Debug)]
#[repr(transparent)]
pub struct RawPathIterator<'a> {
    pub(crate) inner: *mut sk_path_rawiterator_t,
    pub phantom: PhantomData<&'a SkiaPath>,
}
impl<'a> Drop for RawPathIterator<'a> {
    fn drop(&mut self) {
        unsafe {
            sk_path_rawiter_destroy(self.inner);
        }
    }
}

impl<'a> RawPathIterator<'a> {
    pub fn next(&mut self, points: &mut [Point; 4]) -> PathVerb {
        unsafe { sk_path_rawiter_next(self.inner, points.as_mut_ptr() as _) }
    }
    pub fn conic_weight(&mut self) -> f32 {
        unsafe { sk_path_rawiter_conic_weight(self.inner) }
    }
    pub fn peek(&mut self) -> PathVerb {
        unsafe { sk_path_rawiter_peek(self.inner) }
    }
}
crate::skia_wrapper!(unique, OpBuilder, sk_opbuilder_t, sk_opbuilder_destroy);
impl Default for OpBuilder {
    fn default() -> Self {
        unsafe { Self::from_owned_ptr(sk_opbuilder_new()) }
    }
}
impl OpBuilder {
    pub fn add(&mut self, path: &SkiaPath, op: PathOp) {
        unsafe { sk_opbuilder_add(self.inner, path.inner, op) }
    }

    /// returns true on success.
    #[must_use]
    pub fn resolve(&mut self, result: &mut SkiaPath) -> bool {
        unsafe { sk_opbuilder_resolve(self.inner, result.inner) }
    }
}
crate::skia_wrapper!(
    unique,
    PathMeasure,
    sk_pathmeasure_t,
    sk_pathmeasure_destroy
);
impl Default for PathMeasure {
    fn default() -> Self {
        unsafe { Self::from_owned_ptr(sk_pathmeasure_new()) }
    }
}
impl PathMeasure {
    pub fn new_with_path(path: &SkiaPath, force_closed: bool, res_scale: f32) -> Self {
        unsafe {
            Self::from_owned_ptr(sk_pathmeasure_new_with_path(
                path.inner,
                force_closed,
                res_scale,
            ))
        }
    }
    pub fn set_path(&mut self, path: &SkiaPath, force_closed: bool) {
        unsafe { sk_pathmeasure_set_path(self.inner, path.inner, force_closed) }
    }
    pub fn get_length(&mut self) -> f32 {
        unsafe { sk_pathmeasure_get_length(self.inner) }
    }
    /// returns (position, tangent) if there is a path and distance > 0
    pub fn get_pos_tan(&mut self, distance: f32) -> Option<(Point, Vector)> {
        let mut position = Point::default();
        let mut tangent = Vector::default();
        unsafe {
            sk_pathmeasure_get_pos_tan(
                self.inner,
                distance,
                position.as_ptr_mut(),
                tangent.as_ptr_mut(),
            )
            .then_some((position, tangent))
        }
    }
    pub fn get_matrix(&mut self, distance: f32, flags: PathMeasureMatrixflags) -> Option<Matrix> {
        let mut matrix = Matrix::default();

        unsafe {
            sk_pathmeasure_get_matrix(self.inner, distance, matrix.as_ptr_mut(), flags)
                .then_some(matrix)
        }
    }

    /// returns true on success. if false, then dest is unmodified.
    #[must_use]
    pub fn get_segment(
        &mut self,
        start_distnce: f32,
        stop_distance: f32,
        dest: &mut SkiaPath,
        starts_with_move: bool,
    ) -> bool {
        unsafe {
            sk_pathmeasure_get_segment(
                self.inner,
                start_distnce,
                stop_distance,
                dest.inner,
                starts_with_move,
            )
        }
    }
    pub fn is_closed(&mut self) -> bool {
        unsafe { sk_pathmeasure_is_closed(self.inner) }
    }
    pub fn next_contour(&mut self) -> bool {
        unsafe { sk_pathmeasure_next_contour(self.inner) }
    }
}
