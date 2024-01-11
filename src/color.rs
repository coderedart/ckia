use std::{marker::PhantomData, ops::Add};

use ckia_sys::*;

pub type TransferFn = sk_colorspace_transfer_fn_t;
pub type XYZ = sk_colorspace_xyz_t;

pub type ColorType = sk_colortype_t;
pub type AlphaType = sk_alphatype_t;
pub type BlendMode = sk_blendmode_t;

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct Color4f(pub(crate) sk_color4f_t);
impl Default for Color4f {
    fn default() -> Self {
        Self(sk_color4f_t {
            fR: 0.0,
            fG: 0.0,
            fB: 0.0,
            fA: 0.0,
        })
    }
}
impl From<Color> for Color4f {
    fn from(value: Color) -> Self {
        let mut c = Self::default();
        unsafe {
            sk_color4f_from_color(value.0, &mut c.0 as _);
        }
        c
    }
}
/// color type represented by u32 made up of ARGB components (1 byte each) in that particular order.
/// unmultiplied
#[derive(Debug, Clone, Copy, Default)]
#[repr(transparent)]
pub struct Color(pub(crate) sk_color_t);
#[derive(Debug, Clone, Copy, Default)]

/// Premultiplied color represented by u32. The color components order is based on the [ColorType] in use.
#[repr(transparent)]
pub struct PMColor(pub(crate) sk_pmcolor_t);

impl Color {
    /// Represents fully transparent SkColor. May be used to initialize a destination
    /// containing a mask or a non-rectangular image.
    pub const TRANSPARENT: Self = Self::new(0x00, 0x00, 0x00, 0x00);

    /// Represents fully opaque black.
    pub const BLACK: Self = Self::new(0xFF, 0x00, 0x00, 0x00);

    /// Represents fully opaque dark gray.
    /// Note that SVG dark gray is equivalent to 0xFFA9A9A9.
    pub const DKGRAY: Self = Self::new(0xFF, 0x44, 0x44, 0x44);

    /// Represents fully opaque gray.
    /// Note that HTML gray is equivalent to 0xFF808080.
    pub const GRAY: Self = Self::new(0xFF, 0x88, 0x88, 0x88);

    /// Represents fully opaque light gray. HTML silver is equivalent to 0xFFC0C0C0.
    /// Note that SVG light gray is equivalent to 0xFFD3D3D3.
    pub const LTGRAY: Self = Self::new(0xFF, 0xCC, 0xCC, 0xCC);

    /// Represents fully opaque white.
    pub const WHITE: Self = Self::new(0xFF, 0xFF, 0xFF, 0xFF);

    /// Represents fully opaque red.
    pub const RED: Self = Self::new(0xFF, 0xFF, 0x00, 0x00);

    /// Represents fully opaque green. HTML lime is equivalent.
    /// Note that HTML green is equivalent to 0xFF008000.
    pub const GREEN: Self = Self::new(0xFF, 0x00, 0xFF, 0x00);

    /// Represents fully opaque blue.
    pub const BLUE: Self = Self::new(0xFF, 0x00, 0x00, 0xFF);

    /// Represents fully opaque yellow.
    pub const YELLOW: Self = Self::new(0xFF, 0xFF, 0xFF, 0x00);

    /// Represents fully opaque cyan. HTML aqua is equivalent.
    pub const CYAN: Self = Self::new(0xFF, 0x00, 0xFF, 0xFF);

    /// Represents fully opaque magenta. HTML fuchsia is equivalent.
    pub const MAGENTA: Self = Self::new(0xFF, 0xFF, 0x00, 0xFF);

    /// you can think of this as `(a << 24) | (r << 16) | (g << 8) | (b << 0);`
    /// alpha at most significant 8 bits, r at next 8 bits ...
    pub const fn new(a: u8, r: u8, g: u8, b: u8) -> Self {
        let mut color = (a as u32) << 24;
        color |= (r as u32) << 16;
        color |= (g as u32) << 8;
        color |= (b as u32) << 0;
        Self(color)
    }
}
impl From<Color> for PMColor {
    fn from(value: Color) -> Self {
        unsafe { Self(sk_color_premultiply(value.0)) }
    }
}
impl From<PMColor> for Color {
    fn from(value: PMColor) -> Self {
        unsafe { Self(sk_color_unpremultiply(value.0)) }
    }
}
impl PMColor {
    /// The current default layout as the bitshifts of each of the color components in order ARGB
    /// eg: arr[0] is the number of bits shifted for alpha component. 2 is red, 3 green, 4 blue.
    pub fn get_bit_shift() -> [i32; 4] {
        let mut arr = [0i32; 4];
        unsafe {
            sk_color_get_bit_shift(
                &mut arr[0] as _,
                &mut arr[1] as _,
                &mut arr[2] as _,
                &mut arr[3] as _,
            );
        }
        arr
    }
}
pub fn color_type_default() -> ColorType {
    ColorType::BGRA_8888_SK_COLORTYPE
}
pub fn alpha_type_default() -> AlphaType {
    AlphaType::PREMUL_SK_ALPHATYPE
}
pub fn bytes_per_pixel(ct: ColorType) -> u8 {
    use sk_colortype_t::*;
    match ct {
        UNKNOWN_SK_COLORTYPE => 0,
        ALPHA_8_SK_COLORTYPE => 1,
        RGB_565_SK_COLORTYPE => 2,
        ARGB_4444_SK_COLORTYPE => 2,
        RGBA_8888_SK_COLORTYPE => 4,
        BGRA_8888_SK_COLORTYPE => 4,
        RGB_888X_SK_COLORTYPE => 4,
        RGBA_1010102_SK_COLORTYPE => 4,
        RGB_101010X_SK_COLORTYPE => 4,
        BGRA_1010102_SK_COLORTYPE => 4,
        BGR_101010X_SK_COLORTYPE => 4,
        BGR_101010X_XR_SK_COLORTYPE => 4,
        RGBA_10X6_SK_COLORTYPE => 8,
        GRAY_8_SK_COLORTYPE => 1,
        RGBA_F16_NORM_SK_COLORTYPE => 8,
        RGBA_F16_SK_COLORTYPE => 8,
        RGBA_F32_SK_COLORTYPE => 16,
        R8G8_UNORM_SK_COLORTYPE => 2,
        A16_UNORM_SK_COLORTYPE => 2,
        R16G16_UNORM_SK_COLORTYPE => 4,
        A16_FLOAT_SK_COLORTYPE => 2,
        R16G16_FLOAT_SK_COLORTYPE => 4,
        R16G16B16A16_UNORM_SK_COLORTYPE => 8,
        SRGBA_8888_SK_COLORTYPE => 4,
        R8_UNORM_SK_COLORTYPE => 1,
    }
}

#[repr(transparent)]
pub struct ColorSpaceXYZ(sk_colorspace_xyz_t);

impl Default for ColorSpaceXYZ {
    fn default() -> Self {
        Self(sk_colorspace_xyz_t {
            fM00: 0.0,
            fM01: 0.0,
            fM02: 0.0,
            fM10: 0.0,
            fM11: 0.0,
            fM12: 0.0,
            fM20: 0.0,
            fM21: 0.0,
            fM22: 0.0,
        })
    }
}
impl ColorSpaceXYZ {
    pub fn set_srgb(&mut self) {
        unsafe { sk_colorspace_xyz_named_srgb(&mut self.0 as _) }
    }
    pub fn set_adobe_rgb(&mut self) {
        unsafe { sk_colorspace_xyz_named_adobe_rgb(&mut self.0 as _) }
    }
    pub fn set_display_p3(&mut self) {
        unsafe { sk_colorspace_xyz_named_display_p3(&mut self.0 as _) }
    }
    pub fn set_rec2020(&mut self) {
        unsafe { sk_colorspace_xyz_named_rec2020(&mut self.0 as _) }
    }
    pub fn set_xyz(&mut self) {
        unsafe { sk_colorspace_xyz_named_xyz(&mut self.0 as _) }
    }
    pub fn invert(&self) -> Option<Self> {
        let mut dst = Self::default();
        unsafe { sk_colorspace_xyz_invert(&self.0 as _, &mut dst.0 as _).then_some(dst) }
    }
}

impl Add for ColorSpaceXYZ {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let mut result = Self::default();
        unsafe {
            sk_colorspace_xyz_concat(&self.0 as _, &rhs.0 as _, &mut result.0 as _);
        }
        result
    }
}

crate::opaque_shared!(
    ColorSpace,
    sk_colorspace_t,
    sk_colorspace_unref,
    sk_colorspace_ref
    
);
/// holds a non-null pointer to colorspace opaque struct

impl ColorSpace {
    pub fn new_srgb() -> Self {
        let inner = unsafe { sk_colorspace_new_srgb() };
        assert!(!inner.is_null(), "colospace new returned nullptr");
        Self { inner }
    }
    pub fn new_srgb_linear() -> Self {
        let inner = unsafe { sk_colorspace_new_srgb_linear() };
        assert!(!inner.is_null(), "colospace new returned nullptr");
        Self { inner }
    }
    pub fn new_rgb(transfer_fn: &TransferFn, to_xyzd50: &XYZ) -> Self {
        let inner = unsafe { sk_colorspace_new_rgb(transfer_fn as _, to_xyzd50 as _) };
        assert!(!inner.is_null(), "colospace new returned nullptr");
        Self { inner }
    }
    pub fn new_icc(profile: &ICCProfile) -> Self {
        let inner = unsafe { sk_colorspace_new_icc(profile.inner) };
        assert!(!inner.is_null(), "colospace new returned nullptr");
        Self { inner }
    }
    pub fn to_profile(&self) -> ICCProfile<'static> {
        let p = ICCProfile::new();
        unsafe {
            sk_colorspace_to_profile(self.inner, p.inner);
        }
        p
    }
    /*
    pub fn sk_colorspace_gamma_close_to_srgb(colorspace: *const sk_colorspace_t) -> bool;
    pub fn sk_colorspace_gamma_is_linear(colorspace: *const sk_colorspace_t) -> bool;
    pub fn sk_colorspace_is_numerical_transfer_fn(
        colorspace: *const sk_colorspace_t,
        transferFn: *mut sk_colorspace_transfer_fn_t,
    ) -> bool;
    pub fn sk_colorspace_to_xyzd50(
        colorspace: *const sk_colorspace_t,
        toXYZD50: *mut sk_colorspace_xyz_t,
    ) -> bool;
    pub fn sk_colorspace_make_linear_gamma(
        colorspace: *const sk_colorspace_t,
    ) -> *mut sk_colorspace_t;
    pub fn sk_colorspace_make_srgb_gamma(
        colorspace: *const sk_colorspace_t,
    ) -> *mut sk_colorspace_t;
    pub fn sk_colorspace_is_srgb(colorspace: *const sk_colorspace_t) -> bool;
    pub fn sk_colorspace_equals(src: *const sk_colorspace_t, dst: *const sk_colorspace_t) -> bool;
    pub fn sk_colorspace_transfer_fn_named_srgb(transferFn: *mut sk_colorspace_transfer_fn_t);
    pub fn sk_colorspace_transfer_fn_named_2dot2(transferFn: *mut sk_colorspace_transfer_fn_t);
    pub fn sk_colorspace_transfer_fn_named_linear(transferFn: *mut sk_colorspace_transfer_fn_t);
    pub fn sk_colorspace_transfer_fn_named_rec2020(transferFn: *mut sk_colorspace_transfer_fn_t);
    pub fn sk_colorspace_transfer_fn_named_pq(transferFn: *mut sk_colorspace_transfer_fn_t);
    pub fn sk_colorspace_transfer_fn_named_hlg(transferFn: *mut sk_colorspace_transfer_fn_t);
    pub fn sk_colorspace_transfer_fn_eval(
        transferFn: *const sk_colorspace_transfer_fn_t,
        x: f32,
    ) -> f32;
    pub fn sk_colorspace_transfer_fn_invert(
        src: *const sk_colorspace_transfer_fn_t,
        dst: *mut sk_colorspace_transfer_fn_t,
    ) -> bool;
    pub fn sk_colorspace_primaries_to_xyzd50(
        primaries: *const sk_colorspace_primaries_t,
        toXYZD50: *mut sk_colorspace_xyz_t,
    ) -> bool;
    pub fn sk_colorspace_xyz_named_srgb(xyz: *mut sk_colorspace_xyz_t);
    pub fn sk_colorspace_xyz_named_adobe_rgb(xyz: *mut sk_colorspace_xyz_t);
    pub fn sk_colorspace_xyz_named_display_p3(xyz: *mut sk_colorspace_xyz_t);
    pub fn sk_colorspace_xyz_named_rec2020(xyz: *mut sk_colorspace_xyz_t);
    pub fn sk_colorspace_xyz_named_xyz(xyz: *mut sk_colorspace_xyz_t);
    pub fn sk_colorspace_xyz_invert(
        src: *const sk_colorspace_xyz_t,
        dst: *mut sk_colorspace_xyz_t,
    ) -> bool;
    pub fn sk_colorspace_xyz_concat(
        a: *const sk_colorspace_xyz_t,
        b: *const sk_colorspace_xyz_t,
        result: *mut sk_colorspace_xyz_t,
    );
    pub fn sk_colorspace_icc_profile_delete(profile: *mut sk_colorspace_icc_profile_t);
    pub fn sk_colorspace_icc_profile_new() -> *mut sk_colorspace_icc_profile_t;
    pub fn sk_colorspace_icc_profile_parse(
        buffer: *const ::std::os::raw::c_void,
        length: usize,
        profile: *mut sk_colorspace_icc_profile_t,
    ) -> bool;
    pub fn sk_colorspace_icc_profile_get_buffer(
        profile: *const sk_colorspace_icc_profile_t,
        size: *mut u32,
    ) -> *const u8;
    pub fn sk_colorspace_icc_profile_get_to_xyzd50(
        profile: *const sk_colorspace_icc_profile_t,
        toXYZD50: *mut sk_colorspace_xyz_t,
    ) -> bool; */
}
pub struct ICCProfile<'a> {
    inner: *mut sk_colorspace_icc_profile_t,
    phantom: PhantomData<&'a [u8]>,
}
impl<'a> Drop for ICCProfile<'a> {
    fn drop(&mut self) {
        unsafe {
            sk_colorspace_icc_profile_delete(self.inner);
        }
    }
}
impl<'a> ICCProfile<'a> {
    pub fn new() -> ICCProfile<'static> {
        ICCProfile {
            inner: unsafe { sk_colorspace_icc_profile_new() },
            phantom: PhantomData,
        }
    }
    pub fn parse<'b>(self, buffer: &'b [u8]) -> Result<ICCProfile<'b>, Self> {
        if unsafe {
            sk_colorspace_icc_profile_parse(buffer.as_ptr() as _, buffer.len(), self.inner)
        } {
            Ok(ICCProfile {
                inner: self.inner,
                phantom: PhantomData,
            })
        } else {
            Err(self)
        }
    }
    pub fn get_buffer(&self) -> &[u8] {
        let mut length = 0u32;
        unsafe {
            let ptr = sk_colorspace_icc_profile_get_buffer(self.inner, &mut length as _);
            std::slice::from_raw_parts(ptr, length as _)
        }
    }
    pub fn get_to_xyzd50(&self) -> Option<ColorSpaceXYZ> {
        let mut space = ColorSpaceXYZ::default();
        unsafe {
            sk_colorspace_icc_profile_get_to_xyzd50(self.inner, &mut space.0 as _).then_some(space)
        }
    }
}
