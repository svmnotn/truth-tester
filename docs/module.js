// Here should go the WASM module code
import init, {init_storage, render_all, render_failures, render_successes, change_value, get_value} from './webview.js';

function initButtons() {
    const text = document.getElementById("function");
    document.getElementById("get-all").onclick = function () { render_all(text.value); };
    document.getElementById("get-success").onclick = function () { render_successes(text.value); };
    document.getElementById("get-failures").onclick = function () { render_failures(text.value); };
}

function setupOptionField(id) {
    // obtain the element
    const elem = document.getElementById(id);
    // set its initial value
    elem.value = get_value(id);
    // make sure that when it changes, it alters the value
    elem.oninput = function () {
        change_value(id, elem.value);
    }
}

async function initialize() {
    // Initialize WASM
    await init();
    
    // Initialize the Storage
    init_storage();
    // set all the buttons to do their thing
    initButtons();

    // set up all the option fields
    setupOptionField("lit-true");
    setupOptionField("lit-false");
    setupOptionField("not");
    setupOptionField("and");
    setupOptionField("xor");
    setupOptionField("or");
    setupOptionField("implication");
    setupOptionField("equality");
    setupOptionField("left-paren");
    setupOptionField("right-paren");
}

initialize();