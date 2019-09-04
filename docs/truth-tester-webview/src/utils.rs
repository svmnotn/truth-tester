use truth_tester::parsing::TokenLiterals;
use web_sys::{Document, Element, Storage, Window};

pub(crate) fn get_window() -> Window {
    web_sys::window().expect("No global `window` found in this context!")
}

pub(crate) fn get_document(window: &Window) -> Document {
    window.document().expect("No `document` found in `window`!")
}

pub(crate) fn get_output_elem(document: &Document) -> Element {
    document
        .get_element_by_id("data-output")
        .expect("No Element with id `data-output` found!")
}

pub(crate) fn get_storage(window: &Window) -> Storage {
    window
        .local_storage()
        .expect("Error when trying to obtain `LocalStorage`")
        .expect("No Local Storage found!")
}

fn set_item(store: &Storage, key: &str, value: &str) {
    store.set_item(key, value).expect("Unable to set item!");
}

fn get_item(store: &Storage, key: &str) -> String {
    store
        .get_item(key)
        .expect("Unable to retrive item")
        .expect("Expected item not found")
}

pub(crate) fn update_storage(store: &Storage, literals: &TokenLiterals) {
    set_item(store, "lit-true", &literals.lit_true.join(", "));
    set_item(store, "lit-false", &literals.lit_false.join(", "));
    set_item(store, "not", &literals.not.join(", "));
    set_item(store, "and", &literals.and.join(", "));
    set_item(store, "xor", &literals.xor.join(", "));
    set_item(store, "or", &literals.or.join(", "));
    set_item(store, "implication", &literals.implication.join(", "));
    set_item(store, "equality", &literals.equality.join(", "));
    set_item(store, "left-paren", &literals.left_paren.join(", "));
    set_item(store, "right-paren", &literals.right_paren.join(", "));
}

pub(crate) fn read_storage<'a>(store: &'a Storage) -> TokenLiterals {
    let pat: &'a str = ", ";
    TokenLiterals {
        lit_true: get_item(store, "lit-true")
            .split(pat)
            .map(Into::into)
            .collect(),
        lit_false: get_item(store, "lit-false")
            .split(pat)
            .map(Into::into)
            .collect(),
        not: get_item(store, "not").split(pat).map(Into::into).collect(),
        and: get_item(store, "and").split(pat).map(Into::into).collect(),
        xor: get_item(store, "xor").split(pat).map(Into::into).collect(),
        or: get_item(store, "or").split(pat).map(Into::into).collect(),
        implication: get_item(store, "implication")
            .split(pat)
            .map(Into::into)
            .collect(),
        equality: get_item(store, "equality")
            .split(pat)
            .map(Into::into)
            .collect(),
        left_paren: get_item(store, "left-paren")
            .split(pat)
            .map(Into::into)
            .collect(),
        right_paren: get_item(store, "right-paren")
            .split(pat)
            .map(Into::into)
            .collect(),
    }
}