use std::{ffi::{CString, c_char}};

#[derive(Clone)]
pub struct  ContextPDF{
    pub ptr: *mut CairoOpaque,
}

#[repr(C)]
pub struct CreatePDFOpaque {
    _private: [u8; 0],
}

#[repr(C)]
pub struct CairoOpaque{
    _private: [u8; 0],
}

unsafe extern "C" {
    unsafe fn crpdf_create(path: *const c_char, width: f64, height: f64) -> *mut CreatePDFOpaque;
    unsafe fn crpdf_free(text: *mut CreatePDFOpaque);
    unsafe fn crpdf_get_context(pdf: *mut CreatePDFOpaque) -> *mut CairoOpaque;
    unsafe fn crpdf_cairo_surface_free(pdf: *mut CreatePDFOpaque);
}

pub struct CreatePDF {
    ptr: *mut CreatePDFOpaque,
    path: String,
    page_width: f64,
    page_height: f64,
    context: Option<ContextPDF>,
}

impl CreatePDF {
    pub fn new(dir: String, width: f64, height: f64) -> Option<Self> {
        let _ptr = unsafe {
            let _path = CString::new(dir.to_string()).unwrap();
            crpdf_create(_path.as_ptr(), width, height)
        };
        if _ptr.is_null() {
            None
        } else {
            Some(Self{
                ptr: _ptr,
                path: dir,
                page_width: width,
                page_height: height,
                context: None,
            })
        }
    }

    pub fn get_context(&self) -> Option<ContextPDF>{
        if let Some(ctx) = self.context.clone() {
            Some(ctx)
        }else {
            let _ptr_context = unsafe {
                crpdf_get_context(self.ptr)
            };

            if _ptr_context.is_null(){
                None
            }
            else {
                Some(ContextPDF{
                    ptr: _ptr_context,
                })
            }
        }
        
    }
}

impl Drop for CreatePDF {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                crpdf_cairo_surface_free(self.ptr);
                crpdf_free(self.ptr);
            }
        }
    }
}