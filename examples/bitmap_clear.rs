use std::ffi::CStr;

use ckia::{
    bitmap::BitMap, canvas::Canvas, color::ColorSpace, image_info::ImageInfo, pixmap::PixMap,
    stream::FileWStream, AlphaType, ColorType, Color,
};

fn main() {
    let mut bm = BitMap::new();
    let mut info = ImageInfo::default();
    info.set_color_type(ColorType::BGRA_8888_SK_COLORTYPE);
    info.set_alpha_type(AlphaType::PREMUL_SK_ALPHATYPE);
    info.set_width(100);
    info.set_height(100);
    info.set_color_space(ColorSpace::new_srgb());

    bm.try_alloc_pixels(&info, 0).unwrap();
    assert!(bm.ready_to_draw());
    {
        let mut canvas = Canvas::from_bitmap(&bm);
        canvas.clear(Color::new(200, 134, 23, 0));
    }
    let mut fstream = FileWStream::new(CStr::from_bytes_with_nul(b"./skia_bitmap.png\0").unwrap())
        .expect("failed to open file");
    let pixmap = PixMap::new();
    if let Ok(pixmap) = bm.peek_pixels(pixmap) {
        pixmap
            .encode_png(&mut fstream, None, None)
            .expect("failed to write to file");
    }
}
