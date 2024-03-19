use std::ffi::CString;

use crate::types::*;
use crate::{bindings::*, skia_wrapper, Color};
use crate::{
    canvas::Canvas,
    paint::Paint,
    path::SkiaPath,
    string::SkiaString,
    text_blob::TextBlob,
    typeface::{FontMgr, FontStyle, Typeface},
};

skia_wrapper!(
    unique,
    ParagraphFontArguments,
    tl_font_arguments_t,
    tl_font_arguments_delete
);

impl ParagraphFontArguments {
    // lets bother when we actually need this
    // pub fn tl_font_arguments_from_sk_fontarguments(
    //     sk_fontarg: SkFontAr,
    // ) -> *mut tl_font_arguments_t;
}

skia_wrapper!(unique, TextStyle, tl_text_style_t, tl_text_style_delete);

impl Default for TextStyle {
    fn default() -> Self {
        unsafe { Self::from_owned_ptr(tl_text_style_new()) }
    }
}
impl TextStyle {
    pub fn get_color(&self) -> Color {
        Color(unsafe { tl_text_style_get_color(self.as_ptr()) })
    }
    pub fn set_color(&mut self, color: Color) {
        unsafe { tl_text_style_set_color(self.as_ptr_mut(), color.0) }
    }
    pub fn has_foreground(&self) -> bool {
        unsafe { tl_text_style_has_foreground(self.as_ptr()) }
    }
    pub fn get_foreground(&self) -> Paint {
        unsafe { Paint::from_owned_ptr(tl_text_style_get_foreground(self.as_ptr())) }
    }
    pub fn set_foreground(&mut self, paint: &Paint) {
        unsafe { tl_text_style_set_foreground_paint(self.as_ptr_mut(), paint.as_ptr()) };
    }
    pub fn clear_foreground_color(&mut self) {
        unsafe {
            tl_text_style_clear_foreground_color(self.as_ptr_mut());
        }
    }
    pub fn has_background(&self) -> bool {
        unsafe { tl_text_style_has_background(self.as_ptr()) }
    }
    pub fn get_background(&self) -> Paint {
        unsafe { Paint::from_owned_ptr(tl_text_style_get_background(self.as_ptr())) }
    }
    pub fn set_background(&mut self, paint: &Paint) {
        unsafe { tl_text_style_set_background_paint(self.as_ptr_mut(), paint.as_ptr()) };
    }
    pub fn clear_background_color(&mut self) {
        unsafe {
            tl_text_style_clear_background_color(self.as_ptr_mut());
        }
    }
    pub fn get_decoration(&self) -> Decoration {
        unsafe {
            let mut decoration = std::mem::zeroed::<Decoration>();
            tl_text_style_get_decoration(self.as_ptr(), decoration.as_ptr_mut());
            decoration
        }
    }
    pub fn get_decoration_type(&self) -> TextDecoration {
        unsafe { tl_text_style_get_decoration_type(self.as_ptr()) }
    }
    pub fn get_decoration_mode(&self) -> TextDecorationMode {
        unsafe { tl_text_style_get_decoration_mode(self.as_ptr()) }
    }
    pub fn get_decoration_color(&self) -> Color {
        unsafe { Color(tl_text_style_get_decoration_color(self.as_ptr())) }
    }
    pub fn get_decoration_style(&self) -> TextDecorationStyle {
        unsafe { tl_text_style_get_decoration_style(self.as_ptr()) }
    }
    pub fn get_decoration_thickness_multiplier(&self) -> f32 {
        unsafe { tl_text_style_get_decoration_thickness_multiplier(self.as_ptr()) }
    }
    pub fn get_font_style(&self) -> FontStyle {
        unsafe { FontStyle::from_owned_ptr(tl_text_style_get_font_style(self.as_ptr())) }
    }
    pub fn get_shadow_number(&self) -> usize {
        unsafe { tl_text_style_get_shadow_number(self.as_ptr()) }
    }
    pub fn get_font_feature_number(&self) -> usize {
        unsafe { tl_text_style_get_font_feature_number(self.as_ptr()) }
    }
    pub fn get_font_size(&self) -> f32 {
        unsafe { tl_text_style_get_font_size(self.as_ptr()) }
    }
    pub fn get_baseline_shift(&self) -> f32 {
        unsafe { tl_text_style_get_baseline_shift(self.as_ptr()) }
    }
    pub fn get_height(&self) -> f32 {
        unsafe { tl_text_style_get_height(self.as_ptr()) }
    }
    pub fn get_height_override(&self) -> bool {
        unsafe { tl_text_style_get_height_override(self.as_ptr()) }
    }
    pub fn get_half_leading(&self) -> bool {
        unsafe { tl_text_style_get_half_leading(self.as_ptr()) }
    }
    pub fn get_letter_spacing(&self) -> f32 {
        unsafe { tl_text_style_get_letter_spacing(self.as_ptr()) }
    }
    pub fn get_word_spacing(&self) -> f32 {
        unsafe { tl_text_style_get_word_spacing(self.as_ptr()) }
    }

    pub fn get_typeface(&self) -> Typeface {
        unsafe { Typeface::from_owned_ptr(tl_text_style_get_typeface(self.as_ptr())) }
    }
    pub fn get_locale(&self) -> SkiaString {
        unsafe { SkiaString::from_owned_ptr(tl_text_style_get_locale(self.as_ptr())) }
    }
    pub fn get_text_baseline(&self) -> TextBaseline {
        unsafe { tl_text_style_get_text_baseline(self.as_ptr()) }
    }
    pub fn set_decoration_type(&mut self, value: TextDecoration) {
        unsafe { tl_text_style_set_decoration_type(self.as_ptr_mut(), value) }
    }
    pub fn set_decoration_mode(&mut self, value: TextDecorationMode) {
        unsafe { tl_text_style_set_decoration_mode(self.as_ptr_mut(), value) }
    }
    pub fn set_decoration_color(&mut self, value: Color) {
        unsafe { tl_text_style_set_decoration_color(self.as_ptr_mut(), value.0) }
    }
    pub fn set_decoration_style(&mut self, value: TextDecorationStyle) {
        unsafe { tl_text_style_set_decoration_style(self.as_ptr_mut(), value) }
    }

    pub fn set_decoration_thickness_multiplier(&mut self, value: f32) {
        unsafe {
            tl_text_style_set_decoration_thickness_multiplier(self.as_ptr_mut(), value);
        }
    }
    pub fn set_font_style(&mut self, value: &FontStyle) {
        unsafe {
            tl_text_style_set_font_style(self.as_ptr_mut(), value.as_ptr());
        }
    }

    pub fn set_font_size(&mut self, value: f32) {
        unsafe {
            tl_text_style_set_font_size(self.as_ptr_mut(), value);
        }
    }
    pub fn set_baseline_shift(&mut self, value: f32) {
        unsafe {
            tl_text_style_set_baseline_shift(self.as_ptr_mut(), value);
        }
    }
    pub fn set_height(&mut self, value: f32) {
        unsafe {
            tl_text_style_set_height(self.as_ptr_mut(), value);
        }
    }
    pub fn set_height_override(&mut self, value: bool) {
        unsafe {
            tl_text_style_set_height_override(self.as_ptr_mut(), value);
        }
    }
    pub fn set_half_leading(&mut self, value: bool) {
        unsafe {
            tl_text_style_set_half_leading(self.as_ptr_mut(), value);
        }
    }
    pub fn set_letter_spacing(&mut self, value: f32) {
        unsafe {
            tl_text_style_set_letter_spacing(self.as_ptr_mut(), value);
        }
    }
    pub fn set_word_spacing(&mut self, value: f32) {
        unsafe {
            tl_text_style_set_word_spacing(self.as_ptr_mut(), value);
        }
    }

    pub fn set_typeface(&mut self, value: &Typeface) {
        unsafe { tl_text_style_set_typeface(self.as_ptr_mut(), value.as_ptr()) }
    }
    pub fn set_locale(&mut self, value: &SkiaString) {
        unsafe { tl_text_style_set_locale(self.as_ptr_mut(), value.as_ptr()) }
    }
    pub fn tl_text_style_set_text_baseline(&mut self, value: TextBaseline) {
        unsafe { tl_text_style_set_text_baseline(self.as_ptr_mut(), value) }
    }
    pub fn reset_shadows(&mut self) {
        unsafe { tl_text_style_reset_shadows(self.as_ptr_mut()) }
    }
    pub fn reset_font_features(&mut self) {
        unsafe { tl_text_style_reset_font_features(self.as_ptr_mut()) }
    }
}

skia_wrapper!(unique, StrutStyle, tl_strut_style_t, tl_strut_style_delete);
impl Default for StrutStyle {
    fn default() -> Self {
        unsafe { Self::from_owned_ptr(tl_strut_style_new()) }
    }
}
impl StrutStyle {
    pub fn get_font_style(&self) -> FontStyle {
        unsafe { FontStyle::from_owned_ptr(tl_strut_get_font_style(self.as_ptr())) }
    }
    pub fn set_font_style(&mut self, font_style: &FontStyle) {
        unsafe { tl_strut_set_font_style(self.as_ptr_mut(), font_style.as_ptr()) };
    }
    pub fn get_font_size(&self) -> f32 {
        unsafe { tl_strut_get_font_size(self.as_ptr()) }
    }
    pub fn set_font_size(&mut self, font_size: f32) {
        unsafe { tl_strut_set_font_size(self.as_ptr_mut(), font_size) }
    }
    pub fn get_height(&self) -> f32 {
        unsafe { tl_strut_get_height(self.as_ptr()) }
    }
    pub fn set_height(&mut self, height: f32) {
        unsafe { tl_strut_set_height(self.as_ptr_mut(), height) }
    }
    pub fn get_leading(&self) -> f32 {
        unsafe { tl_strut_get_leading(self.as_ptr()) }
    }
    pub fn set_leading(&mut self, leading: f32) {
        unsafe { tl_strut_set_leading(self.as_ptr_mut(), leading) }
    }
    pub fn get_strut_enabled(&self) -> bool {
        unsafe { tl_strut_get_strut_enabled(self.as_ptr()) }
    }
    pub fn set_strut_enabled(&mut self, value: bool) {
        unsafe { tl_strut_set_strut_enabled(self.as_ptr_mut(), value) }
    }
    pub fn get_force_strut_height(&self) -> bool {
        unsafe { tl_strut_get_force_strut_height(self.as_ptr()) }
    }
    pub fn set_force_strut_height(&mut self, value: bool) {
        unsafe { tl_strut_set_force_strut_height(self.as_ptr_mut(), value) }
    }
    pub fn get_height_override(&self) -> bool {
        unsafe { tl_strut_get_height_override(self.as_ptr()) }
    }
    pub fn set_height_override(&mut self, value: bool) {
        unsafe { tl_strut_set_height_override(self.as_ptr_mut(), value) }
    }
}

skia_wrapper!(
    unique,
    ParagraphStyle,
    tl_paragraph_style_t,
    tl_paragraph_style_delete
);
impl Default for ParagraphStyle {
    fn default() -> Self {
        unsafe { Self::from_owned_ptr(tl_paragraph_style_new()) }
    }
}
impl ParagraphStyle {
    pub fn get_strut_style(&self) -> StrutStyle {
        unsafe { StrutStyle::from_owned_ptr(tl_paragraph_style_get_strut_style(self.as_ptr())) }
    }
    pub fn set_strut_style(&mut self, strut_style: &StrutStyle) {
        unsafe { tl_paragraph_style_set_strut_style(self.as_ptr_mut(), strut_style.as_ptr()) };
    }
    pub fn get_text_style(&self) -> TextStyle {
        unsafe { TextStyle::from_owned_ptr(tl_paragraph_style_get_text_style(self.as_ptr())) }
    }
    pub fn set_text_style(&mut self, text_style: &TextStyle) {
        unsafe { tl_paragraph_style_set_text_style(self.as_ptr_mut(), text_style.as_ptr()) };
    }
    pub fn get_text_direction(&self) -> TextDirection {
        unsafe { tl_paragraph_style_get_text_direction(self.as_ptr()) }
    }
    pub fn set_text_direction(&mut self, value: TextDirection) {
        unsafe { tl_paragraph_style_set_text_direction(self.as_ptr_mut(), value) };
    }
    pub fn get_text_align(&self) -> ParagraphTextAlign {
        unsafe { tl_paragraph_style_get_text_align(self.as_ptr()) }
    }
    pub fn set_text_align(&mut self, value: ParagraphTextAlign) {
        unsafe { tl_paragraph_style_set_text_align(self.as_ptr_mut(), value) };
    }
    pub fn get_max_lines(&self) -> usize {
        unsafe { tl_paragraph_style_get_max_lines(self.as_ptr()) }
    }
    pub fn set_max_lines(&mut self, value: usize) {
        unsafe { tl_paragraph_style_set_max_lines(self.as_ptr_mut(), value) };
    }
    pub fn get_ellipsis(&self) -> SkiaString {
        unsafe { SkiaString::from_owned_ptr(tl_paragraph_style_get_ellipsis(self.as_ptr())) }
    }
    pub fn set_ellipsis(&mut self, value: &SkiaString) {
        unsafe { tl_paragraph_style_set_ellipsis(self.as_ptr_mut(), value.as_ptr()) };
    }
    pub fn get_height(&self) -> f32 {
        unsafe { tl_paragraph_style_get_height(self.as_ptr()) }
    }
    pub fn set_height(&mut self, value: f32) {
        unsafe { tl_paragraph_style_set_height(self.as_ptr_mut(), value) };
    }
    pub fn get_text_height_behavior(&self) -> TextHeightBehavior {
        unsafe { tl_paragraph_style_get_text_height_behavior(self.as_ptr()) }
    }
    pub fn set_text_height_behavior(&mut self, value: TextHeightBehavior) {
        unsafe { tl_paragraph_style_set_text_height_behavior(self.as_ptr_mut(), value) };
    }
    pub fn get_replace_tab_characters(&self) -> bool {
        unsafe { tl_paragraph_style_get_replace_tab_characters(self.as_ptr()) }
    }
    pub fn set_replace_tab_characters(&mut self, value: bool) {
        unsafe { tl_paragraph_style_set_replace_tab_characters(self.as_ptr_mut(), value) };
    }
    pub fn get_apply_rounding_hack(&self) -> bool {
        unsafe { tl_paragraph_style_get_apply_rounding_hack(self.as_ptr()) }
    }
    pub fn set_apply_rounding_hack(&mut self, value: bool) {
        unsafe { tl_paragraph_style_set_apply_rounding_hack(self.as_ptr_mut(), value) };
    }
    pub fn hinting_is_on(&self) -> bool {
        unsafe { tl_paragraph_style_hinting_is_on(self.as_ptr()) }
    }
    pub fn turn_hinting_off(&mut self) {
        unsafe { tl_paragraph_style_turn_hinting_off(self.as_ptr_mut()) }
    }
    pub fn unlimited_lines(&self) -> bool {
        unsafe { tl_paragraph_style_unlimited_lines(self.as_ptr()) }
    }
    pub fn ellipsized(&self) -> bool {
        unsafe { tl_paragraph_style_ellipsized(self.as_ptr()) }
    }
    pub fn effective_align(&self) -> ParagraphTextAlign {
        unsafe { tl_paragraph_style_effective_align(self.as_ptr()) }
    }
}
skia_wrapper!(
    shared,
    FontCollection,
    tl_font_collection_t,
    tl_font_collection_unref,
    tl_font_collection_ref
);
impl Default for FontCollection {
    fn default() -> Self {
        unsafe { Self::from_owned_ptr(tl_font_collection_new()) }
    }
}

impl FontCollection {
    pub fn set_asset_font_manager(&mut self, fontmgr: &FontMgr) {
        unsafe { tl_font_collection_set_asset_font_manager(self.as_ptr_mut(), fontmgr.as_ptr()) };
    }
    pub fn set_dynamic_font_manager(&mut self, fontmgr: &FontMgr) {
        unsafe { tl_font_collection_set_dynamic_font_manager(self.as_ptr_mut(), fontmgr.as_ptr()) };
    }
    pub fn set_test_font_manager(&mut self, fontmgr: &FontMgr) {
        unsafe { tl_font_collection_set_test_font_manager(self.as_ptr_mut(), fontmgr.as_ptr()) };
    }
    pub fn set_default_font_manager(&mut self, fontmgr: &FontMgr) {
        unsafe { tl_font_collection_set_default_font_manager(self.as_ptr_mut(), fontmgr.as_ptr()) };
    }
    pub fn set_default_font_manager_with_family_name(
        &mut self,
        fontmgr: &FontMgr,
        family_name: &str,
    ) {
        let cfamily_name = CString::new(family_name).unwrap();
        unsafe {
            tl_font_collection_set_default_font_manager_with_family_name(
                self.as_ptr_mut(),
                fontmgr.as_ptr(),
                cfamily_name.as_ptr(),
            )
        };
    }
    pub fn get_fallback_font_manager(&self) -> FontMgr {
        unsafe {
            FontMgr::from_owned_ptr(tl_font_collection_get_fallback_font_manager(self.as_ptr()))
        }
    }
    pub fn disable_font_fallback(&mut self) {
        unsafe { tl_font_collection_disable_font_fallback(self.as_ptr_mut()) }
    }
    pub fn enable_font_fallback(&mut self) {
        unsafe { tl_font_collection_enable_font_fallback(self.as_ptr_mut()) }
    }
    pub fn font_fallback_enabled(&mut self) -> bool {
        unsafe { tl_font_collection_font_fallback_enabled(self.as_ptr_mut()) }
    }
}
skia_wrapper!(
    unique,
    ParagraphBuider,
    tl_paragraph_builder_t,
    tl_paragraph_builder_delete
);
impl ParagraphBuider {
    pub fn new(style: &ParagraphStyle, font_collection: &FontCollection) -> Self {
        unsafe {
            Self::from_owned_ptr(tl_paragraph_builder_new(
                style.as_ptr(),
                font_collection.as_ptr(),
            ))
        }
    }
    pub fn push_style(&mut self, text_style: &TextStyle) {
        unsafe { tl_paragraph_builder_push_style(self.as_ptr_mut(), text_style.as_ptr()) }
    }
    pub fn pop(&mut self) {
        unsafe { tl_paragraph_builder_pop(self.as_ptr_mut()) }
    }
    pub fn peek_style(&mut self) -> TextStyle {
        unsafe { TextStyle::from_owned_ptr(tl_paragraph_builder_peek_style(self.as_ptr_mut())) }
    }
    pub fn add_text(&mut self, text: &str) {
        unsafe { tl_paragraph_builder_add_text(self.as_ptr_mut(), text.as_ptr() as _, text.len()) }
    }
    pub fn add_place_holder(&mut self, place_holder_style: &PlaceholderStyle) {
        unsafe {
            tl_paragraph_builder_add_place_holder(self.as_ptr_mut(), place_holder_style.as_ptr())
        }
    }
    pub fn build(&mut self) -> Paragraph {
        unsafe { Paragraph::from_owned_ptr(tl_paragraph_builder_build(self.as_ptr_mut())) }
    }
    /// please don't call any other fn while holding this str return value.
    pub fn get_text(&mut self) -> &str {
        let mut len = 0usize;
        unsafe {
            let ptr = tl_paragraph_builder_get_text(self.as_ptr_mut(), &mut len as *mut usize);
            let slice = std::slice::from_raw_parts(ptr as *const u8, len);
            return std::str::from_utf8(slice)
                .expect("failed to get text from paragraph builder because its invalid utf-8");
        }
    }
    pub fn reset(&mut self) {
        unsafe { tl_paragraph_builder_reset(self.as_ptr_mut()) }
    }
}
skia_wrapper!(unique, Paragraph, tl_paragraph_t, tl_paragraph_delete);
impl Paragraph {
    pub fn get_max_width(&mut self) -> f32 {
        unsafe { tl_paragraph_get_max_width(self.as_ptr_mut()) }
    }
    pub fn get_height(&mut self) -> f32 {
        unsafe { tl_paragraph_get_height(self.as_ptr_mut()) }
    }
    pub fn get_min_intrinsic_width(&mut self) -> f32 {
        unsafe { tl_paragraph_get_min_intrinsic_width(self.as_ptr_mut()) }
    }
    pub fn get_max_intrinsic_width(&mut self) -> f32 {
        unsafe { tl_paragraph_get_max_intrinsic_width(self.as_ptr_mut()) }
    }
    pub fn get_alphabetic_baseline(&mut self) -> f32 {
        unsafe { tl_paragraph_get_alphabetic_baseline(self.as_ptr_mut()) }
    }
    pub fn get_ideographic_baseline(&mut self) -> f32 {
        unsafe { tl_paragraph_get_ideographic_baseline(self.as_ptr_mut()) }
    }
    pub fn get_longest_line(&mut self) -> f32 {
        unsafe { tl_paragraph_get_longest_line(self.as_ptr_mut()) }
    }
    pub fn get_did_exceed_max_lines(&mut self) -> bool {
        unsafe { tl_paragraph_get_did_exceed_max_lines(self.as_ptr_mut()) }
    }
    pub fn layout(&mut self, width: f32) {
        unsafe { tl_paragraph_layout(self.as_ptr_mut(), width) }
    }
    pub fn paint(&mut self, canvas: &mut Canvas, x: f32, y: f32) {
        unsafe { tl_paragraph_paint(self.as_ptr_mut(), canvas.as_ptr_mut(), x, y) }
    }
    ///
    /// # Safety
    /// If you pass in None, then we return the size of vec needed.
    /// If you pass in Some, then we will fill as much as we can and still return the vec size needed.
    /// So, it is upto the user to actually ensure that the returned usize is less than or equal to vec size.
    /// So that they are not missing any textboxes
    pub unsafe fn get_rects_for_range(
        &mut self,
        start: u32,
        end: u32,
        hstyle: RectHeightStyle,
        wstyle: RectWidthStyle,
        vec: Option<&mut Vec<TextBox>>,
    ) -> usize {
        let len = vec.as_ref().map(|v| v.len()).unwrap_or_default();

        unsafe {
            tl_paragraph_get_rects_for_range(
                self.as_ptr_mut(),
                start,
                end,
                hstyle,
                wstyle,
                vec.map(|v| v as *mut Vec<TextBox> as *mut tl_text_box_t)
                    .unwrap_or(std::ptr::null_mut()),
                len,
            )
        }
    }
    ///
    /// # Safety
    /// If you pass in None, then we return the size of vec needed.
    /// If you pass in Some, then we will fill as much as we can and still return the vec size needed.
    /// So, it is upto the user to actually ensure that the returned usize is less than or equal to vec size.
    /// So that they are not missing any textboxes
    pub unsafe fn get_rects_for_placeholders(&mut self, vec: Option<&mut Vec<TextBox>>) -> usize {
        let len = vec.as_ref().map(|v| v.len()).unwrap_or_default();

        unsafe {
            tl_paragraph_get_rects_for_placeholders(
                self.as_ptr_mut(),
                vec.map(|v| v as *mut Vec<TextBox> as *mut tl_text_box_t)
                    .unwrap_or(std::ptr::null_mut()),
                len,
            )
        }
    }
    pub fn get_glyph_position_at_coordinate(&mut self, dx: f32, dy: f32) -> PositionWithAffinity {
        let mut pa = PositionWithAffinity {
            position: 0,
            affinity: Affinity::DOWNSTREAM_AFFINITY,
        };
        unsafe {
            tl_paragraph_get_glyph_position_at_coordinate(
                self.as_ptr_mut(),
                dx,
                dy,
                pa.as_ptr_mut(),
            );
        }
        pa
    }
    pub fn get_word_boundary(&mut self, offset: u32) -> (usize, usize) {
        let mut start = 0usize;
        let mut end = 0usize;
        unsafe {
            tl_paragraph_get_word_boundary(
                self.as_ptr_mut(),
                offset,
                &mut start as *mut usize,
                &mut end as *mut usize,
            )
        };
        (start, end)
    }
    pub fn line_number(&mut self) -> usize {
        unsafe { tl_paragraph_line_number(self.as_ptr_mut()) }
    }
    pub fn mark_dirty(&mut self) {
        unsafe { tl_paragraph_mark_dirty(self.as_ptr_mut()) }
    }
    pub fn unresolved_glyphs(&mut self) -> i32 {
        unsafe { tl_paragraph_unresolved_glyphs(self.as_ptr_mut()) }
    }
    pub fn update_text_align(&mut self, align: ParagraphTextAlign) {
        unsafe { tl_paragraph_update_text_align(self.as_ptr_mut(), align) }
    }
    pub fn update_font_size(&mut self, from: usize, to: usize, font_size: f32) {
        unsafe { tl_paragraph_update_font_size(self.as_ptr_mut(), from, to, font_size) }
    }
    pub fn update_foreground_paint(&mut self, from: usize, to: usize, paint: &Paint) {
        unsafe { tl_paragraph_update_foreground_paint(self.as_ptr_mut(), from, to, paint.as_ptr()) }
    }
    pub fn update_background_paint(&mut self, from: usize, to: usize, paint: &Paint) {
        unsafe { tl_paragraph_update_background_paint(self.as_ptr_mut(), from, to, paint.as_ptr()) }
    }
    pub fn get_path(&mut self, line_number: i32, dest: &mut SkiaPath) -> i32 {
        unsafe { tl_paragraph_get_path(self.as_ptr_mut(), line_number, dest.as_ptr_mut()) }
    }
    pub fn get_path_text_blob(blob: &mut TextBlob) -> SkiaPath {
        unsafe { SkiaPath::from_owned_ptr(tl_paragraph_get_path_text_blob(blob.as_ptr_mut())) }
    }
    pub fn contains_emoji(&mut self, blob: &mut TextBlob) -> bool {
        unsafe { tl_paragraph_contains_emoji(self.as_ptr_mut(), blob.as_ptr_mut()) }
    }
    pub fn contains_color_font_or_bitmap(&mut self, blob: &mut TextBlob) -> bool {
        unsafe { tl_paragraph_contains_color_font_or_bitmap(self.as_ptr_mut(), blob.as_ptr_mut()) }
    }
    pub fn get_line_number_at(&mut self, index: usize) -> i32 {
        unsafe { tl_paragraph_get_line_number_at(self.as_ptr_mut(), index) }
    }
    pub fn get_actual_text_range(
        &mut self,
        line_number: i32,
        include_spaces: bool,
    ) -> (usize, usize) {
        let mut start = 0usize;
        let mut end = 0usize;
        unsafe {
            tl_paragraph_get_actual_text_range(
                self.as_ptr_mut(),
                line_number,
                include_spaces,
                &mut start as *mut usize,
                &mut end as *mut usize,
            )
        };
        (start, end)
    }
}
