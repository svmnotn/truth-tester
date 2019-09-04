use truth_tester::parsing::TokenLiterals;
use wasm_bindgen::prelude::*;

mod utils;
use utils::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn init_storage() {
    web_sys::console::log_1(&JsValue::from_str("Initializing Storage!"));

    let store = get_storage(&get_window());

    if is_item(&store, "init") == false {
        web_sys::console::log_1(&JsValue::from_str("Storage hadn't been initialized!"));
        // Initialize the store values to their defaults
        let default = TokenLiterals::default();
        fill_storage(&store, &default);
        // make sure to store that we've been initialized
        set_item(&store, "init", "true");
    }
}

#[wasm_bindgen]
pub fn render_all(input: &str) {
    web_sys::console::log_2(
        &JsValue::from_str("Writting out the truth table of expr '%s'!"),
        &JsValue::from_str(input),
    );
}

#[wasm_bindgen]
pub fn render_successes(input: &str) {
    web_sys::console::log_2(
        &JsValue::from_str("Writting out the places where expr '%s' is true!"),
        &JsValue::from_str(input),
    );
}

#[wasm_bindgen]
pub fn render_failures(input: &str) {
    web_sys::console::log_2(
        &JsValue::from_str("Writting out the places where expr '%s' is false!"),
        &JsValue::from_str(input),
    );
}

#[wasm_bindgen]
pub fn change_value(id: &str, value: &str) {
    web_sys::console::log_3(
        &JsValue::from_str("Changing the value of '%s' to '%s'!"),
        &JsValue::from_str(id),
        &JsValue::from_str(value),
    );
    let store = get_storage(&get_window());
    set_item(&store, id, value);
}

#[wasm_bindgen]
pub fn get_value(id: &str) -> String {
    web_sys::console::log_2(
        &JsValue::from_str("Getting the value of '%s'!"),
        &JsValue::from_str(id),
    );
    let store = get_storage(&get_window());
    get_item(&store, id)
}
