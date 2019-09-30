use truth_tester::{
    eval::{State, Tester},
    parsing::{TokenLiterals, Tokens},
};
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{Document, Element, HtmlElement, HtmlTableElement, Storage, Window};

pub(crate) fn load(input: &str) -> Result<(Document, Tester<Tokens>, HtmlElement), JsValue> {
    // Get the current window
    let window = get_window()?;
    // obtain the literals configured by the user
    let literals = read_storage(&get_storage(&window)?)?;
    // read the input into a testable expression
    let tester = Tester::parse_with_literals(input, literals);
    // get the current document
    let doc = get_document(&window)?;

    //
    // Make the Table
    //

    // make sure to remove past tables
    if let Some(t) = doc.get_element_by_id("output-table") {
        t.remove();
    }
    // Make the output table
    let table = doc.create_element("TABLE")?;
    // the code below is fine since we just made sure to make it above
    let table: HtmlTableElement = table.unchecked_into();
    table.set_id("output-table");

    //
    // Render the table header
    //

    // Create the actual header element
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
    // add everything we just made
    // to the header
    header.append_with_node_1(&h_row)?;

    //
    // Append the table to the document
    //

    // get the output div
    let out = get_output_elem(&doc)?;
    // add the table to it
    out.append_with_node_1(&table)?;

    Ok((doc, tester, table.create_t_body()))
}

pub(crate) fn render_data<S: State>(
    doc: &Document,
    body: &HtmlElement,
    tester: &Tester<Tokens>,
    state: S,
    res: &str,
) -> Result<(), JsValue> {
    // Create the row for this data
    let row = doc.create_element("TR")?;
    for val in tester.var_vals(state) {
        let col = doc.create_element("TD")?;
        col.set_text_content(Some(&val.to_string()));
        row.append_with_node_1(&col)?;
    }
    let col = doc.create_element("TD")?;
    col.set_text_content(Some(res));
    // append everything to the row
    row.append_with_node_1(&col)?;
    // and the row to the table
    body.append_with_node_1(&row)?;

    Ok(())
}

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

pub(crate) fn fill_storage(store: &Storage, literals: &TokenLiterals) -> Result<(), JsValue> {
    const PAT: &'static str = ", ";

    set_item(store, "lit-true", &literals.lit_true().join(PAT))?;
    set_item(store, "lit-false", &literals.lit_false().join(PAT))?;
    set_item(store, "not", &literals.not().join(PAT))?;
    set_item(store, "and", &literals.and().join(PAT))?;
    set_item(store, "xor", &literals.xor().join(PAT))?;
    set_item(store, "or", &literals.or().join(PAT))?;
    set_item(store, "implication", &literals.implication().join(PAT))?;
    set_item(store, "equality", &literals.equality().join(PAT))?;
    set_item(store, "left-paren", &literals.left_paren().join(PAT))?;
    set_item(store, "right-paren", &literals.right_paren().join(PAT))?;

    Ok(())
}

pub(crate) fn read_storage(store: &Storage) -> Result<TokenLiterals, JsValue> {
    const PAT: &'static str = ",";

    let mut lit = TokenLiterals::default();
    lit.set_lit_true(
        get_item(store, "lit-true")?
            .split(PAT)
            .map(|v| v.trim())
            .map(Into::into)
            .collect(),
    );
    lit.set_lit_false(
        get_item(store, "lit-false")?
            .split(PAT)
            .map(|v| v.trim())
            .map(Into::into)
            .collect(),
    );
    lit.set_not(
        get_item(store, "not")?
            .split(PAT)
            .map(|v| v.trim())
            .map(Into::into)
            .collect(),
    );
    lit.set_and(
        get_item(store, "and")?
            .split(PAT)
            .map(|v| v.trim())
            .map(Into::into)
            .collect(),
    );
    lit.set_xor(
        get_item(store, "xor")?
            .split(PAT)
            .map(|v| v.trim())
            .map(Into::into)
            .collect(),
    );
    lit.set_or(
        get_item(store, "or")?
            .split(PAT)
            .map(|v| v.trim())
            .map(Into::into)
            .collect(),
    );
    lit.set_implication(
        get_item(store, "implication")?
            .split(PAT)
            .map(|v| v.trim())
            .map(Into::into)
            .collect(),
    );
    lit.set_equality(
        get_item(store, "equality")?
            .split(PAT)
            .map(|v| v.trim())
            .map(Into::into)
            .collect(),
    );
    lit.set_left_paren(
        get_item(store, "left-paren")?
            .split(PAT)
            .map(|v| v.trim())
            .map(Into::into)
            .collect(),
    );
    lit.set_right_paren(
        get_item(store, "right-paren")?
            .split(PAT)
            .map(|v| v.trim())
            .map(Into::into)
            .collect(),
    );
    Ok(lit)
}
