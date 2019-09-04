// Here should go the WASM module code
import init, {init_storage, render_all, render_failures, render_successes} from './webview.js';

async function initialize() {
    // Initialize WASM
    await init();
    // Initialize the Storage
    init_storage();
    // set all the buttons to do their thing
    const text = document.getElementById("function");
    document.getElementById("get-all").onclick = function () { render_all(text.nodeValue); };
    document.getElementById("get-success").onclick = function () { render_successes(text.nodeValue); };
    document.getElementById("get-failures").onclick = function () { render_failures(text.nodeValue); };
    /*
    document.getElementById("lit-true").oninput;
    document.getElementById("lit-false").oninput;
    document.getElementById("not").oninput;
    document.getElementById("and").oninput;
    document.getElementById("xor").oninput;
    document.getElementById("or").oninput;
    document.getElementById("implication").oninput;
    document.getElementById("equality").oninput;
    document.getElementById("left-paren").oninput;
    document.getElementById("right-paren").oninput;*/
}

initialize();