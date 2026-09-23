use std::ffi::{CString, c_char};

use crate::controls::{CairoOpaque, ContextPDF, DrawExtPDF};

#[repr(C)]
#[derive(Clone)]
pub enum FontSlant {
    Normal, //C order items
    Static,
    Oblique,
}

#[repr(C)]
#[derive(Clone)]
pub enum FontWeight {
    Normal,
    Bold,
}

#[repr(C)]
pub struct TextOpaque {
    _private: [u8; 0],
}

unsafe extern "C" {

    unsafe fn text_create(
        label: *const c_char,
        font_famyly: *const c_char,
        font_size: f64,
        font_slant: FontSlant,
        font_weight: FontWeight,
        w: f64,
        h: f64,
        x: f64,
        y: f64,
    ) -> *mut TextOpaque;

    unsafe fn text_draw(text: *mut TextOpaque, ctx: *mut CairoOpaque);

    unsafe fn text_free(text: *mut TextOpaque);
}


pub struct TextPDF {
    ptr: *mut TextOpaque,
    label: String,
    font_famyly: String,
    font_size: f64,
    font_slant: FontSlant,
    font_weight: FontWeight,
    w: f64,
    h: f64,
    x: f64,
    y: f64,
}

impl TextPDF {
    pub fn new(
        label: String,
        font_famyly: String,
        font_size: f64,
        font_slant: FontSlant,
        font_weight: FontWeight,
        w: f64,
        h: f64,
        x: f64,
        y: f64,
    ) -> Option<Self> {
        let _ptr = unsafe {
            let _label = CString::new(label.to_string()).unwrap();
            let _font_family = CString::new(font_famyly.to_string()).unwrap();
            text_create(
                _label.as_ptr(),
                _font_family.as_ptr(),
                font_size,
                font_slant.clone(),
                font_weight.clone(),
                w,
                h,
                x,
                y,
            )
        };

        if _ptr.is_null() {
            None
        } else {
            Some(Self {
                ptr: _ptr,
                label,
                font_famyly,
                font_size,
                font_slant,
                font_weight,
                w,
                h,
                x,
                y,
            })
        }
    }
}

impl DrawExtPDF for TextPDF {
    fn draw(&self, ctx: Option<ContextPDF>) {
        if let Some(_ctx) = ctx {
            unsafe {
                text_draw(self.ptr, _ctx.ptr);
            }
        }
    }
}

impl Drop for TextPDF {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                text_free(self.ptr);
            }
        }
    }
}