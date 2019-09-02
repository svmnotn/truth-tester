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

pub(crate) fn update_storage(store: &Storage, literals: &TokenLiterals) {
    unimplemented!()
}

pub(crate) fn read_storage(store: &Storage) -> TokenLiterals {
    unimplemented!()
}
