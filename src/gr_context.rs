use std::ffi::CStr;

use crate::SkiaPointer;
use ckia_sys::*;

use crate::*;

crate::skia_wrapper!(
    unique,
    GrRecordingContext,
    gr_recording_context_t,
    gr_recording_context_unref
);
impl GrRecordingContext {
    pub fn get_max_surface_sample_count_for_color_type(&mut self, color_type: ColorType) -> i32 {
        unsafe {
            gr_recording_context_get_max_surface_sample_count_for_color_type(
                self.as_ptr_mut(),
                color_type,
            )
        }
    }
    pub fn get_backend(&mut self) -> Backend {
        unsafe { gr_recording_context_get_backend(self.as_ptr_mut()) }
    }
    pub fn is_abandoned(&mut self) -> bool {
        unsafe { gr_recording_context_is_abandoned(self.as_ptr_mut()) }
    }
    pub fn max_texture_size(&mut self) -> i32 {
        unsafe { gr_recording_context_max_texture_size(self.as_ptr_mut()) }
    }
    pub fn max_render_target_size(&mut self) -> i32 {
        unsafe { gr_recording_context_max_render_target_size(self.as_ptr_mut()) }
    }
}
crate::skia_wrapper!(
    unique,
    DirectContext,
    gr_direct_context_t,
    gr_direct_context_release_resources_and_abandon_context
);
/// Direct Context inherits Recording context
impl AsMut<GrRecordingContext> for DirectContext {
    fn as_mut(&mut self) -> &mut GrRecordingContext {
        unsafe { std::mem::transmute(self) }
    }
}
impl DirectContext {
    pub fn make_gl(interface: &GlInterface) -> Self {
        unsafe { Self::from_owned_ptr(gr_direct_context_make_gl(interface.as_ptr())) }
    }
    pub fn make_gl_with_options(interface: &GlInterface, options: &ContextOptions) -> Self {
        unsafe {
            Self::from_owned_ptr(gr_direct_context_make_gl_with_options(
                interface.as_ptr(),
                options.as_ptr(),
            ))
        }
    }
    /*pub fn gr_direct_context_make_vulkan(
        vkBackendContext: gr_vk_backendcontext_t,
    ) -> *mut gr_direct_context_t;
    pub fn gr_direct_context_make_vulkan_with_options(
        vkBackendContext: gr_vk_backendcontext_t,
        options: *const gr_context_options_t,
    ) -> *mut gr_direct_context_t;
    pub fn gr_direct_context_make_metal(
        device: *mut ::std::os::raw::c_void,
        queue: *mut ::std::os::raw::c_void,
    ) -> *mut gr_direct_context_t;
    pub fn gr_direct_context_make_metal_with_options(
        device: *mut ::std::os::raw::c_void,
        queue: *mut ::std::os::raw::c_void,
        options: *const gr_context_options_t,
    ) -> *mut gr_direct_context_t; */
    pub fn is_abandoned(&mut self) -> bool {
        unsafe { gr_direct_context_is_abandoned(self.as_ptr_mut()) }
    }
    pub fn abandon_context(&mut self) {
        unsafe { gr_direct_context_abandon_context(self.as_ptr_mut()) }
    }
    pub fn release_resources_and_abandon_context(&mut self) {
        unsafe { gr_direct_context_release_resources_and_abandon_context(self.as_ptr_mut()) }
    }
    pub fn get_resource_cache_limit(&mut self) -> usize {
        unsafe { gr_direct_context_get_resource_cache_limit(self.as_ptr_mut()) }
    }
    pub fn set_resource_cache_limit(&mut self, max_resource_bytes: usize) {
        unsafe { gr_direct_context_set_resource_cache_limit(self.as_ptr_mut(), max_resource_bytes) }
    }
    /// returns (resource_count, max_resource_bytes_used)
    pub fn get_resource_cache_usage(&mut self) -> (i32, usize) {
        let mut resource_count = 0;
        let mut max_resource_bytes = 0;
        unsafe {
            gr_direct_context_get_resource_cache_usage(
                self.as_ptr_mut(),
                &mut resource_count as _,
                &mut max_resource_bytes as _,
            );
        }
        (resource_count, max_resource_bytes)
    }
    pub fn flush(&mut self) {
        unsafe { gr_direct_context_flush(self.as_ptr_mut()) }
    }
    pub fn submit(&mut self, sync_cpu: bool) -> bool {
        unsafe { gr_direct_context_submit(self.as_ptr_mut(), sync_cpu) }
    }
    pub fn flush_and_submit(&mut self, sync_cpu: bool) {
        unsafe { gr_direct_context_flush_and_submit(self.as_ptr_mut(), sync_cpu) }
    }
    pub fn reset_context(&mut self, state: u32) {
        unsafe { gr_direct_context_reset_context(self.as_ptr_mut(), state) }
    }
    /*
    pub fn gr_direct_context_dump_memory_statistics(
        context: *const gr_direct_context_t,
        dump: *mut sk_tracememorydump_t,
    ); */
    pub fn free_gpu_resources(&mut self) {
        unsafe { gr_direct_context_free_gpu_resources(self.as_ptr_mut()) }
    }
    pub fn perform_deferred_cleanup(&mut self, ms: i64) {
        unsafe { gr_direct_context_perform_deferred_cleanup(self.as_ptr_mut(), ms) }
    }
    pub fn purge_unlocked_resources_bytes(
        &mut self,
        bytes_to_purge: usize,
        prefer_scratch_resources: bool,
    ) {
        unsafe {
            gr_direct_context_purge_unlocked_resources_bytes(
                self.as_ptr_mut(),
                bytes_to_purge,
                prefer_scratch_resources,
            )
        }
    }
    pub fn purge_unlocked_resources(&mut self, scratch_resources_only: bool) {
        unsafe {
            gr_direct_context_purge_unlocked_resources(self.as_ptr_mut(), scratch_resources_only)
        }
    }
}
crate::skia_wrapper!(shared, GlInterface, gr_glinterface_t, gr_glinterface_unref);
impl GlInterface {
    pub fn create_native_interface() -> Self {
        unsafe {
            let inner = gr_glinterface_create_native_interface();
            assert!(!inner.is_null());
            Self { inner: inner as _ }
        }
    }
    pub unsafe fn assemble_interface(ctx: *mut std::ffi::c_void, get: gr_gl_get_proc) -> Self {
        let inner = gr_glinterface_assemble_interface(ctx, get);
        assert!(!inner.is_null());
        Self { inner: inner as _ }
    }
    pub unsafe fn assemble_gl_interface(ctx: *mut std::ffi::c_void, get: gr_gl_get_proc) -> Self {
        let inner = gr_glinterface_assemble_gl_interface(ctx, get);
        assert!(!inner.is_null());
        Self { inner: inner as _ }
    }
    pub unsafe fn assemble_gles_interface(ctx: *mut std::ffi::c_void, get: gr_gl_get_proc) -> Self {
        let inner = gr_glinterface_assemble_gles_interface(ctx, get);
        assert!(!inner.is_null());
        Self { inner: inner as _ }
    }
    pub unsafe fn assemble_webgl_interface(
        ctx: *mut std::ffi::c_void,
        get: gr_gl_get_proc,
    ) -> Self {
        let inner = gr_glinterface_assemble_webgl_interface(ctx, get);
        assert!(!inner.is_null());
        Self { inner: inner as _ }
    }
    pub fn validate(&self) -> bool {
        unsafe { gr_glinterface_validate(self.as_ptr()) }
    }
    pub fn has_extension(&self, extension: &CStr) -> bool {
        unsafe { gr_glinterface_has_extension(self.as_ptr(), extension.as_ptr()) }
    }
}

/*

pub fn gr_vk_extensions_new() -> *mut gr_vk_extensions_t;
pub fn gr_vk_extensions_delete(extensions: *mut gr_vk_extensions_t);
pub fn gr_vk_extensions_init(
    extensions: *mut gr_vk_extensions_t,
    getProc: gr_vk_get_proc,
    userData: *mut ::std::os::raw::c_void,
    instance: *mut vk_instance_t,
    physDev: *mut vk_physical_device_t,
    instanceExtensionCount: u32,
    instanceExtensions: *mut *const ::std::os::raw::c_char,
    deviceExtensionCount: u32,
    deviceExtensions: *mut *const ::std::os::raw::c_char,
);
pub fn gr_vk_extensions_has_extension(
    extensions: *mut gr_vk_extensions_t,
    ext: *const ::std::os::raw::c_char,
    minVersion: u32,
) -> bool;
*/
crate::skia_wrapper!(
    unique,
    BackendTexture,
    gr_backendtexture_t,
    gr_backendtexture_delete
);
impl BackendTexture {
    pub fn new_gl(width: i32, height: i32, mipmapped: bool, info: &GlTextureInfo) -> Self {
        unsafe {
            Self::from_owned_ptr(gr_backendtexture_new_gl(
                width,
                height,
                mipmapped,
                info.as_ptr(),
            ))
        }
    }

    /*

    pub fn gr_backendtexture_new_vulkan(
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        vkInfo: *const gr_vk_imageinfo_t,
    ) -> *mut gr_backendtexture_t;
    pub fn gr_backendtexture_new_metal(
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        mipmapped: bool,
        mtlInfo: *const gr_mtl_textureinfo_t,
    ) -> *mut gr_backendtexture_t;
    */
    pub fn is_valid(&self) -> bool {
        unsafe { gr_backendtexture_is_valid(self.as_ptr()) }
    }
    pub fn get_width(&self) -> i32 {
        unsafe { gr_backendtexture_get_width(self.as_ptr()) }
    }
    pub fn get_height(&self) -> i32 {
        unsafe { gr_backendtexture_get_height(self.as_ptr()) }
    }
    pub fn has_mipmaps(&self) -> bool {
        unsafe { gr_backendtexture_has_mipmaps(self.as_ptr()) }
    }
    pub fn get_backend(&self) -> Backend {
        unsafe { gr_backendtexture_get_backend(self.as_ptr()) }
    }
    pub fn get_gl_texture_info(&self) -> Option<GlTextureInfo> {
        unsafe {
            let mut info = GlTextureInfo::new(0, 0, 0, false);
            gr_backendtexture_get_gl_textureinfo(self.as_ptr(), info.as_ptr_mut()).then_some(info)
        }
    }
}

crate::skia_wrapper!(
    unique,
    BackendRenderTarget,
    gr_backendrendertarget_t,
    gr_backendrendertarget_delete
);
impl BackendRenderTarget {
    pub fn new_gl(
        width: i32,
        height: i32,
        samples: i32,
        stencils: i32,
        info: &GlFramebufferInfo,
    ) -> Self {
        unsafe {
            Self::from_owned_ptr(gr_backendrendertarget_new_gl(
                width,
                height,
                samples,
                stencils,
                info.as_ptr(),
            ))
        }
    }

    /*
    pub fn gr_backendrendertarget_new_vulkan(
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        vkInfo: *const gr_vk_imageinfo_t,
    ) -> *mut gr_backendrendertarget_t;
    pub fn gr_backendrendertarget_new_metal(
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        mipmapped: bool,
        mtlInfo: *const gr_mtl_textureinfo_t,
    ) -> *mut gr_backendrendertarget_t;
    */
    pub fn is_valid(&self) -> bool {
        unsafe { gr_backendrendertarget_is_valid(self.as_ptr()) }
    }
    pub fn get_width(&self) -> i32 {
        unsafe { gr_backendrendertarget_get_width(self.as_ptr()) }
    }
    pub fn get_height(&self) -> i32 {
        unsafe { gr_backendrendertarget_get_height(self.as_ptr()) }
    }
    pub fn get_samples(&self) -> i32 {
        unsafe { gr_backendrendertarget_get_samples(self.as_ptr()) }
    }
    pub fn get_stencils(&self) -> i32 {
        unsafe { gr_backendrendertarget_get_stencils(self.as_ptr()) }
    }
    pub fn get_backend(&self) -> Backend {
        unsafe { gr_backendrendertarget_get_backend(self.as_ptr()) }
    }
    pub fn get_gl_framebuffer_info(&self) -> Option<GlFramebufferInfo> {
        unsafe {
            let mut info = GlFramebufferInfo::new(0, 0, false);

            gr_backendrendertarget_get_gl_framebufferinfo(self.as_ptr(), info.as_ptr_mut())
                .then_some(info)
        }
    }
}
