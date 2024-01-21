use std::ffi::CStr;
use std::ffi::CString;

use crate::data::SkiaData;
use crate::string::SkiaString;
use crate::FontStyleSlant;
use crate::SkiaPointer;
use ckia_sys::*;

crate::skia_wrapper!(refcnt, Typeface, sk_typeface_t, sk_typeface_unref);

impl Typeface {
    /*
    pub fn sk_typeface_get_fontstyle(typeface: *const sk_typeface_t) -> *mut sk_fontstyle_t;
    pub fn sk_typeface_get_font_weight(typeface: *const sk_typeface_t) -> ::std::os::raw::c_int;
    pub fn sk_typeface_get_font_width(typeface: *const sk_typeface_t) -> ::std::os::raw::c_int;
    pub fn sk_typeface_get_font_slant(typeface: *const sk_typeface_t) -> sk_font_style_slant_t;
    pub fn sk_typeface_is_fixed_pitch(typeface: *const sk_typeface_t) -> bool;
    pub fn sk_typeface_unichars_to_glyphs(
        typeface: *const sk_typeface_t,
        unichars: *const i32,
        count: ::std::os::raw::c_int,
        glyphs: *mut u16,
    );
    pub fn sk_typeface_unichar_to_glyph(typeface: *const sk_typeface_t, unichar: i32) -> u16;
    pub fn sk_typeface_count_glyphs(typeface: *const sk_typeface_t) -> ::std::os::raw::c_int;
    pub fn sk_typeface_count_tables(typeface: *const sk_typeface_t) -> ::std::os::raw::c_int;
    pub fn sk_typeface_get_table_tags(
        typeface: *const sk_typeface_t,
        tags: *mut sk_font_table_tag_t,
    ) -> ::std::os::raw::c_int;
    pub fn sk_typeface_get_table_size(
        typeface: *const sk_typeface_t,
        tag: sk_font_table_tag_t,
    ) -> usize;
    pub fn sk_typeface_get_table_data(
        typeface: *const sk_typeface_t,
        tag: sk_font_table_tag_t,
        offset: usize,
        length: usize,
        data: *mut ::std::os::raw::c_void,
    ) -> usize;
    pub fn sk_typeface_copy_table_data(
        typeface: *const sk_typeface_t,
        tag: sk_font_table_tag_t,
    ) -> *mut sk_data_t;
    pub fn sk_typeface_get_units_per_em(typeface: *const sk_typeface_t) -> ::std::os::raw::c_int;
    pub fn sk_typeface_get_kerning_pair_adjustments(
        typeface: *const sk_typeface_t,
        glyphs: *const u16,
        count: ::std::os::raw::c_int,
        adjustments: *mut i32,
    ) -> bool;
    pub fn sk_typeface_get_family_name(typeface: *const sk_typeface_t) -> *mut sk_string_t;
    pub fn sk_typeface_open_stream(
        typeface: *const sk_typeface_t,
        ttcIndex: *mut ::std::os::raw::c_int,
    ) -> *mut sk_stream_asset_t;

     */
}

crate::skia_wrapper!(refcnt, FontMgr, sk_fontmgr_t, sk_fontmgr_unref);
impl FontMgr {
    pub fn create_custom_dir(path: impl AsRef<std::path::Path>) -> Option<Self> {
        let cstr = CString::new(path.as_ref().as_os_str().as_encoded_bytes()).ok()?;
        unsafe { Self::try_from_owned_ptr(sk_fontmgr_create_custom_dir(cstr.as_ptr())) }
    }
    pub fn create_custom_data(datas: &mut [SkiaData]) -> Option<Self> {
        unsafe {
            Self::try_from_owned_ptr(sk_fontmgr_create_custom_data(
                std::mem::transmute(datas.as_mut_ptr()),
                datas.len(),
            ))
        }
    }
    /// returns a static singleton fontmgr which contains no fonts.
    pub fn create_empty() -> Self {
        unsafe { Self::from_owned_ptr(sk_fontmgr_ref_empty()) }
    }
    pub fn count_families(&mut self) -> i32 {
        unsafe { sk_fontmgr_count_families(self.as_ptr_mut()) }
    }
    /// probably returns empty name if there's no family at index
    pub fn get_family_name(&mut self, index: i32) -> SkiaString {
        let mut skstr = SkiaString::new_empty();
        unsafe { sk_fontmgr_get_family_name(self.as_ptr_mut(), index, skstr.as_ptr_mut()) }
        skstr
    }
    pub fn create_styleset(&mut self, index: i32) -> Option<FontStyleSet> {
        unsafe {
            FontStyleSet::try_from_owned_ptr(sk_fontmgr_create_styleset(self.as_ptr_mut(), index))
        }
    }
    pub fn match_family(&mut self, family_name: &CStr) -> Option<FontStyleSet> {
        unsafe {
            FontStyleSet::try_from_owned_ptr(sk_fontmgr_match_family(
                self.as_ptr_mut(),
                family_name.as_ptr(),
            ))
        }
    }
    pub fn match_family_style(
        &mut self,
        family_name: &CStr,
        style: &mut FontStyle,
    ) -> Option<Typeface> {
        unsafe {
            Typeface::try_from_owned_ptr(sk_fontmgr_match_family_style(
                self.as_ptr_mut(),
                family_name.as_ptr(),
                style.as_ptr_mut(),
            ))
        }
    }

    /*
    pub fn sk_fontmgr_match_family_style_character(
        arg1: *mut sk_fontmgr_t,
        familyName: *const ::std::os::raw::c_char,
        style: *mut sk_fontstyle_t,
        bcp47: *mut *const ::std::os::raw::c_char,
        bcp47Count: ::std::os::raw::c_int,
        character: i32,
    ) -> *mut sk_typeface_t;
    */
    pub fn create_from_data(&mut self, data: &mut SkiaData, index: i32) -> Option<Typeface> {
        unsafe {
            Typeface::try_from_owned_ptr(sk_fontmgr_create_from_data(
                self.as_ptr_mut(),
                data.as_ptr_mut(),
                index,
            ))
        }
    }
    /*
          pub fn sk_fontmgr_create_from_stream(
              arg1: *mut sk_fontmgr_t,
              stream: *mut sk_stream_asset_t,
              index: ::std::os::raw::c_int,
          ) -> *mut sk_typeface_t;
    */
    pub fn create_from_file(&mut self, path: &std::path::Path, index: i32) -> Option<Typeface> {
        let cstr = CString::new(path.as_os_str().as_encoded_bytes()).ok()?;
        unsafe {
            Typeface::try_from_owned_ptr(sk_fontmgr_create_from_file(
                self.as_ptr_mut(),
                cstr.as_ptr(),
                index,
            ))
        }
    }
    pub fn create_from_name(&mut self, family_name: &CStr, style: &FontStyle) -> Option<Typeface> {
        unsafe {
            Typeface::try_from_owned_ptr(sk_fontmgr_typeface_create_from_name(
                self.as_ptr_mut(),
                family_name.as_ptr(),
                style.as_ptr(),
            ))
        }
    }
}

crate::skia_wrapper!(unique, FontStyle, sk_fontstyle_t, sk_fontstyle_delete);

impl FontStyle {
    pub fn new(weight: i32, width: i32, slant: FontStyleSlant) -> Self {
        unsafe { Self::from_owned_ptr(sk_fontstyle_new(weight, width, slant)) }
    }
    pub fn get_weight(&self) -> i32 {
        unsafe { sk_fontstyle_get_weight(self.as_ptr()) }
    }
    pub fn get_width(&self) -> i32 {
        unsafe { sk_fontstyle_get_width(self.as_ptr()) }
    }
    pub fn get_slant(&self) -> FontStyleSlant {
        unsafe { sk_fontstyle_get_slant(self.as_ptr()) }
    }
}

crate::skia_wrapper!(
    refcnt,
    FontStyleSet,
    sk_fontstyleset_t,
    sk_fontstyleset_unref
);

impl FontStyleSet {
    pub fn new_empty() -> Option<Self> {
        unsafe { Self::try_from_owned_ptr(sk_fontstyleset_create_empty()) }
    }
    pub fn get_count(&mut self) -> i32 {
        unsafe { sk_fontstyleset_get_count(self.as_ptr_mut()) }
    }
    /// `index` refers to the index of the font style
    /// If `fs` is provided, then we will set the fontstyle at that index into this mut ref.
    /// If `name` is provided, then we will also set the name. you can provile None as fs, if you just want the name.
    /// You can also provide None as name, if you don't care about name (especially for custom fontmgrs).
    /// If the font style doesn't have name, and you provided a mut ref to SkiaString, then it will simply be reset (made empty).
    /// If the index refers to an invalid font, then this does nothing.
    pub fn get_style(
        &mut self,
        index: i32,
        fs: Option<&mut FontStyle>,
        name: Option<&mut SkiaString>,
    ) {
        unsafe {
            sk_fontstyleset_get_style(
                self.as_ptr_mut(),
                index,
                fs.map(|fs| fs.as_ptr_mut()).unwrap_or(std::ptr::null_mut()),
                name.map(|n| n.as_ptr_mut()).unwrap_or(std::ptr::null_mut()),
            )
        }
    }
    pub fn create_typeface(&mut self, index: i32) -> Option<Typeface> {
        unsafe {
            Typeface::try_from_owned_ptr(sk_fontstyleset_create_typeface(self.as_ptr_mut(), index))
        }
    }
    pub fn match_style(&mut self, style: &mut FontStyle) -> Option<Typeface> {
        unsafe {
            Typeface::try_from_owned_ptr(sk_fontstyleset_match_style(
                self.as_ptr_mut(),
                style.as_ptr_mut(),
            ))
        }
    }
}
