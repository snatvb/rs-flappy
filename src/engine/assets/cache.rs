use std::{collections::HashMap, rc::Rc, sync::RwLock};

use derive_more::Debug;
use raylib::prelude::*;

#[derive(Debug, Default)]
pub struct AssetsCache {
    pub textures: RwLock<HashMap<String, Rc<Texture2D>>>,
}
