use ckia_sys::*;

use crate::{bitmap::BitMap, opaque_unique, BlendMode, Color, Color4f};

opaque_unique!(Canvas, sk_canvas_t, sk_canvas_destroy);

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
            sk_canvas_clear_color4f(self.inner, color.0);
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
            sk_canvas_draw_color4f(self.inner, color.0, mode);
        }
    }

    /*
    pub fn sk_canvas_draw_points(
        ccanvas: *mut sk_canvas_t,
        pointMode: sk_point_mode_t,
        count: usize,
        points: *const sk_point_t,
        cpaint: *const sk_paint_t,
    );
    pub fn sk_canvas_draw_point(
        ccanvas: *mut sk_canvas_t,
        x: f32,
        y: f32,
        cpaint: *const sk_paint_t,
    );
    pub fn sk_canvas_draw_line(
        ccanvas: *mut sk_canvas_t,
        x0: f32,
        y0: f32,
        x1: f32,
        y1: f32,
        cpaint: *mut sk_paint_t,
    );
    pub fn sk_canvas_draw_simple_text(
        ccanvas: *mut sk_canvas_t,
        text: *const ::std::os::raw::c_void,
        byte_length: usize,
        encoding: sk_text_encoding_t,
        x: f32,
        y: f32,
        cfont: *const sk_font_t,
        cpaint: *const sk_paint_t,
    );
    pub fn sk_canvas_draw_text_blob(
        ccanvas: *mut sk_canvas_t,
        text: *mut sk_textblob_t,
        x: f32,
        y: f32,
        cpaint: *const sk_paint_t,
    );
    pub fn sk_canvas_reset_matrix(ccanvas: *mut sk_canvas_t);
    pub fn sk_canvas_set_matrix(ccanvas: *mut sk_canvas_t, cmatrix: *const sk_matrix44_t);
    pub fn sk_canvas_get_matrix(ccanvas: *mut sk_canvas_t, cmatrix: *mut sk_matrix44_t);
    pub fn sk_canvas_draw_round_rect(
        ccanvas: *mut sk_canvas_t,
        crect: *const sk_rect_t,
        rx: f32,
        ry: f32,
        cpaint: *const sk_paint_t,
    );
    pub fn sk_canvas_clip_rect_with_operation(
        ccanvas: *mut sk_canvas_t,
        crect: *const sk_rect_t,
        op: sk_clipop_t,
        doAA: bool,
    );
    pub fn sk_canvas_clip_path_with_operation(
        ccanvas: *mut sk_canvas_t,
        cpath: *const sk_path_t,
        op: sk_clipop_t,
        doAA: bool,
    );
    pub fn sk_canvas_clip_rrect_with_operation(
        ccanvas: *mut sk_canvas_t,
        crect: *const sk_rrect_t,
        op: sk_clipop_t,
        doAA: bool,
    );
    pub fn sk_canvas_get_local_clip_bounds(
        ccanvas: *mut sk_canvas_t,
        cbounds: *mut sk_rect_t,
    ) -> bool;
    pub fn sk_canvas_get_device_clip_bounds(
        ccanvas: *mut sk_canvas_t,
        cbounds: *mut sk_irect_t,
    ) -> bool;
    pub fn sk_canvas_save(ccanvas: *mut sk_canvas_t) -> ::std::os::raw::c_int;
    pub fn sk_canvas_save_layer(
        ccanvas: *mut sk_canvas_t,
        crect: *const sk_rect_t,
        cpaint: *const sk_paint_t,
    ) -> ::std::os::raw::c_int;
    pub fn sk_canvas_restore(ccanvas: *mut sk_canvas_t);
    pub fn sk_canvas_translate(ccanvas: *mut sk_canvas_t, dx: f32, dy: f32);
    pub fn sk_canvas_scale(ccanvas: *mut sk_canvas_t, sx: f32, sy: f32);
    pub fn sk_canvas_rotate_degrees(ccanvas: *mut sk_canvas_t, degrees: f32);
    pub fn sk_canvas_rotate_radians(ccanvas: *mut sk_canvas_t, radians: f32);
    pub fn sk_canvas_skew(ccanvas: *mut sk_canvas_t, sx: f32, sy: f32);
    pub fn sk_canvas_concat(ccanvas: *mut sk_canvas_t, cmatrix: *const sk_matrix44_t);
    pub fn sk_canvas_quick_reject(ccanvas: *mut sk_canvas_t, crect: *const sk_rect_t) -> bool;
    pub fn sk_canvas_clip_region(
        ccanvas: *mut sk_canvas_t,
        region: *const sk_region_t,
        op: sk_clipop_t,
    );
    pub fn sk_canvas_draw_paint(ccanvas: *mut sk_canvas_t, cpaint: *const sk_paint_t);
    pub fn sk_canvas_draw_region(
        ccanvas: *mut sk_canvas_t,
        cregion: *const sk_region_t,
        cpaint: *const sk_paint_t,
    );
    pub fn sk_canvas_draw_rect(
        ccanvas: *mut sk_canvas_t,
        crect: *const sk_rect_t,
        cpaint: *const sk_paint_t,
    );
    pub fn sk_canvas_draw_rrect(
        ccanvas: *mut sk_canvas_t,
        crect: *const sk_rrect_t,
        cpaint: *const sk_paint_t,
    );
    pub fn sk_canvas_draw_circle(
        ccanvas: *mut sk_canvas_t,
        cx: f32,
        cy: f32,
        rad: f32,
        cpaint: *const sk_paint_t,
    );
    pub fn sk_canvas_draw_oval(
        ccanvas: *mut sk_canvas_t,
        crect: *const sk_rect_t,
        cpaint: *const sk_paint_t,
    );
    pub fn sk_canvas_draw_path(
        ccanvas: *mut sk_canvas_t,
        cpath: *const sk_path_t,
        cpaint: *const sk_paint_t,
    );
    pub fn sk_canvas_draw_image(
        ccanvas: *mut sk_canvas_t,
        cimage: *const sk_image_t,
        x: f32,
        y: f32,
        sampling: *const sk_sampling_options_t,
        cpaint: *const sk_paint_t,
    );
    pub fn sk_canvas_draw_image_rect(
        ccanvas: *mut sk_canvas_t,
        cimage: *const sk_image_t,
        csrcR: *const sk_rect_t,
        cdstR: *const sk_rect_t,
        sampling: *const sk_sampling_options_t,
        cpaint: *const sk_paint_t,
    );
    pub fn sk_canvas_draw_picture(
        ccanvas: *mut sk_canvas_t,
        cpicture: *const sk_picture_t,
        cmatrix: *const sk_matrix_t,
        cpaint: *const sk_paint_t,
    );
    pub fn sk_canvas_draw_drawable(
        ccanvas: *mut sk_canvas_t,
        cdrawable: *mut sk_drawable_t,
        cmatrix: *const sk_matrix_t,
    );
    pub fn sk_canvas_flush(ccanvas: *mut sk_canvas_t);
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
    pub fn sk_canvas_is_clip_empty(ccanvas: *mut sk_canvas_t) -> bool;
    pub fn sk_canvas_is_clip_rect(ccanvas: *mut sk_canvas_t) -> bool;
     */
}
