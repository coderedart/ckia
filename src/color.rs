use std::{marker::PhantomData, ops::Add};

use ckia_sys::*;

use crate::{Color4f, ColorspaceTransferFn, ColorspaceXyz, SkiaPointer};

impl Default for Color4f {
    fn default() -> Self {
        Self {
            fR: 0.0,
            fG: 0.0,
            fB: 0.0,
            fA: 0.0,
        }
    }
}
impl From<Color> for Color4f {
    fn from(value: Color) -> Self {
        let mut c = Self::default();
        let ca = value.0.to_ne_bytes().map(|c| c as f32 / 255.0);
        // sk_color4f_from_color(color, color4f) -- there's something wrong with this function. It confuses ARGB as BGRA.
        c.fA = ca[0];
        c.fR = ca[1];
        c.fG = ca[2];
        c.fB = ca[3];
        c
    }
}
impl From<Color4f> for Color {
    fn from(value: Color4f) -> Self {
        Self::new(
            (value.fA * 255.0).clamp(0.0, 255.0) as u8,
            (value.fR * 255.0).clamp(0.0, 255.0) as u8,
            (value.fG * 255.0).clamp(0.0, 255.0) as u8,
            (value.fB * 255.0).clamp(0.0, 255.0) as u8,
        )
    }
}
/// color type represented by u32 made up of ARGB components (1 byte each) in that particular order.
/// unmultiplied
#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Color(pub sk_color_t);
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
        color |= b as u32;
        Self(color)
    }
    pub const fn from_u32(color: u32) -> Self {
        let bytes = color.to_ne_bytes();
        Self::new(bytes[0], bytes[1], bytes[2], bytes[3])
    }
    pub const fn as_u32(&self) -> u32 {
        self.0
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
    /// So, if the format is ARGB8888, then Alpha is shifted by 24 bits, R by 16, G by 8 and B by 0.
    ///
    /// This will tell you the "layout" of color components within a pixel. eg: ARGB vs ABGR
    ///
    /// **NOTE**: This has NOTHING to do with `ColorTypeByteShiftPerPixel` from skia, which is completely different.
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

impl Default for ColorspaceXyz {
    fn default() -> Self {
        Self {
            fM00: 0.0,
            fM01: 0.0,
            fM02: 0.0,
            fM10: 0.0,
            fM11: 0.0,
            fM12: 0.0,
            fM20: 0.0,
            fM21: 0.0,
            fM22: 0.0,
        }
    }
}
impl ColorspaceXyz {
    pub fn set_srgb(&mut self) {
        unsafe { sk_colorspace_xyz_named_srgb(self.as_ptr_mut()) }
    }
    pub fn set_adobe_rgb(&mut self) {
        unsafe { sk_colorspace_xyz_named_adobe_rgb(self.as_ptr_mut()) }
    }
    pub fn set_display_p3(&mut self) {
        unsafe { sk_colorspace_xyz_named_display_p3(self.as_ptr_mut()) }
    }
    pub fn set_rec2020(&mut self) {
        unsafe { sk_colorspace_xyz_named_rec2020(self.as_ptr_mut()) }
    }
    pub fn set_xyz(&mut self) {
        unsafe { sk_colorspace_xyz_named_xyz(self.as_ptr_mut()) }
    }
    pub fn invert(&self) -> Option<Self> {
        let mut dst = Self::default();
        unsafe { sk_colorspace_xyz_invert(self.as_ptr(), dst.as_ptr_mut()).then_some(dst) }
    }
}

impl Add for ColorspaceXyz {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let mut result = Self::default();
        unsafe {
            sk_colorspace_xyz_concat(self.as_ptr(), rhs.as_ptr(), result.as_ptr_mut());
        }
        result
    }
}

crate::skia_wrapper!(
    nvrefcnt,
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
    pub fn new_rgb(transfer_fn: &ColorspaceTransferFn, to_xyzd50: &ColorspaceXyz) -> Self {
        let inner = unsafe { sk_colorspace_new_rgb(transfer_fn.as_ptr(), to_xyzd50.as_ptr()) };
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
    pub fn gamma_close_to_srgb(&self) -> bool {
        unsafe { sk_colorspace_gamma_close_to_srgb(self.as_ptr()) }
    }
    pub fn gamme_is_linear(&self) -> bool {
        unsafe { sk_colorspace_gamma_is_linear(self.as_ptr()) }
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
    */
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
    /// We take self by value because our lifetime (and by extension our type) depends on success vs failure result
    /// If we succeed, then our lifetime is attached to the new `buffer`.
    /// But if we fail, our lifetime should still remain the same as before this function (because failure case won't change anything within our struct)
    pub fn parse(self, buffer: &[u8]) -> Result<ICCProfile, Self> {
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
    pub fn get_to_xyzd50(&self) -> Option<ColorspaceXyz> {
        let mut space = ColorspaceXyz::default();
        unsafe {
            sk_colorspace_icc_profile_get_to_xyzd50(self.inner, space.as_ptr_mut()).then_some(space)
        }
    }
}
