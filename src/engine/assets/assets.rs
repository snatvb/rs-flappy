use std::{
    cell::{RefCell, RefMut},
    rc::Rc,
};

use super::cache::AssetsCache;
use raylib::prelude::*;

pub struct Assets {
    root: String,
    cache: AssetsCache,
}

impl Assets {
    pub fn new(root: &str) -> Self {
        Self {
            root: root.to_owned(),
            cache: Default::default(),
        }
    }

    pub fn load_texture(
        &self,
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        filepath: &str,
    ) -> Result<Rc<Texture2D>, String> {
        let cache = self.cache.textures.read().unwrap();
        if let Some(texture) = cache.get(filepath) {
            log::debug!("Cache hit: Texture2D {filepath}");
            let texture = texture.to_owned();
            return Ok(texture);
        }
        drop(cache);

        let mut cache = self.cache.textures.write().unwrap();
        let path = format!("{}/{}", self.root, filepath);
        let texture = Rc::new(rl.load_texture(thread, &path)?);
        cache.insert(filepath.to_owned(), texture.clone());

        Ok(texture)
    }
}
