use wasm_bindgen::prelude::*;
use truth_tester::parsing::TokenLiterals;

mod utils;
use utils::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn init_storage() {
    let store = get_storage(&get_window());

    if store
        .get_item("init")
        .expect("Could not get any Items from the storage")
        .is_none()
    {
        // Initialize the store values to their defaults
        let default = TokenLiterals::default();
        update_storage(&store, &default);
        // make sure to store that we've been initialized
        store.set_item("init", "true").expect("Unable to set item in our storage!");
    }
}

#[wasm_bindgen]
pub fn render_all(input: &str) {
    unimplemented!()
}

#[wasm_bindgen]
pub fn render_successes(input: &str) {
    unimplemented!()
}

#[wasm_bindgen]
pub fn render_failures(input: &str) {
    unimplemented!()
}
