use js_sys::{Array, Function, Object};
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{HtmlElement, KeyboardEvent, MouseEvent, Worker};
type HTMLElement = HtmlElement;
include!("monaco_extern.rs");
include!("monaco_help.rs");