use crate::image::Image;
use crate::picture::Picture;
use crate::{bindings::*, ColorChannel, IPoint, ISize, Matrix, Point3, SamplingOptions};

use crate::{
    color::Color, shader::Shader, skia_wrapper, BlendMode, BlurStyle, Highcontrastconfig, Rect,
    ShaderTileMode, SkiaOptPtr,
};
use crate::{SkiaPtr, SkiaPtrMut};

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
    pub fn new_image(
        img: &mut Image,
        src_rect: &Rect,
        dst_rect: &Rect,
        sampling: &SamplingOptions,
    ) -> Self {
        unsafe {
            Self::from_owned_ptr(sk_imagefilter_new_image(
                img.as_ptr_mut(),
                src_rect.as_ptr(),
                dst_rect.as_ptr(),
                sampling.as_ptr(),
            ))
        }
    }
    pub fn new_image_simple(img: &mut Image, sampling: &SamplingOptions) -> Self {
        unsafe {
            Self::from_owned_ptr(sk_imagefilter_new_image_simple(
                img.as_ptr_mut(),
                sampling.as_ptr(),
            ))
        }
    }
    pub fn new_magnifier(
        lens_bounds: &Rect,
        zoom_amount: f32,
        inset: f32,
        sampling: &SamplingOptions,
        input: Option<&Self>,
        crop_rect: Option<&Rect>,
    ) -> Self {
        unsafe {
            Self::from_owned_ptr(sk_imagefilter_new_magnifier(
                lens_bounds.as_ptr(),
                zoom_amount,
                inset,
                sampling.as_ptr(),
                input.or_null(),
                crop_rect.or_null(),
            ))
        }
    }

    pub fn new_matrix_convolution(
        kernel_size: &ISize,
        kernel: &[f32],
        gain: f32,
        bias: f32,
        kernel_offset: &IPoint,
        tile_mode: ShaderTileMode,
        convolve_alpha: bool,
        input: Option<&Self>,
        crop_rect: Option<&Rect>,
    ) -> Self {
        assert_eq!(
            kernel.len(),
            (kernel_size.w * kernel_size.h) as usize,
            "Kernel size does not match specified dimensions"
        );
        unsafe {
            Self::from_owned_ptr(sk_imagefilter_new_matrix_convolution(
                kernel_size.as_ptr(),
                kernel.as_ptr(),
                gain,
                bias,
                kernel_offset.as_ptr(),
                tile_mode,
                convolve_alpha,
                input.or_null(),
                crop_rect.or_null(),
            ))
        }
    }
    pub fn new_matrix_transform(
        cmatrix: &Matrix,
        sampling: &SamplingOptions,
        input: Option<&Self>,
    ) -> Self {
        unsafe {
            Self::from_owned_ptr(sk_imagefilter_new_matrix_transform(
                cmatrix.as_ptr(),
                sampling.as_ptr(),
                input.or_null(),
            ))
        }
    }

    pub fn new_merge(filters: &[&Self], crop_rect: Option<&Rect>) -> Self {
        unsafe {
            let mut cfilters: Vec<*const sk_imagefilter_t> =
                filters.iter().map(|f| f.as_ptr()).collect();
            Self::from_owned_ptr(sk_imagefilter_new_merge(
                cfilters.as_mut_ptr(),
                cfilters.len() as ::std::os::raw::c_int,
                crop_rect.or_null(),
            ))
        }
    }

    pub fn new_merge_simple(first: &Self, second: &Self, crop_rect: Option<&Rect>) -> Self {
        unsafe {
            Self::from_owned_ptr(sk_imagefilter_new_merge_simple(
                first.as_ptr(),
                second.as_ptr(),
                crop_rect.or_null(),
            ))
        }
    }
    pub fn new_offset(dx: f32, dy: f32, input: Option<&Self>, crop_rect: Option<&Rect>) -> Self {
        unsafe {
            Self::from_owned_ptr(sk_imagefilter_new_offset(
                dx,
                dy,
                input.or_null(),
                crop_rect.or_null(),
            ))
        }
    }

    pub fn new_picture(picture: &Picture) -> Self {
        unsafe { Self::from_owned_ptr(sk_imagefilter_new_picture(picture.as_ptr())) }
    }

    pub fn new_picture_with_rect(picture: &Picture, target_rect: &Rect) -> Self {
        unsafe {
            Self::from_owned_ptr(sk_imagefilter_new_picture_with_rect(
                picture.as_ptr(),
                target_rect.as_ptr(),
            ))
        }
    }

    pub fn new_shader(shader: &Shader, dither: bool, crop_rect: Option<&Rect>) -> Self {
        unsafe {
            Self::from_owned_ptr(sk_imagefilter_new_shader(
                shader.as_ptr(),
                dither,
                crop_rect.or_null(),
            ))
        }
    }

    pub fn new_tile(src: &Rect, dst: &Rect, input: Option<&Self>) -> Self {
        unsafe {
            Self::from_owned_ptr(sk_imagefilter_new_tile(
                src.as_ptr(),
                dst.as_ptr(),
                input.or_null(),
            ))
        }
    }

    pub fn new_dilate(
        radius_x: f32,
        radius_y: f32,
        input: Option<&Self>,
        crop_rect: Option<&Rect>,
    ) -> Self {
        unsafe {
            Self::from_owned_ptr(sk_imagefilter_new_dilate(
                radius_x,
                radius_y,
                input.or_null(),
                crop_rect.or_null(),
            ))
        }
    }

    pub fn new_erode(
        radius_x: f32,
        radius_y: f32,
        input: Option<&Self>,
        crop_rect: Option<&Rect>,
    ) -> Self {
        unsafe {
            Self::from_owned_ptr(sk_imagefilter_new_erode(
                radius_x,
                radius_y,
                input.or_null(),
                crop_rect.or_null(),
            ))
        }
    }

    pub fn new_distant_lit_diffuse(
        direction: &Point3,
        light_color: Color,
        surface_scale: f32,
        kd: f32,
        input: Option<&Self>,
        crop_rect: Option<&Rect>,
    ) -> Self {
        unsafe {
            Self::from_owned_ptr(sk_imagefilter_new_distant_lit_diffuse(
                direction.as_ptr(),
                light_color.as_u32(),
                surface_scale,
                kd,
                input.or_null(),
                crop_rect.or_null(),
            ))
        }
    }

    pub fn new_point_lit_diffuse(
        location: &Point3,
        light_color: Color,
        surface_scale: f32,
        kd: f32,
        input: Option<&Self>,
        crop_rect: Option<&Rect>,
    ) -> Self {
        unsafe {
            Self::from_owned_ptr(sk_imagefilter_new_point_lit_diffuse(
                location.as_ptr(),
                light_color.as_u32(),
                surface_scale,
                kd,
                input.or_null(),
                crop_rect.or_null(),
            ))
        }
    }
    pub fn new_spot_lit_diffuse(
        location: &Point3,
        target: &Point3,
        specular_exponent: f32,
        cutoff_angle: f32,
        light_color: Color,
        surface_scale: f32,
        kd: f32,
        input: Option<&Self>,
        crop_rect: Option<&Rect>,
    ) -> Self {
        unsafe {
            Self::from_owned_ptr(sk_imagefilter_new_spot_lit_diffuse(
                location.as_ptr(),
                target.as_ptr(),
                specular_exponent,
                cutoff_angle,
                light_color.as_u32(),
                surface_scale,
                kd,
                input.or_null(),
                crop_rect.or_null(),
            ))
        }
    }
    pub fn new_distant_lit_specular(
        direction: &Point3,
        light_color: Color,
        surface_scale: f32,
        ks: f32,
        shininess: f32,
        input: Option<&Self>,
        crop_rect: Option<&Rect>,
    ) -> Self {
        unsafe {
            Self::from_owned_ptr(sk_imagefilter_new_distant_lit_specular(
                direction.as_ptr(),
                light_color.as_u32(),
                surface_scale,
                ks,
                shininess,
                input.or_null(),
                crop_rect.or_null(),
            ))
        }
    }

    pub fn new_point_lit_specular(
        location: &Point3,
        light_color: Color,
        surface_scale: f32,
        ks: f32,
        shininess: f32,
        input: Option<&Self>,
        crop_rect: Option<&Rect>,
    ) -> Self {
        unsafe {
            Self::from_owned_ptr(sk_imagefilter_new_point_lit_specular(
                location.as_ptr(),
                light_color.as_u32(),
                surface_scale,
                ks,
                shininess,
                input.or_null(),
                crop_rect.or_null(),
            ))
        }
    }

    pub fn new_spot_lit_specular(
        location: &Point3,
        target: &Point3,
        specular_exponent: f32,
        cutoff_angle: f32,
        light_color: Color,
        surface_scale: f32,
        ks: f32,
        shininess: f32,
        input: Option<&Self>,
        crop_rect: Option<&Rect>,
    ) -> Self {
        unsafe {
            Self::from_owned_ptr(sk_imagefilter_new_spot_lit_specular(
                location.as_ptr(),
                target.as_ptr(),
                specular_exponent,
                cutoff_angle,
                light_color.as_u32(),
                surface_scale,
                ks,
                shininess,
                input.or_null(),
                crop_rect.or_null(),
            ))
        }
    }
}
