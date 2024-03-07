use wasm_bindgen::prelude::*;
use wasm_react::{h, import_components, Component, VNode};

import_components! {
  #[wasm_bindgen(raw_module = "../App.js")]
  MyComponent
}

struct App;

impl Component for App {
    fn render(&self) -> VNode {
        MyComponent::new().build(())
        // h!(div).build((MyComponent::new()
        //     .attr("prop", &"Hello World!".into())
        //     .build(()),))
    }
}

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[wasm_bindgen]
pub fn render() -> JsValue {
    App.render().as_ref().clone()
}
