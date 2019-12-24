#![recursion_limit="256"]

mod yuumu;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn go() -> Result<(), JsValue> {
    yew::initialize();
    yew::start_app::<yuumu::Index>();
    Ok(())
}
