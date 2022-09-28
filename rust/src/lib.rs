extern crate cfg_if;
extern crate wasm_bindgen;
use std::cell::Cell;
use std::rc::Rc;
use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;
use crate::wasm_bindgen::JsCast;

cfg_if! {
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    let colors = vec!["#F4908E", "#F2F097", "#88B0DC", "#F7B5D1", "#53C4AF", "#FDE38C"];
    let window = web_sys::window().expect("should have a window in this context");
    let document = window.document().expect("window should have a document");
    
    let canvas = document
        .create_element("canvas").expect("should create canvas")
        .dyn_into::<web_sys::HtmlCanvasElement>().expect("should cast to canvas");

    document.body().unwrap().append_child(&canvas).expect("should append canvas");

    canvas.set_width(640);
    canvas.set_height(480);
    canvas.style().set_property("border", "solid").expect("should set border");
    let context = canvas
    .get_context("2d").expect("should get context")
    .unwrap()
    .dyn_into::<web_sys::CanvasRenderingContext2d>().expect("should cast to context");

    let context = Rc::new(context);
    let pressed = Rc::new(Cell::new(false));

    { mouse_down(&context, &pressed, &canvas); }
    { mouse_move(&context, &pressed, &canvas); }
    { mouse_up(&context, &pressed, &canvas); }

    // Create divs for color picker
    for c in colors {
        let div = document
            .create_element("div").expect("should create div")
            .dyn_into::<web_sys::HtmlElement>().expect("should cast to div");
        div.set_class_name("color");
        {
            click(&context, &div, c.clone());
        }
        
        div.style().set_property("background-color", c).expect("should set background color");
        div.style().set_property("width", "50px").expect("should set width");
        div.style().set_property("height", "50px").expect("should set height");
        let div = div.dyn_into::<web_sys::Node>().expect("should cast to node");
        document.body().unwrap().append_child(&div).expect("should append div");
    }
    println!("{}", name);
    // alert(&format!("Hello,{}!", name));
}


fn click(context: &std::rc::Rc<web_sys::CanvasRenderingContext2d>, div: &web_sys::HtmlElement, c: &str) {
    let context = context.clone();
    let c = JsValue::from(String::from(c));
    let closure = Closure::wrap(Box::new(move || {
        context.set_stroke_style(&c);            
    }) as Box<dyn FnMut()>);

    div.set_onclick(Some(closure.as_ref().unchecked_ref()));
    closure.forget();
}


fn mouse_up(context: &std::rc::Rc<web_sys::CanvasRenderingContext2d>, pressed: &std::rc::Rc<std::cell::Cell<bool>>, canvas: &web_sys::HtmlCanvasElement) {
    let context = context.clone();
    let pressed = pressed.clone();
    let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
        pressed.set(false);
        context.line_to(event.offset_x() as f64, event.offset_y() as f64);
        context.stroke();
    }) as Box<dyn FnMut(_)>);
    canvas.add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref()).unwrap();
    closure.forget();
}

fn mouse_move(context: &std::rc::Rc<web_sys::CanvasRenderingContext2d>, pressed: &std::rc::Rc<std::cell::Cell<bool>>, canvas: &web_sys::HtmlCanvasElement){
    let context = context.clone();
    let pressed = pressed.clone();
    let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
        if pressed.get() {
            context.line_to(event.offset_x() as f64, event.offset_y() as f64);
            context.stroke();
            context.begin_path();
            context.move_to(event.offset_x() as f64, event.offset_y() as f64);
        }
    }) as Box<dyn FnMut(_)>);
    canvas.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref()).unwrap();
    closure.forget();
}


fn mouse_down(context: &std::rc::Rc<web_sys::CanvasRenderingContext2d>, pressed: &std::rc::Rc<std::cell::Cell<bool>>, canvas: &web_sys::HtmlCanvasElement){
    let context = context.clone();
    let pressed = pressed.clone();

    let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
        context.begin_path();
        context.set_line_width(5.0);
        context.move_to(event.offset_x() as f64, event.offset_y() as f64);
        pressed.set(true);
    }) as Box<dyn FnMut(_)>);
    canvas.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref()).unwrap();
    closure.forget();
}