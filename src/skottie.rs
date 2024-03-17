use std::ffi::CStr;

use crate::bindings::*;
use crate::{
    canvas::Canvas, sksg::SkSgInvalidationController, stream::Stream, string::SkiaString, Rect,
    Size, SkottieAnimationRenderFlags,
};
crate::skia_wrapper!(
    nvrefcnt,
    SkottieAnimation,
    skottie_animation_t,
    skottie_animation_unref,
    skottie_animation_ref
);
impl SkottieAnimation {
    pub fn keep_alive() {
        unsafe { skottie_animation_keepalive() }
    }
    pub fn new_from_string(text: &str) -> Option<Self> {
        unsafe {
            Self::try_from_owned_ptr(skottie_animation_make_from_string(
                text.as_ptr() as _,
                text.len(),
            ))
        }
    }
    pub fn new_from_stream(stream: &mut impl Stream) -> Option<Self> {
        unsafe {
            Self::try_from_owned_ptr(skottie_animation_make_from_stream(
                stream.borrow_stream_mut_ptr(),
            ))
        }
    }
    pub fn new_from_file(path: &CStr) -> Option<Self> {
        unsafe { Self::try_from_owned_ptr(skottie_animation_make_from_file(path.as_ptr())) }
    }

    /*
    // why delete when it already has ref/unref???
    pub fn skottie_animation_delete(instance: *mut skottie_animation_t);
     */
    pub fn render(&mut self, canvas: &mut Canvas, dst: &mut Rect) {
        unsafe {
            skottie_animation_render(self.as_ptr_mut(), canvas.as_ptr_mut(), dst.as_ptr_mut())
        }
    }
    pub fn render_with_flags(
        &mut self,
        canvas: &mut Canvas,
        dst: &mut Rect,
        flags: SkottieAnimationRenderFlags,
    ) {
        unsafe {
            skottie_animation_render_with_flags(
                self.as_ptr_mut(),
                canvas.as_ptr_mut(),
                dst.as_ptr_mut(),
                flags,
            )
        }
    }
    pub fn seek(&mut self, t: f32, ic: &mut SkSgInvalidationController) {
        unsafe { skottie_animation_seek(self.as_ptr_mut(), t, ic.as_ptr_mut()) }
    }
    pub fn seek_frame(&mut self, t: f32, ic: &mut SkSgInvalidationController) {
        unsafe { skottie_animation_seek_frame(self.as_ptr_mut(), t, ic.as_ptr_mut()) }
    }
    pub fn seek_frame_time(&mut self, t: f32, ic: &mut SkSgInvalidationController) {
        unsafe { skottie_animation_seek_frame_time(self.as_ptr_mut(), t, ic.as_ptr_mut()) }
    }

    pub fn get_duration(&mut self) -> f64 {
        unsafe { skottie_animation_get_duration(self.as_ptr_mut()) }
    }
    pub fn get_fps(&mut self) -> f64 {
        unsafe { skottie_animation_get_fps(self.as_ptr_mut()) }
    }
    pub fn get_in_point(&mut self) -> f64 {
        unsafe { skottie_animation_get_in_point(self.as_ptr_mut()) }
    }
    pub fn get_out_point(&mut self) -> f64 {
        unsafe { skottie_animation_get_out_point(self.as_ptr_mut()) }
    }
    pub fn get_version(&mut self) -> SkiaString {
        let mut version = SkiaString::new_empty();
        unsafe { skottie_animation_get_version(self.as_ptr_mut(), version.as_ptr_mut()) }
        version
    }
    pub fn get_size(&mut self) -> Size {
        let mut size = Size::default();
        unsafe { skottie_animation_get_size(self.as_ptr_mut(), size.as_ptr_mut()) };
        size
    }
}
