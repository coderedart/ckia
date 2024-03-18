use crate::{bindings::*, SkiaOptPtr};

use crate::{
    bitmap::BitMap, color::Color, font::Font, image::Image, paint::Paint, path::SkiaPath,
    picture::Picture, region::Region, rrect::RRect, skia_wrapper, text_blob::TextBlob, BlendMode,
    ClipOp, Color4f, IRect, Matrix, Matrix44, Point, PointMode, Rect, SamplingOptions,
    TextEncoding,
};

skia_wrapper!(unique, Canvas, sk_canvas_t, sk_canvas_destroy);

impl Canvas {
    pub fn from_bitmap(bitmap: &BitMap) -> Self {
        unsafe {
            Self {
                inner: sk_canvas_new_from_bitmap(bitmap.inner), // the fn will inc ref count
            }
        }
    }
    pub fn clear(&mut self, color: Color) {
        unsafe {
            sk_canvas_clear(self.inner, color.0);
        }
    }
    pub fn clear_color4f(&mut self, color: Color4f) {
        unsafe {
            sk_canvas_clear_color4f(self.inner, color);
        }
    }
    pub fn discard(&mut self) {
        unsafe { sk_canvas_discard(self.inner) }
    }
    pub fn get_save_count(&mut self) -> i32 {
        unsafe { sk_canvas_get_save_count(self.inner) }
    }
    pub fn restore_to_count(&mut self, save_count: i32) {
        unsafe { sk_canvas_restore_to_count(self.inner, save_count) }
    }
    pub fn draw_color(&mut self, color: Color, mode: BlendMode) {
        unsafe {
            sk_canvas_draw_color(self.inner, color.0, mode);
        }
    }
    pub fn draw_color4f(&mut self, color: Color4f, mode: BlendMode) {
        unsafe {
            sk_canvas_draw_color4f(self.inner, color, mode);
        }
    }
    pub fn draw_points(&mut self, mode: PointMode, points: &[Point], paint: &Paint) {
        unsafe {
            sk_canvas_draw_points(
                self.as_ptr_mut(),
                mode,
                points.len(),
                points.as_ptr() as _,
                paint.as_ptr(),
            )
        }
    }
    pub fn draw_point(&mut self, x: f32, y: f32, paint: &Paint) {
        unsafe { sk_canvas_draw_point(self.as_ptr_mut(), x, y, paint.as_ptr()) }
    }
    pub fn draw_line(&mut self, x0: f32, y0: f32, x1: f32, y1: f32, paint: &mut Paint) {
        unsafe { sk_canvas_draw_line(self.as_ptr_mut(), x0, y0, x1, y1, paint.as_ptr_mut()) }
    }

    pub fn draw_simple_text(&mut self, text: &str, x: f32, y: f32, font: &Font, paint: &Paint) {
        unsafe {
            sk_canvas_draw_simple_text(
                self.as_ptr_mut(),
                text.as_ptr() as _,
                text.len(),
                TextEncoding::UTF8_SK_TEXT_ENCODING,
                x,
                y,
                font.as_ptr(),
                paint.as_ptr(),
            );
        }
    }
    pub fn draw_text_blob(&mut self, text: &mut TextBlob, x: f32, y: f32, paint: &Paint) {
        unsafe {
            sk_canvas_draw_text_blob(self.as_ptr_mut(), text.as_ptr_mut(), x, y, paint.as_ptr())
        }
    }
    pub fn reset_matrix(&mut self) {
        unsafe { sk_canvas_reset_matrix(self.as_ptr_mut()) }
    }
    pub fn set_matrix(&mut self, mat: &Matrix44) {
        unsafe { sk_canvas_set_matrix(self.as_ptr_mut(), mat.as_ptr()) }
    }
    pub fn get_matrix(&mut self) -> Matrix44 {
        let mut mat = Matrix44::default();
        unsafe {
            sk_canvas_get_matrix(self.as_ptr_mut(), mat.as_ptr_mut());
        }
        mat
    }
    pub fn draw_round_rect(&mut self, rect: &Rect, rx: f32, ry: f32, paint: &Paint) {
        unsafe {
            sk_canvas_draw_round_rect(self.as_ptr_mut(), rect.as_ptr(), rx, ry, paint.as_ptr())
        }
    }
    pub fn clip_rect_with_operation(&mut self, rect: &Rect, op: ClipOp, do_aa: bool) {
        unsafe { sk_canvas_clip_rect_with_operation(self.as_ptr_mut(), rect.as_ptr(), op, do_aa) }
    }
    pub fn clip_path_with_operation(&mut self, path: &SkiaPath, op: ClipOp, do_aa: bool) {
        unsafe { sk_canvas_clip_path_with_operation(self.as_ptr_mut(), path.as_ptr(), op, do_aa) }
    }
    pub fn get_local_clip_bounds(&mut self) -> Option<Rect> {
        let mut rect = Rect::default();
        unsafe {
            sk_canvas_get_local_clip_bounds(self.as_ptr_mut(), rect.as_ptr_mut()).then_some(rect)
        }
    }
    pub fn get_device_clip_bounds(&mut self) -> Option<IRect> {
        let mut rect = IRect::default();
        unsafe {
            sk_canvas_get_device_clip_bounds(self.as_ptr_mut(), rect.as_ptr_mut()).then_some(rect)
        }
    }
    pub fn save(&mut self) -> i32 {
        unsafe { sk_canvas_save(self.as_ptr_mut()) }
    }
    pub fn save_layer(&mut self, rect: Option<&Rect>, paint: Option<&Paint>) -> i32 {
        unsafe { sk_canvas_save_layer(self.as_ptr_mut(), rect.or_null(), paint.or_null()) }
    }
    pub fn restore(&mut self) {
        unsafe { sk_canvas_restore(self.as_ptr_mut()) }
    }
    pub fn translate(&mut self, dx: f32, dy: f32) {
        unsafe { sk_canvas_translate(self.as_ptr_mut(), dx, dy) }
    }
    pub fn scale(&mut self, sx: f32, sy: f32) {
        unsafe { sk_canvas_scale(self.as_ptr_mut(), sx, sy) }
    }
    pub fn rotate_degrees(&mut self, degrees: f32) {
        unsafe { sk_canvas_rotate_degrees(self.as_ptr_mut(), degrees) }
    }
    pub fn rotate_radians(&mut self, radians: f32) {
        unsafe { sk_canvas_rotate_radians(self.as_ptr_mut(), radians) }
    }
    pub fn skew(&mut self, sx: f32, sy: f32) {
        unsafe { sk_canvas_skew(self.as_ptr_mut(), sx, sy) }
    }
    pub fn concat(&mut self, mat: &Matrix44) {
        unsafe { sk_canvas_concat(self.as_ptr_mut(), mat.as_ptr()) }
    }
    pub fn quick_reject(&mut self, rect: &Rect) -> bool {
        unsafe { sk_canvas_quick_reject(self.as_ptr_mut(), rect.as_ptr()) }
    }
    pub fn clip_region(&mut self, region: &Region, op: ClipOp) {
        unsafe { sk_canvas_clip_region(self.as_ptr_mut(), region.as_ptr(), op) }
    }
    pub fn draw_paint(&mut self, paint: &Paint) {
        unsafe { sk_canvas_draw_paint(self.as_ptr_mut(), paint.as_ptr()) }
    }
    pub fn draw_region(&mut self, region: &Region, paint: &Paint) {
        unsafe { sk_canvas_draw_region(self.as_ptr_mut(), region.as_ptr(), paint.as_ptr()) }
    }
    pub fn draw_rect(&mut self, rect: &Rect, paint: &Paint) {
        unsafe { sk_canvas_draw_rect(self.as_ptr_mut(), rect.as_ptr(), paint.as_ptr()) }
    }
    pub fn draw_rrect(&mut self, rrect: &RRect, paint: &Paint) {
        unsafe { sk_canvas_draw_rrect(self.as_ptr_mut(), rrect.as_ptr(), paint.as_ptr()) }
    }
    pub fn draw_circle(&mut self, cx: f32, cy: f32, rad: f32, paint: &Paint) {
        unsafe { sk_canvas_draw_circle(self.as_ptr_mut(), cx, cy, rad, paint.as_ptr()) }
    }
    pub fn draw_oval(&mut self, rect: &Rect, paint: &Paint) {
        unsafe { sk_canvas_draw_oval(self.as_ptr_mut(), rect.as_ptr(), paint.as_ptr()) }
    }
    pub fn draw_path(&mut self, path: &SkiaPath, paint: &Paint) {
        unsafe { sk_canvas_draw_path(self.as_ptr_mut(), path.as_ptr(), paint.as_ptr()) }
    }
    pub fn draw_image(
        &mut self,
        image: &Image,
        x: f32,
        y: f32,
        sampling: &SamplingOptions,
        paint: &Paint,
    ) {
        unsafe {
            sk_canvas_draw_image(
                self.as_ptr_mut(),
                image.as_ptr(),
                x,
                y,
                sampling.as_ptr(),
                paint.as_ptr(),
            )
        }
    }
    pub fn draw_image_rect(
        &mut self,
        image: &Image,
        src: &Rect,
        dst: &Rect,
        sampling: &SamplingOptions,
        paint: &Paint,
    ) {
        unsafe {
            sk_canvas_draw_image_rect(
                self.as_ptr_mut(),
                image.as_ptr(),
                src.as_ptr(),
                dst.as_ptr(),
                sampling.as_ptr(),
                paint.as_ptr(),
            )
        }
    }
    pub fn draw_picture(&mut self, picture: &Picture, mat: &Matrix, paint: &Paint) {
        unsafe {
            sk_canvas_draw_picture(
                self.as_ptr_mut(),
                picture.as_ptr(),
                mat.as_ptr(),
                paint.as_ptr(),
            )
        }
    }
    /*
    pub fn sk_canvas_draw_drawable(
        ccanvas: *mut sk_canvas_t,
        cdrawable: *mut sk_drawable_t,
        cmatrix: *const sk_matrix_t,
    );
    */
    pub fn flush(&mut self) {
        unsafe { sk_canvas_flush(self.as_ptr_mut()) }
    }

    /*
    pub fn sk_canvas_new_from_raster(
        cinfo: *const sk_imageinfo_t,
        pixels: *mut ::std::os::raw::c_void,
        rowBytes: usize,
        props: *const sk_surfaceprops_t,
    ) -> *mut sk_canvas_t;
    pub fn sk_canvas_draw_annotation(
        t: *mut sk_canvas_t,
        rect: *const sk_rect_t,
        key: *const ::std::os::raw::c_char,
        value: *mut sk_data_t,
    );
    pub fn sk_canvas_draw_url_annotation(
        t: *mut sk_canvas_t,
        rect: *const sk_rect_t,
        value: *mut sk_data_t,
    );
    pub fn sk_canvas_draw_named_destination_annotation(
        t: *mut sk_canvas_t,
        point: *const sk_point_t,
        value: *mut sk_data_t,
    );
    pub fn sk_canvas_draw_link_destination_annotation(
        t: *mut sk_canvas_t,
        rect: *const sk_rect_t,
        value: *mut sk_data_t,
    );
    pub fn sk_canvas_draw_image_lattice(
        ccanvas: *mut sk_canvas_t,
        image: *const sk_image_t,
        lattice: *const sk_lattice_t,
        dst: *const sk_rect_t,
        mode: sk_filter_mode_t,
        paint: *const sk_paint_t,
    );
    pub fn sk_canvas_draw_image_nine(
        ccanvas: *mut sk_canvas_t,
        image: *const sk_image_t,
        center: *const sk_irect_t,
        dst: *const sk_rect_t,
        mode: sk_filter_mode_t,
        paint: *const sk_paint_t,
    );
    pub fn sk_canvas_draw_vertices(
        ccanvas: *mut sk_canvas_t,
        vertices: *const sk_vertices_t,
        mode: sk_blendmode_t,
        paint: *const sk_paint_t,
    );
    pub fn sk_canvas_draw_arc(
        ccanvas: *mut sk_canvas_t,
        oval: *const sk_rect_t,
        startAngle: f32,
        sweepAngle: f32,
        useCenter: bool,
        paint: *const sk_paint_t,
    );
    pub fn sk_canvas_draw_drrect(
        ccanvas: *mut sk_canvas_t,
        outer: *const sk_rrect_t,
        inner: *const sk_rrect_t,
        paint: *const sk_paint_t,
    );
    pub fn sk_canvas_draw_atlas(
        ccanvas: *mut sk_canvas_t,
        atlas: *const sk_image_t,
        xform: *const sk_rsxform_t,
        tex: *const sk_rect_t,
        colors: *const sk_color_t,
        count: ::std::os::raw::c_int,
        mode: sk_blendmode_t,
        sampling: *const sk_sampling_options_t,
        cullRect: *const sk_rect_t,
        paint: *const sk_paint_t,
    );
    pub fn sk_canvas_draw_patch(
        ccanvas: *mut sk_canvas_t,
        cubics: *const sk_point_t,
        colors: *const sk_color_t,
        texCoords: *const sk_point_t,
        mode: sk_blendmode_t,
        paint: *const sk_paint_t,
    );
     */
    pub fn is_clip_empty(&mut self) -> bool {
        unsafe { sk_canvas_is_clip_empty(self.as_ptr_mut()) }
    }
    pub fn is_clip_rect(&mut self) -> bool {
        unsafe { sk_canvas_is_clip_rect(self.as_ptr_mut()) }
    }
}
