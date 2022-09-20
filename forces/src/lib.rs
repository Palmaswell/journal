use std::f64;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d};
use js_sys::{Uint32Array};

struct State {
    width: u32,
    height: u32,
    coordinates: Vec<i32>,
    data_ptr: *mut u32,
    data_len: usize,
    data_capacity: usize,
}


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
pub fn get_coordinates(ctx: &CanvasRenderingContext2d) -> Uint32Array {
    
    ctx.begin_path();

    // Draw the outer circle.
    ctx
        .arc(450.0, 250.0, 80.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();
    ctx.set_fill_style(&"rgb(247, 37, 133)".into());
    ctx.fill();
    ctx.stroke();
    ctx.close_path();


    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);

    Uint32Array::from(&vec[..])
}
