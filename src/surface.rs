use std::marker::PhantomData;

use crate::bindings::*;
use crate::{
    canvas::Canvas,
    color::ColorSpace,
    gr_context::{BackendRenderTarget, BackendTexture, GrRecordingContext},
    paint::Paint,
    ColorType, ImageInfo, PixelGeometry, SurfaceOrigin,
};
crate::skia_wrapper!(refcnt, Surface, sk_surface_t, sk_surface_unref);

impl Surface {
    pub fn new_null(width: u32, height: u32) -> Self {
        unsafe { Self::from_owned_ptr(sk_surface_new_null(width as _, height as _)) }
    }
    pub fn new_raster(info: &ImageInfo, row_bytes: usize, props: &SurfaceProps) -> Self {
        unsafe {
            Self::from_owned_ptr(sk_surface_new_raster(
                info.as_ptr(),
                row_bytes,
                props.as_ptr(),
            ))
        }
    }
    /*
    pub fn sk_surface_new_raster_direct(
        arg1: *const sk_imageinfo_t,
        pixels: *mut ::std::os::raw::c_void,
        rowBytes: usize,
        releaseProc: sk_surface_raster_release_proc,
        context: *mut ::std::os::raw::c_void,
        props: *const sk_surfaceprops_t,
    ) -> *mut sk_surface_t; */
    pub fn new_backend_texture(
        context: &mut GrRecordingContext,
        texture: &BackendTexture,
        origin: SurfaceOrigin,
        samples: i32,
        ct: ColorType,
        cs: &mut ColorSpace,
        props: &SurfaceProps,
    ) -> Self {
        unsafe {
            Self::from_owned_ptr(sk_surface_new_backend_texture(
                context.as_ptr_mut(),
                texture.as_ptr(),
                origin,
                samples,
                ct,
                cs.as_ptr_mut(),
                props.as_ptr(),
            ))
        }
    }
    pub fn new_backend_render_target(
        context: &mut GrRecordingContext,
        render_target: &BackendRenderTarget,
        origin: SurfaceOrigin,
        ct: ColorType,
        cs: &mut ColorSpace,
        props: &SurfaceProps,
    ) -> Self {
        unsafe {
            Self::from_owned_ptr(sk_surface_new_backend_render_target(
                context.as_ptr_mut(),
                render_target.as_ptr(),
                origin,
                ct,
                cs.as_ptr_mut(),
                props.as_ptr(),
            ))
        }
    }
    pub fn new_render_target(
        context: &mut GrRecordingContext,
        budgeted: bool,
        info: &ImageInfo,
        samples: i32,
        origin: SurfaceOrigin,
        props: &SurfaceProps,
        should_create_with_mips: bool,
    ) -> Self {
        unsafe {
            Self::from_owned_ptr(sk_surface_new_render_target(
                context.as_ptr_mut(),
                budgeted,
                info.as_ptr(),
                samples,
                origin,
                props.as_ptr(),
                should_create_with_mips,
            ))
        }
    }
    /*
    pub fn sk_surface_new_metal_layer(
        context: *mut gr_recording_context_t,
        layer: *const ::std::os::raw::c_void,
        origin: gr_surfaceorigin_t,
        sampleCount: ::std::os::raw::c_int,
        colorType: sk_colortype_t,
        colorspace: *mut sk_colorspace_t,
        props: *const sk_surfaceprops_t,
        drawable: *mut *const ::std::os::raw::c_void,
    ) -> *mut sk_surface_t;
    pub fn sk_surface_new_metal_view(
        context: *mut gr_recording_context_t,
        mtkView: *const ::std::os::raw::c_void,
        origin: gr_surfaceorigin_t,
        sampleCount: ::std::os::raw::c_int,
        colorType: sk_colortype_t,
        colorspace: *mut sk_colorspace_t,
        props: *const sk_surfaceprops_t,
    ) -> *mut sk_surface_t; */
    pub fn get_canvas(&mut self) -> impl AsMut<Canvas> {
        // hack to bind the lifetime of canvas to surface
        #[repr(transparent)]
        struct SurfaceCanvas<'a>(*mut sk_canvas_t, PhantomData<&'a mut Self>);
        impl<'a> AsMut<Canvas> for SurfaceCanvas<'a> {
            fn as_mut(&mut self) -> &mut Canvas {
                unsafe { std::mem::transmute(&mut self.0) }
            }
        }
        unsafe {
            let ptr = sk_surface_get_canvas(self.as_ptr_mut());
            assert!(!ptr.is_null());
            SurfaceCanvas(ptr, PhantomData)
        }
    }

    /*


      pub fn sk_surface_new_image_snapshot(arg1: *mut sk_surface_t) -> *mut sk_image_t;
      pub fn sk_surface_new_image_snapshot_with_crop(
          surface: *mut sk_surface_t,
          bounds: *const sk_irect_t,
      ) -> *mut sk_image_t;
    */
    pub fn draw(&mut self, canvas: &mut Canvas, x: f32, y: f32, paint: &Paint) {
        unsafe { sk_surface_draw(self.as_ptr_mut(), canvas.as_ptr_mut(), x, y, paint.as_ptr()) };
    }
    /*
       pub fn sk_surface_peek_pixels(surface: *mut sk_surface_t, pixmap: *mut sk_pixmap_t) -> bool;
       pub fn sk_surface_read_pixels(
           surface: *mut sk_surface_t,
           dstInfo: *mut sk_imageinfo_t,
           dstPixels: *mut ::std::os::raw::c_void,
           dstRowBytes: usize,
           srcX: ::std::os::raw::c_int,
           srcY: ::std::os::raw::c_int,
       ) -> bool;
       pub fn sk_surface_get_props(surface: *mut sk_surface_t) -> *const sk_surfaceprops_t;
       pub fn sk_surface_get_recording_context(
           surface: *mut sk_surface_t,
       ) -> *mut gr_recording_context_t;
    */
}
crate::skia_wrapper!(
    unique,
    SurfaceProps,
    sk_surfaceprops_t,
    sk_surfaceprops_delete
);

impl SurfaceProps {
    pub fn new(flags: u32, geometry: PixelGeometry) -> Self {
        unsafe { Self::from_owned_ptr(sk_surfaceprops_new(flags, geometry)) }
    }
    pub fn get_flags(&mut self) -> u32 {
        unsafe { sk_surfaceprops_get_flags(self.as_ptr_mut()) }
    }
    pub fn get_pixel_geometry(&mut self) -> PixelGeometry {
        unsafe { sk_surfaceprops_get_pixel_geometry(self.as_ptr_mut()) }
    }
}
