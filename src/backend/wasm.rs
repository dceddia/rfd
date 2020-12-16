use crate::DialogOptions;
use std::path::PathBuf;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Document, Element};

use web_sys::{HtmlButtonElement, HtmlInputElement};

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

pub struct FileHandle(web_sys::File);

impl FileHandle {
    pub async fn read(self) -> Vec<u8> {
        let promise = js_sys::Promise::new(&mut move |res, _rej| {
            let file_reader = web_sys::FileReader::new().unwrap();

            let fr = file_reader.clone();
            let closure = Closure::wrap(Box::new(move || {
                res.call1(&JsValue::undefined(), &fr.result().unwrap())
                    .unwrap();
            }) as Box<dyn FnMut()>);

            file_reader.set_onload(Some(closure.as_ref().unchecked_ref()));

            closure.forget();

            file_reader.read_as_array_buffer(&self.0).unwrap();
        });

        let future = wasm_bindgen_futures::JsFuture::from(promise);

        let res = future.await.unwrap();

        let buffer: js_sys::Uint8Array = js_sys::Uint8Array::new(&res);
        let mut vec = vec![0; buffer.length() as usize];
        buffer.copy_to(&mut vec[..]);

        vec
    }

    pub fn web_sys_file(&self) -> web_sys::File {
        self.0.clone()
    }
}

pub struct Dialog {
    overlay: Element,
    card: Element,
    input: HtmlInputElement,
    button: HtmlButtonElement,

    style: Element,
}

impl Dialog {
    pub fn new(document: &Document) -> Self {
        let overlay = document.create_element("div").unwrap();
        overlay.set_id("rfd-overlay");

        let card = {
            let card = document.create_element("div").unwrap();
            card.set_id("rfd-card");
            overlay.append_child(&card).unwrap();

            card
        };

        let input = {
            let input_el = document.create_element("input").unwrap();
            let input: HtmlInputElement = wasm_bindgen::JsCast::dyn_into(input_el).unwrap();

            input.set_id("rfd-input");
            input.set_type("file");

            card.append_child(&input).unwrap();
            input
        };

        let button = {
            let btn_el = document.create_element("button").unwrap();
            let btn: HtmlButtonElement = wasm_bindgen::JsCast::dyn_into(btn_el).unwrap();

            btn.set_id("rfd-button");
            btn.set_inner_text("Ok");

            card.append_child(&btn).unwrap();
            btn
        };

        let style = document.create_element("style").unwrap();
        style.set_inner_html(include_str!("./wasm/style.css"));
        overlay.append_child(&style).unwrap();

        Self {
            overlay,
            card,
            button,
            input,

            style,
        }
    }

    pub async fn open(&mut self, body: &Element) -> Vec<FileHandle> {
        let overlay = self.overlay.clone();
        let button = self.button.clone();

        let promise = js_sys::Promise::new(&mut move |res, _rej| {
            let closure = Closure::wrap(Box::new(move || {
                res.call0(&JsValue::undefined()).unwrap();
            }) as Box<dyn FnMut()>);

            button.set_onclick(Some(closure.as_ref().unchecked_ref()));
            closure.forget();
            body.append_child(&overlay).ok();
        });
        let future = wasm_bindgen_futures::JsFuture::from(promise);
        future.await.unwrap();

        let mut file_handles = Vec::new();

        if let Some(files) = self.input.files() {
            for id in 0..(files.length()) {
                let file = files.get(id).unwrap();
                file_handles.push(FileHandle(file));
            }
        }

        self.overlay.remove();
        file_handles
    }
}

impl Drop for Dialog {
    fn drop(&mut self) {
        self.button.remove();
        self.input.remove();
        self.card.remove();

        self.style.remove();
        self.overlay.remove();
    }
}

pub fn pick_file<'a>(params: impl Into<Option<DialogOptions<'a>>>) -> Option<PathBuf> {
    let params = params.into().unwrap_or_default();

    None
}

pub fn save_file<'a>(params: impl Into<Option<DialogOptions<'a>>>) -> Option<PathBuf> {
    let params = params.into().unwrap_or_default();

    None
}

pub fn pick_folder<'a>(params: impl Into<Option<DialogOptions<'a>>>) -> Option<PathBuf> {
    let params = params.into().unwrap_or_default();

    None
}

pub fn pick_files<'a>(params: impl Into<Option<DialogOptions<'a>>>) -> Option<Vec<PathBuf>> {
    let params = params.into().unwrap_or_default();

    None
}