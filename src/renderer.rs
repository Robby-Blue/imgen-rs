use image::ImageBuffer;
use image::RgbaImage;

use crate::{primitives::primitives::Primitive, types::values::SizePx};

pub struct RenderContext {
    pub image: RgbaImage,
}

impl RenderContext {
    pub fn new(size: SizePx) -> Self {
        let image = ImageBuffer::new(size.w as u32, size.h as u32);

        RenderContext { image }
    }
}

pub fn render(size: SizePx, primitives: Vec<Box<dyn Primitive>>) -> RenderContext {
    let mut ctx = RenderContext::new(size);

    for primitive in primitives {
        primitive.draw(&mut ctx);
    }

    ctx
}
