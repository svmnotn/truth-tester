use truth_tester::{
    parsing::{TokenLiterals, Tokens},
    tester::Tester,
};
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{console, Document, Element, HtmlElement, HtmlTableElement};

mod utils;
use utils::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn init_storage() {
    console::log_1(&JsValue::from_str("Initializing Storage!"));

    fn store() -> Result<(), JsValue> {
        let store = get_storage(&get_window()?)?;

        if is_item(&store, "init")? == false {
            console::log_1(&JsValue::from_str("Storage hadn't been initialized!"));
            // Initialize the store values to their defaults
            let default = TokenLiterals::default();
            fill_storage(&store, &default)?;
            // make sure to store that we've been initialized
            set_item(&store, "init", "true")?;
        }

        Ok(())
    }

    match store() {
        Ok(_) => (),
        Err(v) => console::log_1(&v),
    }
}

fn load(input: &str) -> Result<(Document, Tester<Tokens>, HtmlElement), JsValue> {
    // Get the current window
    let window = get_window()?;
    // obtain the literals configured by the user
    let literals = read_storage(&get_storage(&window)?)?;
    // read the input into a testable expression
    let tester = Tester::parse_with_literals(input, literals);
    // get the current document
    let doc = get_document(&window)?;
    // make sure to remove past tables
    if let Some(t) = doc.get_element_by_id("output-table") {
        t.remove();
    }
    // Make the output table
    let table = doc.create_element("TABLE")?;
    // the code below is fine since we just made sure to make it above
    let table: HtmlTableElement = table.unchecked_into();
    table.set_id("output-table");

    let header = table.create_t_head();
    let h_row = doc.create_element("TR")?;
    for var in tester.vars() {
        let h_col = doc.create_element("TH")?;
        h_col.set_attribute("scope", "col")?;
        h_col.set_text_content(Some(var));
        h_row.append_with_node_1(&h_col)?;
    }
    let h_col = doc.create_element("TH")?;
    h_col.set_attribute("scope", "col")?;
    h_col.set_text_content(Some("Result"));
    h_row.append_with_node_1(&h_col)?;
    header.append_with_node_1(&h_row)?;

    // get the output div
    let out = get_output_elem(&doc)?;
    // add the table to it
    out.append_with_node_1(&table)?;

    Ok((doc, tester, table.create_t_body()))
}

#[wasm_bindgen]
pub fn render_all(input: &str) {
    console::log_2(
        &JsValue::from_str("Writting out the truth table of expr '%s'!"),
        &JsValue::from_str(input),
    );
    let val = load(input);
    if let Err(v) = val {
        console::log_1(&v);
        return;
    }
    let (doc, tester, table) = val.unwrap();
}

#[wasm_bindgen]
pub fn render_successes(input: &str) {
    console::log_2(
        &JsValue::from_str("Writting out the places where expr '%s' is true!"),
        &JsValue::from_str(input),
    );
    let val = load(input);
    if let Err(v) = val {
        console::log_1(&v);
        return;
    }
    let (doc, tester, table) = val.unwrap();
}

#[wasm_bindgen]
pub fn render_failures(input: &str) {
    console::log_2(
        &JsValue::from_str("Writting out the places where expr '%s' is false!"),
        &JsValue::from_str(input),
    );
    let val = load(input);
    if let Err(v) = val {
        console::log_1(&v);
        return;
    }
    let (doc, tester, table) = val.unwrap();
}

#[wasm_bindgen]
pub fn change_value(id: &str, value: &str) {
    console::log_3(
        &JsValue::from_str("Changing the value of '%s' to '%s'!"),
        &JsValue::from_str(id),
        &JsValue::from_str(value),
    );

    fn doit(id: &str, value: &str) -> Result<(), JsValue> {
        let store = get_storage(&get_window()?)?;
        set_item(&store, id, value)?;
        Ok(())
    }

    match doit(id, value) {
        Ok(_) => (),
        Err(v) => console::log_1(&v),
    }
}

#[wasm_bindgen]
pub fn get_value(id: &str) -> String {
    console::log_2(
        &JsValue::from_str("Getting the value of '%s'!"),
        &JsValue::from_str(id),
    );

    fn doit(id: &str) -> Result<String, JsValue> {
        let store = get_storage(&get_window()?)?;
        get_item(&store, id)
    }

    match doit(id) {
        Ok(v) => v,
        Err(v) => {
            console::log_1(&v);
            String::new()
        }
    }
}
