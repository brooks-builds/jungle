use js_sys::Function;
use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::*;

#[wasm_bindgen]
extern "C" {}

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    // #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub struct GameState {
    image_url: String,
}

#[wasm_bindgen]
impl GameState {
    pub fn setup() -> Self {
        GameState {
            image_url: "./KawaiiIcons_NoBG037.png".to_owned(),
        }
    }

    pub fn get_image_url(&self) -> String {
        self.image_url.clone()
    }
}
