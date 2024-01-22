use std::marker::PhantomData;

use crate::{
    canvas::Canvas,
    data::SkiaData,
    shader::Shader,
    stream::{Stream, WStream},
    FilterMode, Matrix, Rect, ShaderTileMode, SkiaPointer,
};
use ckia_sys::*;
crate::skia_wrapper!(
    refcnt,
    Picture,
    sk_picture_t,
    sk_picture_unref,
    sk_picture_ref
);

impl Picture {
    pub fn get_unique_id(&mut self) -> u32 {
        unsafe { sk_picture_get_unique_id(self.as_ptr_mut()) }
    }
    pub fn get_cull_rect(&mut self) -> Rect {
        let mut rect = Rect::default();
        unsafe { sk_picture_get_cull_rect(self.as_ptr_mut(), rect.as_ptr_mut()) }
        rect
    }
    pub fn make_shader(
        &mut self,
        tmx: ShaderTileMode,
        tmy: ShaderTileMode,
        mode: FilterMode,
        mat: &Matrix,
        tile: &Rect,
    ) -> Shader {
        unsafe {
            Shader::from_owned_ptr(sk_picture_make_shader(
                self.as_ptr_mut(),
                tmx,
                tmy,
                mode,
                mat.as_ptr(),
                tile.as_ptr(),
            ))
        }
    }
    pub fn serialize_to_data(&self) -> SkiaData {
        unsafe { SkiaData::from_owned_ptr(sk_picture_serialize_to_data(self.as_ptr())) }
    }
    pub fn serialize_to_stream(&self, stream: &mut impl WStream) {
        unsafe { sk_picture_serialize_to_stream(self.as_ptr(), stream.borrow_wstream_mut_ptr()) }
    }
    pub fn deserialize_from_stream(stream: &mut impl Stream) -> Option<Picture> {
        unsafe {
            Self::try_from_owned_ptr(sk_picture_deserialize_from_stream(
                stream.borrow_stream_mut_ptr(),
            ))
        }
    }
    pub fn deserialize_from_data(data: &mut SkiaData) -> Option<Self> {
        unsafe { Self::try_from_owned_ptr(sk_picture_deserialize_from_data(data.as_ptr_mut())) }
    }
    pub fn deserialize_from_memory(buffer: &mut [u8]) -> Option<Self> {
        unsafe {
            Self::try_from_owned_ptr(sk_picture_deserialize_from_memory(
                buffer.as_mut_ptr() as _,
                buffer.len(),
            ))
        }
    }
}

crate::skia_wrapper!(
    unique,
    PictureRecorder,
    sk_picture_recorder_t,
    sk_picture_recorder_delete
);
impl Default for PictureRecorder {
    fn default() -> Self {
        unsafe { Self::from_owned_ptr(sk_picture_recorder_new()) }
    }
}
impl PictureRecorder {
    pub fn begin_recording(&mut self, clip_bounds: &Rect) -> impl AsMut<Canvas> {
        let ptr =
            unsafe { sk_picture_recorder_begin_recording(self.as_ptr_mut(), clip_bounds.as_ptr()) };
        PicutreCanvas {
            inner: ptr,
            phantom: PhantomData,
        }
    }
    pub fn get_recording_canvas(&mut self) -> Option<Canvas> {
        unsafe { Canvas::try_from_owned_ptr(sk_picture_get_recording_canvas(self.as_ptr_mut())) }
    }

    pub fn end_recording(&mut self) -> Picture {
        unsafe { Picture::from_owned_ptr(sk_picture_recorder_end_recording(self.as_ptr_mut())) }
    }
    /*
    pub fn sk_picture_recorder_end_recording_as_drawable(
        arg1: *mut sk_picture_recorder_t,
    ) -> *mut sk_drawable_t; */
}
#[repr(transparent)]
struct PicutreCanvas<'a> {
    #[allow(unused)]
    inner: *mut sk_canvas_t,
    phantom: PhantomData<&'a mut Self>,
}
impl<'a> AsMut<Canvas> for PicutreCanvas<'a> {
    fn as_mut(&mut self) -> &mut Canvas {
        unsafe { std::mem::transmute(self) }
    }
}
