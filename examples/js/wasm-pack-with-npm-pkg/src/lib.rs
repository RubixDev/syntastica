use wasm_bindgen::prelude::*;

// import the required functions from the NPM package
#[wasm_bindgen(raw_module = "syntastica")]
extern "C" {
    async fn init(langs: Vec<JsValue>) -> JsValue;
    fn highlight(code: &str, language: &str, theme: &str) -> String;
}

// run this code when the module is loaded
#[wasm_bindgen(start)]
async fn run() -> Result<(), JsValue> {
    // initialize syntastica with Rust support
    init(vec![JsValue::from_str("rust")]).await;

    // get global objects
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let code_div = document.create_element("div")?;
    code_div.set_class_name("code");
    code_div.set_inner_html(&highlight(
        "fn main() {\n    println!(\"Hello, World!\");\n}",
        "rust",
        "one::dark",
    ));
    body.append_child(&code_div)?;

    Ok(())
}
