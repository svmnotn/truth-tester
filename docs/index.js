function toggle(id) {
    var style = document.getElementById(id).style;
    style.display = style.display === "block" ? "none" : "block";
}
// Load WASM, set the buttons to do what is expected of them