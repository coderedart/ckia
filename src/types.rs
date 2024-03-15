#![allow(non_snake_case)]
use ckia_sys::*;

use crate::{color::ColorSpace, SkiaPointer};

pub type ColorType = sk_colortype_t;
pub type AlphaType = sk_alphatype_t;
pub type PixelGeometry = sk_pixelgeometry_t;
pub type SurfacePropsFlags = sk_surfaceprops_flags_t;
pub type BlendMode = sk_blendmode_t;
pub type PointMode = sk_point_mode_t;
pub type TextAlign = sk_text_align_t;
pub type TextEncoding = sk_text_encoding_t;
pub type PathFillType = sk_path_filltype_t;
pub type FontStyleSlant = sk_font_style_slant_t;
pub type ColorChannel = sk_color_channel_t;
pub type RegionOp = sk_region_op_t;
pub type ClipOp = sk_clipop_t;
pub type EncodedImageFormat = sk_encoded_image_format_t;
pub type EncodedOrigin = sk_encodedorigin_t;
pub type CodecResult = sk_codec_result_t;
pub type CodecZeroInitialized = sk_codec_zero_initialized_t;
pub type CodecScanlineOrder = sk_codec_scanline_order_t;
pub type PathVerb = sk_path_verb_t;
pub type PathAddMode = sk_path_add_mode_t;
pub type PathSegmentMask = sk_path_segment_mask_t;
pub type PathEffect1DStyle = sk_path_effect_1d_style_t;
pub type PathEffectTrimMode = sk_path_effect_trim_mode_t;
pub type StrokeCap = sk_stroke_cap_t;
pub type StrokeJoin = sk_stroke_join_t;
pub type ShaderTileMode = sk_shader_tilemode_t;
pub type BlurStyle = sk_blurstyle_t;
pub type PathDirection = sk_path_direction_t;
pub type PathArcSize = sk_path_arc_size_t;
pub type PaintStyle = sk_paint_style_t;
pub type FontHinting = sk_font_hinting_t;
pub type FontEdging = sk_font_edging_t;
pub type SurfaceOrigin = gr_surfaceorigin_t;
pub type Backend = gr_backend_t;
pub type PathOp = sk_pathop_t;
pub type LatticeRecttype = sk_lattice_recttype_t;
pub type PathMeasureMatrixflags = sk_pathmeasure_matrixflags_t;
pub type ImageCachingHint = sk_image_caching_hint_t;
pub type BitmapAllocFlags = sk_bitmap_allocflags_t;
pub type CodecanimationDisposalmethod = sk_codecanimation_disposalmethod_t;
pub type CodecanimationBlend = sk_codecanimation_blend_t;
pub type VerticesVertexMode = sk_vertices_vertex_mode_t;
pub type HighContrastConfigInvertstyle = sk_highcontrastconfig_invertstyle_t;
pub type PngEncoderFilterFlags = sk_pngencoder_filterflags_t;
pub type JpegEncoderDownsample = sk_jpegencoder_downsample_t;
pub type JpegEncoderAlphaOption = sk_jpegencoder_alphaoption_t;
pub type WebpEncoderCompression = sk_webpencoder_compression_t;
pub type RRectType = sk_rrect_type_t;
pub type RRectCorner = sk_rrect_corner_t;
pub type RuntimeEffectUniformType = sk_runtimeeffect_uniform_type_t;
pub type RuntimeEffectChildType = sk_runtimeeffect_child_type_t;
pub type RuntimeEffectUniformFlags = sk_runtimeeffect_uniform_flags_t;
pub type FilterMode = sk_filter_mode_t;
pub type MipmapMode = sk_mipmap_mode_t;
pub type SkottieAnimationRenderFlags = skottie_animation_renderflags_t;

pub type Affinity = tl_affinity_t;
pub type RectHeightStyle = tl_rect_height_style_t;
pub type RectWidthStyle = tl_rect_width_style_t;
pub type ParagraphTextAlign = tl_text_align_t;
pub type TextDirection = tl_text_direction_t;
pub type TextBaseline = tl_text_baseline_t;
pub type TextHeightBehavior = tl_text_height_behavior_t;
pub type LineMetricStyle = tl_line_metric_style_t;
pub type TextDecoration = tl_text_decoration_t;
pub type TextDecorationStyle = tl_text_decoration_style_t;
pub type TextDecorationMode = tl_text_decoration_mode_t;
pub type StyleType = tl_style_type_t;
pub type PlaceholderAlignment = tl_placeholder_alignment_t;

crate::pod_struct!(pub Color4f, sk_color4f_t {
    pub fR: f32,
    pub fG: f32,
    pub fB: f32,
    pub fA: f32,
});
crate::pod_struct!(pub Point, sk_point_t {
    pub x: f32,
    pub y: f32,
});
crate::pod_struct!(pub IRect, sk_irect_t {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
});
crate::pod_struct!(pub Rect, sk_rect_t {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
});
crate::pod_struct!(pub Matrix, sk_matrix_t {
    pub scaleX: f32,
    pub skewX: f32,
    pub transX: f32,
    pub skewY: f32,
    pub scaleY: f32,
    pub transY: f32,
    pub persp0: f32,
    pub persp1: f32,
    pub persp2: f32,
});
crate::pod_struct!(pub Matrix44, sk_matrix44_t {
    pub m00: f32,
    pub m01: f32,
    pub m02: f32,
    pub m03: f32,
    pub m10: f32,
    pub m11: f32,
    pub m12: f32,
    pub m13: f32,
    pub m20: f32,
    pub m21: f32,
    pub m22: f32,
    pub m23: f32,
    pub m30: f32,
    pub m31: f32,
    pub m32: f32,
    pub m33: f32,
});
crate::pod_struct!(pub Point3, sk_point3_t {
    pub x: f32,
    pub y: f32,
    pub z: f32,
});
crate::pod_struct!(pub IPoint, sk_ipoint_t {
    pub x: i32,
    pub y: i32,
});
crate::pod_struct!(pub Size, sk_size_t {
    pub w: f32,
    pub h: f32,
});
crate::pod_struct!(pub ISize, sk_isize_t {
    pub w: i32,
    pub h: i32,
});
crate::pod_struct!(pub FontMetrics, sk_fontmetrics_t {
    pub fFlags: u32,
    pub fTop: f32,
    pub fAscent: f32,
    pub fDescent: f32,
    pub fBottom: f32,
    pub fLeading: f32,
    pub fAvgCharWidth: f32,
    pub fMaxCharWidth: f32,
    pub fXMin: f32,
    pub fXMax: f32,
    pub fXHeight: f32,
    pub fCapHeight: f32,
    pub fUnderlineThickness: f32,
    pub fUnderlinePosition: f32,
    pub fStrikeoutThickness: f32,
    pub fStrikeoutPosition: f32,
});
crate::pod_struct!(pub CodecOptions, sk_codec_options_t {
    pub fZeroInitialized: sk_codec_zero_initialized_t,
    pub fSubset: *mut sk_irect_t,
    pub fFrameIndex: ::std::os::raw::c_int,
    pub fPriorFrame: ::std::os::raw::c_int,
});
crate::pod_struct!(pub ContextOptions, gr_context_options_t {
    pub fAvoidStencilBuffers: bool,
    pub fRuntimeProgramCacheSize: ::std::os::raw::c_int,
    pub fGlyphCacheTextureMaximumBytes: usize,
    pub fAllowPathMaskCaching: bool,
    pub fDoManualMipmapping: bool,
    pub fBufferMapThreshold: ::std::os::raw::c_int,
});
crate::pod_struct!(pub GlTextureInfo, gr_gl_textureinfo_t {
    pub fTarget: ::std::os::raw::c_uint,
    pub fID: ::std::os::raw::c_uint,
    pub fFormat: ::std::os::raw::c_uint,
    pub fProtected: bool,
});
crate::pod_struct!(pub GlFramebufferInfo, gr_gl_framebufferinfo_t {
    pub fFBOID: ::std::os::raw::c_uint,
    pub fFormat: ::std::os::raw::c_uint,
    pub fProtected: bool,
});
crate::pod_struct!(pub VkBackendContext, gr_vk_backendcontext_t {
    pub fInstance: *mut vk_instance_t,
    pub fPhysicalDevice: *mut vk_physical_device_t,
    pub fDevice: *mut vk_device_t,
    pub fQueue: *mut vk_queue_t,
    pub fGraphicsQueueIndex: u32,
    pub fMinAPIVersion: u32,
    pub fInstanceVersion: u32,
    pub fMaxAPIVersion: u32,
    pub fExtensions: u32,
    pub fVkExtensions: *const gr_vk_extensions_t,
    pub fFeatures: u32,
    pub fDeviceFeatures: *const vk_physical_device_features_t,
    pub fDeviceFeatures2: *const vk_physical_device_features_2_t,
    pub fMemoryAllocator: *mut gr_vk_memory_allocator_t,
    pub fGetProc: gr_vk_get_proc,
    pub fGetProcUserData: *mut ::std::os::raw::c_void,
    pub fOwnsInstanceAndDevice: bool,
    pub fProtectedContext: bool,
});
crate::pod_struct!(pub VkAlloc, gr_vk_alloc_t {
    pub fMemory: u64,
    pub fOffset: u64,
    pub fSize: u64,
    pub fFlags: u32,
    pub fBackendMemory: gr_vk_backendmemory_t,
    pub _private_fUsesSystemHeap: bool,
});
crate::pod_struct!(pub VkYcbcrConversioninfo, gr_vk_ycbcrconversioninfo_t {
    pub fFormat: u32,
    pub fExternalFormat: u64,
    pub fYcbcrModel: u32,
    pub fYcbcrRange: u32,
    pub fXChromaOffset: u32,
    pub fYChromaOffset: u32,
    pub fChromaFilter: u32,
    pub fForceExplicitReconstruction: u32,
    pub fFormatFeatures: u32,
});
crate::pod_struct!(pub VkImageInfo, gr_vk_imageinfo_t {
    pub fImage: u64,
    pub fAlloc: gr_vk_alloc_t,
    pub fImageTiling: u32,
    pub fImageLayout: u32,
    pub fFormat: u32,
    pub fImageUsageFlags: u32,
    pub fSampleCount: u32,
    pub fLevelCount: u32,
    pub fCurrentQueueFamily: u32,
    pub fProtected: bool,
    pub fYcbcrConversionInfo: gr_vk_ycbcrconversioninfo_t,
    pub fSharingMode: u32,
});
crate::pod_struct!(pub MtlTextureInfo, gr_mtl_textureinfo_t {
    pub fTexture: *const ::std::os::raw::c_void,
});
crate::pod_struct!(pub Lattice, sk_lattice_t {
    pub fXDivs: *const ::std::os::raw::c_int,
    pub fYDivs: *const ::std::os::raw::c_int,
    pub fRectTypes: *const sk_lattice_recttype_t,
    pub fXCount: ::std::os::raw::c_int,
    pub fYCount: ::std::os::raw::c_int,
    pub fBounds: *const sk_irect_t,
    pub fColors: *const sk_color_t,
});
crate::pod_struct!(pub TimeDatetime, sk_time_datetime_t {
    pub fTimeZoneMinutes: i16,
    pub fYear: u16,
    pub fMonth: u8,
    pub fDayOfWeek: u8,
    pub fDay: u8,
    pub fHour: u8,
    pub fMinute: u8,
    pub fSecond: u8,
});
crate::pod_struct!(pub DocumentPdfMetadata, sk_document_pdf_metadata_t {
    pub fTitle: *mut sk_string_t,
    pub fAuthor: *mut sk_string_t,
    pub fSubject: *mut sk_string_t,
    pub fKeywords: *mut sk_string_t,
    pub fCreator: *mut sk_string_t,
    pub fProducer: *mut sk_string_t,
    pub fCreation: *mut sk_time_datetime_t,
    pub fModified: *mut sk_time_datetime_t,
    pub fRasterDPI: f32,
    pub fPDFA: bool,
    pub fEncodingQuality: ::std::os::raw::c_int,
});
// crate::pod_struct!(pub ImageInfo, sk_imageinfo_t {
//     pub colorspace: *mut sk_colorspace_t,
//     pub width: i32,
//     pub height: i32,
//     pub colorType: sk_colortype_t,
//     pub alphaType: sk_alphatype_t,
// });
crate::pod_struct!(pub CodecFrameInfo, sk_codec_frameinfo_t {
    pub fRequiredFrame: ::std::os::raw::c_int,
    pub fDuration: ::std::os::raw::c_int,
    pub fFullyReceived: bool,
    pub fAlphaType: sk_alphatype_t,
    pub fHasAlphaWithinBounds: bool,
    pub fDisposalMethod: sk_codecanimation_disposalmethod_t,
    pub fBlend: sk_codecanimation_blend_t,
    pub fFrameRect: sk_irect_t,
});
crate::pod_struct!(pub ColorspaceTransferFn, sk_colorspace_transfer_fn_t {
    pub fG: f32,
    pub fA: f32,
    pub fB: f32,
    pub fC: f32,
    pub fD: f32,
    pub fE: f32,
    pub fF: f32,
});
crate::pod_struct!(pub ColorspacePrimaries, sk_colorspace_primaries_t {
    pub fRX: f32,
    pub fRY: f32,
    pub fGX: f32,
    pub fGY: f32,
    pub fBX: f32,
    pub fBY: f32,
    pub fWX: f32,
    pub fWY: f32,
});
crate::pod_struct!(pub ColorspaceXyz, sk_colorspace_xyz_t {
    pub fM00: f32,
    pub fM01: f32,
    pub fM02: f32,
    pub fM10: f32,
    pub fM11: f32,
    pub fM12: f32,
    pub fM20: f32,
    pub fM21: f32,
    pub fM22: f32,
});
crate::pod_struct!(pub Highcontrastconfig, sk_highcontrastconfig_t {
    pub fGrayscale: bool,
    pub fInvertStyle: sk_highcontrastconfig_invertstyle_t,
    pub fContrast: f32,
});
crate::pod_struct!(pub PngEncoderOptions, sk_pngencoder_options_t {
    pub fFilterFlags: sk_pngencoder_filterflags_t,
    pub fZLibLevel: ::std::os::raw::c_int,
    pub fComments: *mut ::std::os::raw::c_void,
    pub fICCProfile: *const sk_colorspace_icc_profile_t,
    pub fICCProfileDescription: *const ::std::os::raw::c_char,
});
crate::pod_struct!(pub JpegEncoderOptions, sk_jpegencoder_options_t {
    pub fQuality: ::std::os::raw::c_int,
    pub fDownsample: sk_jpegencoder_downsample_t,
    pub fAlphaOption: sk_jpegencoder_alphaoption_t,
    pub xmpMetadata: *const sk_data_t,
    pub fICCProfile: *const sk_colorspace_icc_profile_t,
    pub fICCProfileDescription: *const ::std::os::raw::c_char,
});
crate::pod_struct!(pub WebpEncoderOptions, sk_webpencoder_options_t {
    pub fCompression: sk_webpencoder_compression_t,
    pub fQuality: f32,
    pub fICCProfile: *const sk_colorspace_icc_profile_t,
    pub fICCProfileDescription: *const ::std::os::raw::c_char,
});
crate::pod_struct!(pub TextBlobBuilderRunbuffer, sk_textblob_builder_runbuffer_t {
    pub glyphs: *mut ::std::os::raw::c_void,
    pub pos: *mut ::std::os::raw::c_void,
    pub utf8text: *mut ::std::os::raw::c_void,
    pub clusters: *mut ::std::os::raw::c_void,
});
crate::pod_struct!(pub Rsxform, sk_rsxform_t {
    pub fSCos: f32,
    pub fSSin: f32,
    pub fTX: f32,
    pub fTY: f32,
});
crate::pod_struct!(pub RuntimeEffectUniform, sk_runtimeeffect_uniform_t {
    pub fName: *const ::std::os::raw::c_char,
    pub fNameLength: usize,
    pub fOffset: usize,
    pub fType: sk_runtimeeffect_uniform_type_t,
    pub fCount: ::std::os::raw::c_int,
    pub fFlags: sk_runtimeeffect_uniform_flags_t,
});
crate::pod_struct!(pub RuntimeEffectChild, sk_runtimeeffect_child_t {
    pub fName: *const ::std::os::raw::c_char,
    pub fNameLength: usize,
    pub fType: sk_runtimeeffect_child_type_t,
    pub fIndex: ::std::os::raw::c_int,
});
crate::pod_struct!(pub CubicResampler, sk_cubic_resampler_t {
    pub fB: f32,
    pub fC: f32,
});
crate::pod_struct!(pub SamplingOptions, sk_sampling_options_t {
    pub fMaxAniso: ::std::os::raw::c_int,
    pub fUseCubic: bool,
    pub fCubic: sk_cubic_resampler_t,
    pub fFilter: sk_filter_mode_t,
    pub fMipmap: sk_mipmap_mode_t,
});

crate::pod_struct!(pub PositionWithAffinity, tl_position_with_affinity_t {
    pub position: i32,
    pub affinity: tl_affinity_t,
});
crate::pod_struct!(pub TextBox, tl_text_box_t {
    pub rect: sk_rect_t,
    pub direction: tl_text_direction_t,
});
crate::pod_struct!(pub TextShadow, tl_text_shadow_t {
    pub fColor: sk_color_t,
    pub fOffset: sk_point_t,
    pub fBlurSigma: f64,
});
crate::pod_struct!(pub PlaceholderStyle, tl_placeholder_style_t {
    pub fWidth: f32,
    pub fHeight: f32,
    pub fAlignment: tl_placeholder_alignment_t,
    pub fBaseline: tl_text_baseline_t,
    pub fBaselineOffset: f32,
});
crate::pod_struct!(pub Decoration, tl_decoration_t {
    pub fType: tl_text_decoration_t,
    pub fMode: tl_text_decoration_mode_t,
    pub fColor: sk_color_t,
    pub fStyle: tl_text_decoration_style_t,
    pub fThicknessMultiplier: f32,
});
crate::pod_struct!(pub DashPathEffect, tl_dash_path_effect {
    pub fOnLength: f32,
    pub fOffLength: f32,
});

impl Default for Rect {
    fn default() -> Self {
        Self::ZERO
    }
}
impl Rect {
    pub const ZERO: Self = Self::new(0.0, 0.0, 0.0, 0.0);
    pub const INF: Self = Self::new(0.0, 0.0, f32::INFINITY, f32::INFINITY);
    pub const MAX: Self = Self::new(f32::MAX, f32::MAX, f32::MAX, f32::MAX);
    pub const fn new(left: f32, top: f32, right: f32, bottom: f32) -> Self {
        Self {
            left,
            top,
            right,
            bottom,
        }
    }
}
impl Default for IRect {
    fn default() -> Self {
        Self::ZERO
    }
}
impl IRect {
    pub const ZERO: Self = Self::new(0, 0, 0, 0);
    pub const MAX: Self = Self::new(i32::MAX, i32::MAX, i32::MAX, i32::MAX);
    pub const fn new(left: i32, top: i32, right: i32, bottom: i32) -> Self {
        Self {
            left,
            top,
            right,
            bottom,
        }
    }
}
impl Default for Point {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}
impl Point {
    pub const ZERO: Self = Self::new(0.0, 0.0);
    pub const ONE: Self = Self::new(1.0, 1.0);
    pub const X: Self = Self::new(1.0, 0.0);
    pub const Y: Self = Self::new(0.0, 1.0);
    pub const INF: Self = Self::new(f32::INFINITY, f32::INFINITY);

    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}
pub type Vector = Point;
impl Default for Size {
    fn default() -> Self {
        Self::new(0.0, 0.0)
    }
}
impl Size {
    pub const ZERO: Self = Self::new(0.0, 0.0);
    pub const INF: Self = Self::new(f32::INFINITY, f32::INFINITY);

    pub const fn new(w: f32, h: f32) -> Self {
        Self { w, h }
    }
}
impl GlTextureInfo {
    /// target is the texture binding target
    /// id is the gl texture
    /// format is the texture format
    /// what is protected???
    /// # Safety
    /// make sure that the arguments are valid
    pub unsafe fn new(target: u32, id: u32, format: u32, protected: bool) -> Self {
        Self {
            fTarget: target,
            fID: id,
            fFormat: format,
            fProtected: protected,
        }
    }
}
impl SamplingOptions {
    pub const LINEAR: Self = SamplingOptions {
        fMaxAniso: 0,
        fUseCubic: false,
        fCubic: CubicResampler { fB: 0.0, fC: 0.0 }.into_native(),
        fFilter: FilterMode::LINEAR_SK_FILTER_MODE,
        fMipmap: MipmapMode::LINEAR_SK_MIPMAP_MODE,
    };
}
impl Default for SamplingOptions {
    fn default() -> Self {
        Self::LINEAR
    }
}
impl GlFramebufferInfo {
    pub unsafe fn new(id: u32, format: u32, protected: bool) -> Self {
        Self {
            fFBOID: id,
            fFormat: format,
            fProtected: protected,
        }
    }
}
pub const DEFAULT_COLOR_TYPE: ColorType = ColorType::BGRA_8888_SK_COLORTYPE;
pub const DEFAULT_ALPHA_TYPE: AlphaType = AlphaType::PREMUL_SK_ALPHATYPE;
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
#[derive(Debug)]
#[repr(transparent)]
pub struct ImageInfo(sk_imageinfo_t);

impl Drop for ImageInfo {
    fn drop(&mut self) {
        unsafe { ColorSpace::try_from_owned_ptr(self.0.colorspace) };
    }
}

impl Default for ImageInfo {
    fn default() -> Self {
        Self(sk_imageinfo_t {
            colorspace: std::ptr::null_mut(),
            width: 0,
            height: 0,
            colorType: DEFAULT_COLOR_TYPE,
            alphaType: DEFAULT_ALPHA_TYPE,
        })
    }
}
#[allow(unused)]
impl ImageInfo {
    pub(crate) fn as_ptr(&self) -> *const sk_imageinfo_t {
        &self.0 as _
    }
    pub(crate) fn as_ptr_mut(&mut self) -> *mut sk_imageinfo_t {
        &mut self.0 as _
    }
    pub fn get_colorspace(&self) -> Option<&ColorSpace> {
        unsafe {
            if self.0.colorspace.is_null() {
                None
            } else {
                Some(std::mem::transmute::<&*mut sk_colorspace_t, &ColorSpace>(
                    &self.0.colorspace,
                ))
            }
        }
    }
    pub fn set_colorspace(&mut self, colorspace: Option<ColorSpace>) {
        unsafe {
            ColorSpace::try_from_owned_ptr(self.0.colorspace);
            self.0.colorspace = colorspace
                .map(|cs| cs.into_owned_ptr())
                .unwrap_or(std::ptr::null_mut());
        }
    }
    pub fn get_width(&self) -> i32 {
        self.0.width
    }
    pub fn set_width(&mut self, width: i32) {
        self.0.width = width;
    }
    pub fn get_height(&self) -> i32 {
        self.0.height
    }
    pub fn set_height(&mut self, height: i32) {
        self.0.height = height;
    }
    pub fn get_color_type(&self) -> sk_colortype_t {
        self.0.colorType
    }
    pub fn set_color_type(&mut self, colorType: sk_colortype_t) {
        self.0.colorType = colorType;
    }
    pub fn get_alpha_type(&self) -> sk_alphatype_t {
        self.0.alphaType
    }
    pub fn set_alpha_type(&mut self, alphaType: sk_alphatype_t) {
        self.0.alphaType = alphaType;
    }

    pub fn bytes_per_pixel(&self) -> u8 {
        bytes_per_pixel(self.0.colorType)
    }
}
