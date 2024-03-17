use crate::bindings::*;
use crate::{
    bitmap::BitMap,
    color::ColorSpace,
    data::SkiaData,
    gr_context::{DirectContext, GrRecordingContext},
    pixmap::PixMap,
    shader::Shader,
    AlphaType, ColorType, Matrix, SamplingOptions, ShaderTileMode,
};

use crate::ImageInfo;

crate::skia_wrapper!(refcnt, Image, sk_image_t, sk_image_unref, sk_image_ref);

impl Image {
    pub fn new_raster_copy(info: &ImageInfo, pixels: &[u8], row_bytes: usize) -> Option<Self> {
        unsafe {
            Self::try_from_owned_ptr(sk_image_new_raster_copy(
                info.as_ptr(),
                pixels.as_ptr() as _,
                row_bytes,
            ))
        }
    }
    pub fn new_raster_copy_with_pixmap(pixmap: &PixMap) -> Option<Self> {
        unsafe { Self::try_from_owned_ptr(sk_image_new_raster_copy_with_pixmap(pixmap.inner)) }
    }
    pub fn new_raster_data(
        info: &ImageInfo,
        data: &mut SkiaData,
        row_bytes: usize,
    ) -> Option<Self> {
        unsafe {
            Self::try_from_owned_ptr(sk_image_new_raster_data(
                info.as_ptr(),
                data.as_ptr_mut(),
                row_bytes,
            ))
        }
    }
    pub fn new_from_bitmap(bitmap: &BitMap) -> Option<Self> {
        unsafe { Self::try_from_owned_ptr(sk_image_new_from_bitmap(bitmap.as_ptr())) }
    }
    pub fn new_from_encoded(data: &SkiaData) -> Option<Self> {
        unsafe { Self::try_from_owned_ptr(sk_image_new_from_encoded(data.as_ptr())) }
    }

    /*
    pub fn sk_image_new_from_texture(
        context: *mut gr_recording_context_t,
        texture: *const gr_backendtexture_t,
        origin: gr_surfaceorigin_t,
        colorType: sk_colortype_t,
        alpha: sk_alphatype_t,
        colorSpace: *const sk_colorspace_t,
        releaseProc: sk_image_texture_release_proc,
        releaseContext: *mut ::std::os::raw::c_void,
    ) -> *mut sk_image_t;
    pub fn sk_image_new_from_adopted_texture(
        context: *mut gr_recording_context_t,
        texture: *const gr_backendtexture_t,
        origin: gr_surfaceorigin_t,
        colorType: sk_colortype_t,
        alpha: sk_alphatype_t,
        colorSpace: *const sk_colorspace_t,
    ) -> *mut sk_image_t;
    pub fn sk_image_new_from_picture(
        picture: *mut sk_picture_t,
        dimensions: *const sk_isize_t,
        cmatrix: *const sk_matrix_t,
        paint: *const sk_paint_t,
        useFloatingPointBitDepth: bool,
        colorSpace: *const sk_colorspace_t,
        props: *const sk_surfaceprops_t,
    ) -> *mut sk_image_t;
    */
    pub fn get_width(&self) -> i32 {
        unsafe { sk_image_get_width(self.as_ptr()) }
    }
    pub fn get_height(&self) -> i32 {
        unsafe { sk_image_get_height(self.as_ptr()) }
    }
    pub fn get_unique_id(&self) -> u32 {
        unsafe { sk_image_get_unique_id(self.as_ptr()) }
    }
    pub fn get_alpha_type(&self) -> AlphaType {
        unsafe { sk_image_get_alpha_type(self.as_ptr()) }
    }
    pub fn get_color_type(&self) -> ColorType {
        unsafe { sk_image_get_color_type(self.as_ptr()) }
    }
    pub fn get_color_space(&self) -> Option<ColorSpace> {
        unsafe { ColorSpace::try_from_owned_ptr(sk_image_get_colorspace(self.as_ptr())) }
    }
    pub fn is_alpha_only(&self) -> bool {
        unsafe { sk_image_is_alpha_only(self.as_ptr()) }
    }
    pub fn make_shader(
        &self,
        tile_x: ShaderTileMode,
        tile_y: ShaderTileMode,
        sampling: &SamplingOptions,
        mat: &Matrix,
    ) -> Option<Shader> {
        unsafe {
            Shader::try_from_owned_ptr(sk_image_make_shader(
                self.as_ptr(),
                tile_x,
                tile_y,
                sampling.as_ptr(),
                mat.as_ptr(),
            ))
        }
    }
    /*
    pub fn sk_image_peek_pixels(image: *const sk_image_t, pixmap: *mut sk_pixmap_t) -> bool;
    */
    pub fn is_texture_backed(&self) -> bool {
        unsafe { sk_image_is_texture_backed(self.as_ptr()) }
    }
    pub fn is_lazy_generated(&self) -> bool {
        unsafe { sk_image_is_lazy_generated(self.as_ptr()) }
    }
    pub fn is_valid(&self, ctx: &mut GrRecordingContext) -> bool {
        unsafe { sk_image_is_valid(self.as_ptr(), ctx.as_ptr_mut()) }
    }

    /*
    pub fn sk_image_read_pixels(
        image: *const sk_image_t,
        dstInfo: *const sk_imageinfo_t,
        dstPixels: *mut ::std::os::raw::c_void,
        dstRowBytes: usize,
        srcX: ::std::os::raw::c_int,
        srcY: ::std::os::raw::c_int,
        cachingHint: sk_image_caching_hint_t,
    ) -> bool;
    pub fn sk_image_read_pixels_into_pixmap(
        image: *const sk_image_t,
        dst: *const sk_pixmap_t,
        srcX: ::std::os::raw::c_int,
        srcY: ::std::os::raw::c_int,
        cachingHint: sk_image_caching_hint_t,
    ) -> bool;
    pub fn sk_image_scale_pixels(
        image: *const sk_image_t,
        dst: *const sk_pixmap_t,
        sampling: *const sk_sampling_options_t,
        cachingHint: sk_image_caching_hint_t,
    ) -> bool;
    pub fn sk_image_ref_encoded(cimage: *const sk_image_t) -> *mut sk_data_t;
    pub fn sk_image_make_subset_raster(
        cimage: *const sk_image_t,
        subset: *const sk_irect_t,
    ) -> *mut sk_image_t;
    pub fn sk_image_make_subset(
        cimage: *const sk_image_t,
        context: *mut gr_direct_context_t,
        subset: *const sk_irect_t,
    ) -> *mut sk_image_t;
    */
    pub fn make_texture_image(
        &self,
        ctx: &mut DirectContext,
        mipmapped: bool,
        budgeted: bool,
    ) -> Self {
        unsafe {
            Self::from_owned_ptr(sk_image_make_texture_image(
                self.as_ptr(),
                ctx.as_ptr_mut(),
                mipmapped,
                budgeted,
            ))
        }
    }
    /*
    pub fn sk_image_make_non_texture_image(cimage: *const sk_image_t) -> *mut sk_image_t;
    pub fn sk_image_make_raster_image(cimage: *const sk_image_t) -> *mut sk_image_t;
    pub fn sk_image_make_with_filter_raster(
        cimage: *const sk_image_t,
        filter: *const sk_imagefilter_t,
        subset: *const sk_irect_t,
        clipBounds: *const sk_irect_t,
        outSubset: *mut sk_irect_t,
        outOffset: *mut sk_ipoint_t,
    ) -> *mut sk_image_t;
    pub fn sk_image_make_with_filter(
        cimage: *const sk_image_t,
        context: *mut gr_recording_context_t,
        filter: *const sk_imagefilter_t,
        subset: *const sk_irect_t,
        clipBounds: *const sk_irect_t,
        outSubset: *mut sk_irect_t,
        outOffset: *mut sk_ipoint_t,
    ) -> *mut sk_image_t; */
}
