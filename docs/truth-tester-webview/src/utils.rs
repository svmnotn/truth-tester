use truth_tester::parsing::TokenLiterals;
use wasm_bindgen::prelude::*;
use web_sys::{Document, Element, Storage, Window};

pub(crate) fn get_window() -> Result<Window, JsValue> {
    web_sys::window().ok_or_else(|| JsValue::from_str("No Window Found!"))
}

pub(crate) fn get_document(window: &Window) -> Result<Document, JsValue> {
    window
        .document()
        .ok_or_else(|| JsValue::from_str("No Local Storage Found!"))
}

pub(crate) fn get_output_elem(document: &Document) -> Result<Element, JsValue> {
    document
        .get_element_by_id("data-output")
        .ok_or_else(|| JsValue::from_str("data-output div not found!"))
}

pub(crate) fn get_storage(window: &Window) -> Result<Storage, JsValue> {
    window
        .local_storage()?
        .ok_or_else(|| JsValue::from_str("No Local Storage Found!"))
}

pub(crate) fn set_item(store: &Storage, key: &str, value: &str) -> Result<(), JsValue> {
    store.set_item(key, value)?;
    Ok(())
}

pub(crate) fn is_item(store: &Storage, key: &str) -> Result<bool, JsValue> {
    Ok(store.get_item(key)?.is_some())
}

pub(crate) fn get_item(store: &Storage, key: &str) -> Result<String, JsValue> {
    store
        .get_item(key)?
        .ok_or_else(|| JsValue::from_str("Expected item not found"))
}

const PAT: &'static str = ", ";

pub(crate) fn fill_storage(store: &Storage, literals: &TokenLiterals) -> Result<(), JsValue> {
    set_item(store, "lit-true", &literals.lit_true.join(PAT))?;
    set_item(store, "lit-false", &literals.lit_false.join(PAT))?;
    set_item(store, "not", &literals.not.join(PAT))?;
    set_item(store, "and", &literals.and.join(PAT))?;
    set_item(store, "xor", &literals.xor.join(PAT))?;
    set_item(store, "or", &literals.or.join(PAT))?;
    set_item(store, "implication", &literals.implication.join(PAT))?;
    set_item(store, "equality", &literals.equality.join(PAT))?;
    set_item(store, "left-paren", &literals.left_paren.join(PAT))?;
    set_item(store, "right-paren", &literals.right_paren.join(PAT))?;

    Ok(())
}

pub(crate) fn read_storage(store: &Storage) -> Result<TokenLiterals, JsValue> {
    Ok(TokenLiterals {
        lit_true: get_item(store, "lit-true")?
            .split(PAT)
            .map(Into::into)
            .collect(),
        lit_false: get_item(store, "lit-false")?
            .split(PAT)
            .map(Into::into)
            .collect(),
        not: get_item(store, "not")?.split(PAT).map(Into::into).collect(),
        and: get_item(store, "and")?.split(PAT).map(Into::into).collect(),
        xor: get_item(store, "xor")?.split(PAT).map(Into::into).collect(),
        or: get_item(store, "or")?.split(PAT).map(Into::into).collect(),
        implication: get_item(store, "implication")?
            .split(PAT)
            .map(Into::into)
            .collect(),
        equality: get_item(store, "equality")?
            .split(PAT)
            .map(Into::into)
            .collect(),
        left_paren: get_item(store, "left-paren")?
            .split(PAT)
            .map(Into::into)
            .collect(),
        right_paren: get_item(store, "right-paren")?
            .split(PAT)
            .map(Into::into)
            .collect(),
    })
}
