use raylib::prelude::*;

pub struct Renderer {
    pub width: u32,
    pub height: u32,
    pub texture: RenderTexture2D,
}

impl Renderer {
    pub fn new(
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        width: u32,
        height: u32,
    ) -> Result<Self, String> {
        Ok(Self {
            width,
            height,
            texture: rl.load_render_texture(thread, width, height)?,
        })
    }
}
