use truth_tester::parsing::TokenLiterals;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{HtmlTableElement, console};

mod utils;
use utils::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn init_storage() {
    console::log_1(&JsValue::from_str("Initializing Storage!"));

    let store = get_storage(&get_window());

    if is_item(&store, "init") == false {
        console::log_1(&JsValue::from_str("Storage hadn't been initialized!"));
        // Initialize the store values to their defaults
        let default = TokenLiterals::default();
        fill_storage(&store, &default);
        // make sure to store that we've been initialized
        set_item(&store, "init", "true");
    }
}

#[wasm_bindgen]
pub fn render_all(input: &str) {
    console::log_2(
        &JsValue::from_str("Writting out the truth table of expr '%s'!"),
        &JsValue::from_str(input),
    );

    let window = get_window();
    let literals = read_storage(&get_storage(&window));
    let doc = get_document(&window);
    let out = get_output_elem(&doc);

    let table = doc.create_element("TABLE").expect("Unable to make Table!");
    // the code below is fine since we just made sure to make it above
    let table: HtmlTableElement = table.unchecked_into();
    table.set_id("output-table");
    out.append_with_node_1(&table).expect("Unable to append output table!");
}

#[wasm_bindgen]
pub fn render_successes(input: &str) {
    console::log_2(
        &JsValue::from_str("Writting out the places where expr '%s' is true!"),
        &JsValue::from_str(input),
    );

    let window = get_window();
    let literals = read_storage(&get_storage(&window));
    let out = get_output_elem(&get_document(&window));
    out.set_inner_html("Output should go here! With a nice table of all the times where the expression is true");
}

#[wasm_bindgen]
pub fn render_failures(input: &str) {
    console::log_2(
        &JsValue::from_str("Writting out the places where expr '%s' is false!"),
        &JsValue::from_str(input),
    );

    let window = get_window();
    let literals = read_storage(&get_storage(&window));
    let out = get_output_elem(&get_document(&window));
    out.set_inner_html("Output should go here! With a nice table of all the times where the expression is false");
}

#[wasm_bindgen]
pub fn change_value(id: &str, value: &str) {
    console::log_3(
        &JsValue::from_str("Changing the value of '%s' to '%s'!"),
        &JsValue::from_str(id),
        &JsValue::from_str(value),
    );
    let store = get_storage(&get_window());
    set_item(&store, id, value);
}

#[wasm_bindgen]
pub fn get_value(id: &str) -> String {
    console::log_2(
        &JsValue::from_str("Getting the value of '%s'!"),
        &JsValue::from_str(id),
    );
    let store = get_storage(&get_window());
    get_item(&store, id)
}
