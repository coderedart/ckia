use crate::{bindings::*, ColorChannel};

use crate::{
    color::Color, shader::Shader, skia_wrapper, BlendMode, BlurStyle, Highcontrastconfig, Rect,
    ShaderTileMode, SkiaOptPtr,
};

skia_wrapper!(
    refcnt,
    MaskFilter,
    sk_maskfilter_t,
    sk_maskfilter_unref,
    sk_maskfilter_ref
);

impl MaskFilter {
    pub fn new_blur(blur: BlurStyle, sigma: f32) -> Self {
        unsafe { Self::from_owned_ptr(sk_maskfilter_new_blur(blur, sigma)) }
    }
    pub fn new_blur_with_flags(blur: BlurStyle, sigma: f32, respect_c_t_m: bool) -> Self {
        unsafe {
            Self::from_owned_ptr(sk_maskfilter_new_blur_with_flags(
                blur,
                sigma,
                respect_c_t_m,
            ))
        }
    }
    pub fn new_table(table: &[u8; 256]) -> Self {
        // assert!(table.len() >= 256);
        unsafe { Self::from_owned_ptr(sk_maskfilter_new_table(table.as_ptr())) }
    }
    pub fn new_gamma(gamma: f32) -> Self {
        unsafe { Self::from_owned_ptr(sk_maskfilter_new_gamma(gamma)) }
    }
    pub fn new_clip(min: u8, max: u8) -> Self {
        unsafe { Self::from_owned_ptr(sk_maskfilter_new_clip(min, max)) }
    }
    pub fn new_shader(shader: &Shader) -> Self {
        unsafe { Self::from_owned_ptr(sk_maskfilter_new_shader(shader.inner)) }
    }
}

skia_wrapper!(refcnt, ColorFilter, sk_colorfilter_t, sk_colorfilter_unref);

impl ColorFilter {
    pub fn new_mode(color: Color, mode: BlendMode) -> Self {
        unsafe { Self::from_owned_ptr(sk_colorfilter_new_mode(color.0, mode)) }
    }
    pub fn new_lighting(mul: Color, add: Color) -> Self {
        unsafe { Self::from_owned_ptr(sk_colorfilter_new_lighting(mul.0, add.0)) }
    }
    pub fn new_compose(outer: &Self, inner: &Self) -> Self {
        unsafe { Self::from_owned_ptr(sk_colorfilter_new_compose(outer.inner, inner.inner)) }
    }
    pub fn new_color_matrix(array: &[f32]) -> Self {
        assert!(array.len() >= 20);
        unsafe { Self::from_owned_ptr(sk_colorfilter_new_color_matrix(array.as_ptr())) }
    }
    pub fn new_luma_color() -> Self {
        unsafe { Self::from_owned_ptr(sk_colorfilter_new_luma_color()) }
    }
    pub fn new_high_contrast(config: &Highcontrastconfig) -> Self {
        unsafe { Self::from_owned_ptr(sk_colorfilter_new_high_contrast(config.as_ptr())) }
    }

    pub fn new_table(table: &[u8]) -> Self {
        assert!(table.len() >= 256);
        unsafe { Self::from_owned_ptr(sk_colorfilter_new_table(table.as_ptr())) }
    }
    pub fn new_table_argb(table_a: &[u8], table_r: &[u8], table_g: &[u8], table_b: &[u8]) -> Self {
        assert!(table_a.len() >= 256);
        assert!(table_r.len() >= 256);
        assert!(table_g.len() >= 256);
        assert!(table_b.len() >= 256);
        unsafe {
            Self::from_owned_ptr(sk_colorfilter_new_table_argb(
                table_a.as_ptr(),
                table_r.as_ptr(),
                table_g.as_ptr(),
                table_b.as_ptr(),
            ))
        }
    }
}
skia_wrapper!(refcnt, ImageFilter, sk_imagefilter_t, sk_imagefilter_unref);
impl ImageFilter {
    pub fn new_arithmetic(
        k1: f32,
        k2: f32,
        k3: f32,
        k4: f32,
        enforce_pm_color: bool,
        background: Option<&Self>,
        foreground: Option<&Self>,
        crop_rect: Option<&Rect>,
    ) -> Self {
        unsafe {
            Self::from_owned_ptr(sk_imagefilter_new_arithmetic(
                k1,
                k2,
                k3,
                k4,
                enforce_pm_color,
                background.or_null(),
                foreground.or_null(),
                crop_rect.or_null(),
            ))
        }
    }
    pub fn new_blend(
        mode: BlendMode,
        background: Option<&Self>,
        foreground: Option<&Self>,
        crop_rect: Option<&Rect>,
    ) -> Self {
        unsafe {
            Self::from_owned_ptr(sk_imagefilter_new_blend(
                mode,
                background.or_null(),
                foreground.or_null(),
                crop_rect.or_null(),
            ))
        }
    }
    pub fn new_blur(
        sigma_x: f32,
        sigma_y: f32,
        tile_mode: ShaderTileMode,
        input: Option<&Self>,
        crop_rect: Option<&Rect>,
    ) -> Self {
        unsafe {
            Self::from_owned_ptr(sk_imagefilter_new_blur(
                sigma_x,
                sigma_y,
                tile_mode,
                input.or_null(),
                crop_rect.or_null(),
            ))
        }
    }
    pub fn new_color_filter(
        cf: &mut ColorFilter,
        input: Option<&Self>,
        crop_rect: Option<&Rect>,
    ) -> Self {
        unsafe {
            Self::from_owned_ptr(sk_imagefilter_new_color_filter(
                cf.as_ptr_mut(),
                input.or_null(),
                crop_rect.or_null(),
            ))
        }
    }
    pub fn compose(&self, inner: &Self) -> Self {
        unsafe { Self::from_owned_ptr(sk_imagefilter_new_compose(self.as_ptr(), inner.as_ptr())) }
    }
    pub fn new_displacement_map_effect(
        x_channel_selector: ColorChannel,
        y_channel_selector: ColorChannel,
        scale: f32,
        displacement: Option<&Self>,
        color: Option<&Self>,
        crop_rect: Option<&Rect>,
    ) -> Self {
        unsafe {
            Self::from_owned_ptr(sk_imagefilter_new_displacement_map_effect(
                x_channel_selector,
                y_channel_selector,
                scale,
                displacement.or_null(),
                color.or_null(),
                crop_rect.or_null(),
            ))
        }
    }
    pub fn new_drop_shadow(
        dx: f32,
        dy: f32,
        sigma_x: f32,
        sigma_y: f32,
        color: Color,
        input: Option<&Self>,
        crop_rect: Option<&Rect>,
    ) -> Self {
        unsafe {
            Self::from_owned_ptr(sk_imagefilter_new_drop_shadow(
                dx,
                dy,
                sigma_x,
                sigma_y,
                color.as_u32(),
                input.or_null(),
                crop_rect.or_null(),
            ))
        }
    }
    pub fn new_drop_shadow_only(
        dx: f32,
        dy: f32,
        sigma_x: f32,
        sigma_y: f32,
        color: Color,
        input: Option<&Self>,
        crop_rect: Option<&Rect>,
    ) -> Self {
        unsafe {
            Self::from_owned_ptr(sk_imagefilter_new_drop_shadow_only(
                dx,
                dy,
                sigma_x,
                sigma_y,
                color.as_u32(),
                input.or_null(),
                crop_rect.or_null(),
            ))
        }
    }
    /*
    pub fn sk_imagefilter_new_image(
        image: *mut sk_image_t,
        srcRect: *const sk_rect_t,
        dstRect: *const sk_rect_t,
        sampling: *const sk_sampling_options_t,
    ) -> *mut sk_imagefilter_t;
    pub fn sk_imagefilter_new_image_simple(
        image: *mut sk_image_t,
        sampling: *const sk_sampling_options_t,
    ) -> *mut sk_imagefilter_t;
    pub fn sk_imagefilter_new_magnifier(
        lensBounds: *const sk_rect_t,
        zoomAmount: f32,
        inset: f32,
        sampling: *const sk_sampling_options_t,
        input: *const sk_imagefilter_t,
        cropRect: *const sk_rect_t,
    ) -> *mut sk_imagefilter_t;
    pub fn sk_imagefilter_new_matrix_convolution(
        kernelSize: *const sk_isize_t,
        kernel: *const f32,
        gain: f32,
        bias: f32,
        kernelOffset: *const sk_ipoint_t,
        ctileMode: sk_shader_tilemode_t,
        convolveAlpha: bool,
        input: *const sk_imagefilter_t,
        cropRect: *const sk_rect_t,
    ) -> *mut sk_imagefilter_t;
    pub fn sk_imagefilter_new_matrix_transform(
        cmatrix: *const sk_matrix_t,
        sampling: *const sk_sampling_options_t,
        input: *const sk_imagefilter_t,
    ) -> *mut sk_imagefilter_t;
    pub fn sk_imagefilter_new_merge(
        cfilters: *mut *const sk_imagefilter_t,
        count: ::std::os::raw::c_int,
        cropRect: *const sk_rect_t,
    ) -> *mut sk_imagefilter_t;
    pub fn sk_imagefilter_new_merge_simple(
        first: *const sk_imagefilter_t,
        second: *const sk_imagefilter_t,
        cropRect: *const sk_rect_t,
    ) -> *mut sk_imagefilter_t;
    pub fn sk_imagefilter_new_offset(
        dx: f32,
        dy: f32,
        input: *const sk_imagefilter_t,
        cropRect: *const sk_rect_t,
    ) -> *mut sk_imagefilter_t;
    pub fn sk_imagefilter_new_picture(picture: *const sk_picture_t) -> *mut sk_imagefilter_t;
    pub fn sk_imagefilter_new_picture_with_rect(
        picture: *const sk_picture_t,
        targetRect: *const sk_rect_t,
    ) -> *mut sk_imagefilter_t;
    pub fn sk_imagefilter_new_shader(
        shader: *const sk_shader_t,
        dither: bool,
        cropRect: *const sk_rect_t,
    ) -> *mut sk_imagefilter_t;
    pub fn sk_imagefilter_new_tile(
        src: *const sk_rect_t,
        dst: *const sk_rect_t,
        input: *const sk_imagefilter_t,
    ) -> *mut sk_imagefilter_t;
    pub fn sk_imagefilter_new_dilate(
        radiusX: f32,
        radiusY: f32,
        input: *const sk_imagefilter_t,
        cropRect: *const sk_rect_t,
    ) -> *mut sk_imagefilter_t;
    pub fn sk_imagefilter_new_erode(
        radiusX: f32,
        radiusY: f32,
        input: *const sk_imagefilter_t,
        cropRect: *const sk_rect_t,
    ) -> *mut sk_imagefilter_t;
    pub fn sk_imagefilter_new_distant_lit_diffuse(
        direction: *const sk_point3_t,
        lightColor: sk_color_t,
        surfaceScale: f32,
        kd: f32,
        input: *const sk_imagefilter_t,
        cropRect: *const sk_rect_t,
    ) -> *mut sk_imagefilter_t;
    pub fn sk_imagefilter_new_point_lit_diffuse(
        location: *const sk_point3_t,
        lightColor: sk_color_t,
        surfaceScale: f32,
        kd: f32,
        input: *const sk_imagefilter_t,
        cropRect: *const sk_rect_t,
    ) -> *mut sk_imagefilter_t;
    pub fn sk_imagefilter_new_spot_lit_diffuse(
        location: *const sk_point3_t,
        target: *const sk_point3_t,
        specularExponent: f32,
        cutoffAngle: f32,
        lightColor: sk_color_t,
        surfaceScale: f32,
        kd: f32,
        input: *const sk_imagefilter_t,
        cropRect: *const sk_rect_t,
    ) -> *mut sk_imagefilter_t;
    pub fn sk_imagefilter_new_distant_lit_specular(
        direction: *const sk_point3_t,
        lightColor: sk_color_t,
        surfaceScale: f32,
        ks: f32,
        shininess: f32,
        input: *const sk_imagefilter_t,
        cropRect: *const sk_rect_t,
    ) -> *mut sk_imagefilter_t;
    pub fn sk_imagefilter_new_point_lit_specular(
        location: *const sk_point3_t,
        lightColor: sk_color_t,
        surfaceScale: f32,
        ks: f32,
        shininess: f32,
        input: *const sk_imagefilter_t,
        cropRect: *const sk_rect_t,
    ) -> *mut sk_imagefilter_t;
    pub fn sk_imagefilter_new_spot_lit_specular(
        location: *const sk_point3_t,
        target: *const sk_point3_t,
        specularExponent: f32,
        cutoffAngle: f32,
        lightColor: sk_color_t,
        surfaceScale: f32,
        ks: f32,
        shininess: f32,
        input: *const sk_imagefilter_t,
        cropRect: *const sk_rect_t,
    ) -> *mut sk_imagefilter_t;
     */
}
