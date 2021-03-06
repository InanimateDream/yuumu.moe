#![recursion_limit="1024"]

mod yuumu;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn go() -> Result<(), JsValue> {
    yew::initialize();
    yew::start_app::<yuumu::App>();
    Ok(())
}
