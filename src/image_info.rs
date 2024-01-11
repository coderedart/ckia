use ckia_sys::sk_imageinfo_t;

use crate::{
    alpha_type_default, bytes_per_pixel, color::ColorSpace, color_type_default, AlphaType,
    ColorType,
};

#[repr(transparent)]
pub struct ImageInfo(pub(crate) sk_imageinfo_t);
impl Default for ImageInfo {
    fn default() -> Self {
        ImageInfo(sk_imageinfo_t {
            colorspace: std::ptr::null_mut(),
            width: 0,
            height: 0,
            colorType: color_type_default(),
            alphaType: alpha_type_default(),
        })
    }
}
impl Drop for ImageInfo {
    fn drop(&mut self) {
        unsafe {
            ColorSpace::from_owned_ptr(self.0.colorspace);
        }
    }
}
impl ImageInfo {
    pub fn get_width(&self) -> i32 {
        self.0.width
    }
    pub fn get_height(&self) -> i32 {
        self.0.height
    }
    pub fn set_width(&mut self, width: i32) {
        self.0.width = width;
    }
    pub fn set_height(&mut self, height: i32) {
        self.0.height = height;
    }
    pub fn get_color_type(&self) -> ColorType {
        self.0.colorType
    }
    pub fn set_color_type(&mut self, color_type: ColorType) {
        self.0.colorType = color_type;
    }
    pub fn get_alpha_type(&mut self) -> AlphaType {
        self.0.alphaType
    }
    pub fn set_alpha_type(&mut self, alpha_type: AlphaType) {
        self.0.alphaType = alpha_type;
    }
    pub fn get_color_space(&self) -> Option<ColorSpace> {
        unsafe { ColorSpace::from_borrowed_ptr(self.0.colorspace) }
    }
    pub fn set_color_space(&mut self, cs: ColorSpace) {
        unsafe {
            std::mem::drop(ColorSpace::from_owned_ptr(self.0.colorspace));
            self.0.colorspace = cs.into_owned_ptr();
        }
    }
    pub fn bytes_per_pixel(&self) -> u8 {
        bytes_per_pixel(self.0.colorType)
    }
}
