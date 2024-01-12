use ckia_sys::*;

use crate::{opaque_shared, paint::BlurStyle, shader::Shader, BlendMode, Color};

pub type HighContrastConfig = sk_highcontrastconfig_t;
pub type HighContrastConfigInvertStyle = sk_highcontrastconfig_invertstyle_t;
opaque_shared!(
    MaskFilter,
    sk_maskfilter_t,
    sk_maskfilter_unref,
    sk_maskfilter_ref
);

impl MaskFilter {
    pub fn new_blur(blur: BlurStyle, sigma: f32) -> Self {
        let inner = unsafe { sk_maskfilter_new_blur(blur, sigma) };
        assert!(!inner.is_null());
        Self { inner }
    }
    pub fn new_blur_with_flags(blur: BlurStyle, sigma: f32, respect_c_t_m: bool) -> Self {
        let inner = unsafe { sk_maskfilter_new_blur_with_flags(blur, sigma, respect_c_t_m) };
        assert!(!inner.is_null());
        Self { inner }
    }
    pub fn new_table(table: &[u8]) -> Self {
        assert!(table.len() >= 256);
        let inner = unsafe { sk_maskfilter_new_table(table.as_ptr()) };
        assert!(!inner.is_null());
        Self { inner }
    }
    pub fn new_gamma(gamma: f32) -> Self {
        let inner = unsafe { sk_maskfilter_new_gamma(gamma) };
        assert!(!inner.is_null());
        Self { inner }
    }
    pub fn new_clip(min: u8, max: u8) -> Self {
        let inner = unsafe { sk_maskfilter_new_clip(min, max) };
        assert!(!inner.is_null());
        Self { inner }
    }
    pub fn new_shader(shader: &Shader) -> Self {
        let inner = unsafe { sk_maskfilter_new_shader(shader.inner) };
        assert!(!inner.is_null());
        Self { inner }
    }
}

opaque_shared!(ColorFilter, sk_colorfilter_t, sk_colorfilter_unref);
impl ColorFilter {
    pub fn new_mode(color: Color, mode: BlendMode) -> Self {
        let inner = unsafe { sk_colorfilter_new_mode(color.0, mode) };
        assert!(!inner.is_null());
        Self { inner }
    }
    pub fn new_lighting(mul: Color, add: Color) -> Self {
        let inner = unsafe { sk_colorfilter_new_lighting(mul.0, add.0) };
        assert!(!inner.is_null());
        Self { inner }
    }
    pub fn new_compose(outer: &Self, inner: &Self) -> Self {
        let inner = unsafe { sk_colorfilter_new_compose(outer.inner, inner.inner) };
        assert!(!inner.is_null());
        Self { inner }
    }
    pub fn new_color_matrix(array: &[f32]) -> Self {
        assert!(array.len() >= 20);
        let inner = unsafe { sk_colorfilter_new_color_matrix(array.as_ptr()) };
        assert!(!inner.is_null());
        Self { inner }
    }
    pub fn new_luma_color() -> Self {
        let inner = unsafe { sk_colorfilter_new_luma_color() };
        assert!(!inner.is_null());
        Self { inner }
    }
    pub fn new_high_contrast(config: &HighContrastConfig) -> Self {
        let inner = unsafe { sk_colorfilter_new_high_contrast(config as _) };
        assert!(!inner.is_null());
        Self { inner }
    }

    pub fn new_table(table: &[u8]) -> Self {
        assert!(table.len() >= 256);
        let inner = unsafe { sk_colorfilter_new_table(table.as_ptr()) };
        assert!(!inner.is_null());
        Self { inner }
    }
    pub fn new_table_argb(table_a: &[u8], table_r: &[u8], table_g: &[u8], table_b: &[u8]) -> Self {
        assert!(table_a.len() >= 256);
        assert!(table_r.len() >= 256);
        assert!(table_g.len() >= 256);
        assert!(table_b.len() >= 256);
        let inner = unsafe {
            sk_colorfilter_new_table_argb(
                table_a.as_ptr(),
                table_r.as_ptr(),
                table_g.as_ptr(),
                table_b.as_ptr(),
            )
        };
        assert!(!inner.is_null());
        Self { inner }
    }
}
opaque_shared!(ImageFilter, sk_imagefilter_t, sk_imagefilter_unref);
impl ImageFilter {
    /*
    pub fn sk_imagefilter_new_alpha_threshold(
        region: *const sk_region_t,
        innerThreshold: f32,
        outerThreshold: f32,
        input: *const sk_imagefilter_t,
    ) -> *mut sk_imagefilter_t;
    pub fn sk_imagefilter_new_arithmetic(
        k1: f32,
        k2: f32,
        k3: f32,
        k4: f32,
        enforcePMColor: bool,
        background: *const sk_imagefilter_t,
        foreground: *const sk_imagefilter_t,
        cropRect: *const sk_rect_t,
    ) -> *mut sk_imagefilter_t;
    pub fn sk_imagefilter_new_blend(
        mode: sk_blendmode_t,
        background: *const sk_imagefilter_t,
        foreground: *const sk_imagefilter_t,
        cropRect: *const sk_rect_t,
    ) -> *mut sk_imagefilter_t;
    pub fn sk_imagefilter_new_blur(
        sigmaX: f32,
        sigmaY: f32,
        tileMode: sk_shader_tilemode_t,
        input: *const sk_imagefilter_t,
        cropRect: *const sk_rect_t,
    ) -> *mut sk_imagefilter_t;
    pub fn sk_imagefilter_new_color_filter(
        cf: *mut sk_colorfilter_t,
        input: *const sk_imagefilter_t,
        cropRect: *const sk_rect_t,
    ) -> *mut sk_imagefilter_t;
    pub fn sk_imagefilter_new_compose(
        outer: *const sk_imagefilter_t,
        inner: *const sk_imagefilter_t,
    ) -> *mut sk_imagefilter_t;
    pub fn sk_imagefilter_new_displacement_map_effect(
        xChannelSelector: sk_color_channel_t,
        yChannelSelector: sk_color_channel_t,
        scale: f32,
        displacement: *const sk_imagefilter_t,
        color: *const sk_imagefilter_t,
        cropRect: *const sk_rect_t,
    ) -> *mut sk_imagefilter_t;
    pub fn sk_imagefilter_new_drop_shadow(
        dx: f32,
        dy: f32,
        sigmaX: f32,
        sigmaY: f32,
        color: sk_color_t,
        input: *const sk_imagefilter_t,
        cropRect: *const sk_rect_t,
    ) -> *mut sk_imagefilter_t;
    pub fn sk_imagefilter_new_drop_shadow_only(
        dx: f32,
        dy: f32,
        sigmaX: f32,
        sigmaY: f32,
        color: sk_color_t,
        input: *const sk_imagefilter_t,
        cropRect: *const sk_rect_t,
    ) -> *mut sk_imagefilter_t;
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