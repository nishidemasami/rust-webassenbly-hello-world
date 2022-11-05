use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[test]
fn it_works() {
    let result = greet("World");
    assert_eq!(result, "Hello, World!");
}
