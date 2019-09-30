use truth_tester::parsing::TokenLiterals;
use wasm_bindgen::prelude::*;
use web_sys::console;

mod utils;
use utils::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn init_storage() {
    console::log_1(&JsValue::from_str("Initializing Storage!"));

    #[inline]
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

#[wasm_bindgen]
pub fn render_all(input: &str) {
    console::log_2(
        &JsValue::from_str("Writting out the truth table of expr '%s'!"),
        &JsValue::from_str(input),
    );

    #[inline]
    fn doit(input: &str) -> Result<(), JsValue> {
        let (doc, tester, body) = load(input)?;
        for (state, res) in tester.eval() {
            render_data(&doc, &body, &tester, state, &res.to_string())?;
        }
        Ok(())
    }

    match doit(input) {
        Ok(_) => (),
        Err(v) => console::log_1(&v),
    }
}

#[wasm_bindgen]
pub fn render_successes(input: &str) {
    console::log_2(
        &JsValue::from_str("Writting out the places where expr '%s' is true!"),
        &JsValue::from_str(input),
    );

    #[inline]
    fn doit(input: &str) -> Result<(), JsValue> {
        let (doc, tester, body) = load(input)?;
        for state in tester.successes() {
            render_data(&doc, &body, &tester, state, "true")?;
        }
        Ok(())
    }

    match doit(input) {
        Ok(_) => (),
        Err(v) => console::log_1(&v),
    }
}

#[wasm_bindgen]
pub fn render_failures(input: &str) {
    console::log_2(
        &JsValue::from_str("Writting out the places where expr '%s' is false!"),
        &JsValue::from_str(input),
    );

    #[inline]
    fn doit(input: &str) -> Result<(), JsValue> {
        let (doc, tester, body) = load(input)?;
        for state in tester.failures() {
            render_data(&doc, &body, &tester, state, "false")?;
        }
        Ok(())
    }

    match doit(input) {
        Ok(_) => (),
        Err(v) => console::log_1(&v),
    }
}

#[wasm_bindgen]
pub fn change_value(id: &str, value: &str) {
    console::log_3(
        &JsValue::from_str("Changing the value of '%s' to '%s'!"),
        &JsValue::from_str(id),
        &JsValue::from_str(value),
    );

    #[inline]
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

    #[inline]
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
