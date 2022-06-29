use std::f64;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d};

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    // Manufacture the element we're gonna append
    let val = document.create_element("p")?;
    val.set_inner_html("Hello from Rust!");

    body.append_child(&val)?;

    Ok(())
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(ctx: &CanvasRenderingContext2d, msg: &str);
}

#[wasm_bindgen]
pub fn draw(ctx: &CanvasRenderingContext2d) {
    
    ctx.begin_path();

    // Draw the outer circle.
    ctx
        .arc(250.0, 250.0, 70.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();
    ctx.set_fill_style(&"rgb(247, 37, 133)".into());
    ctx.fill();
    ctx.stroke();
    ctx.close_path();

    log(ctx, "hellow world ???????????????222444");
}

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}



