use crate::controls::{CairoOpaque, ContextPDF, DrawExtPDF};

#[repr(C)]
pub struct LineOpaque {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Clone)]
pub enum AxisPDF {
    HORIZONTAL,
    VERTICAL,
}

unsafe extern "C" {
    unsafe fn line_create(
        stickless: f64,
        orientation: AxisPDF,
        w: f64,
        h: f64,
        x: f64,
        y: f64,
    ) -> *mut LineOpaque;
    unsafe fn line_draw(line: *mut LineOpaque, ctx: *mut CairoOpaque);

    unsafe fn line_free(line: *mut LineOpaque);
}
pub struct LinePDF {
    ptr: *mut LineOpaque,
}

impl LinePDF {
    pub fn new(_tickless: f64, _orientation: AxisPDF, w: f64, h: f64, x: f64, y: f64) -> Option<Self>{
          let _ptr = unsafe {
        
            line_create(
                _tickless,
                _orientation,
                w,
                h,
                x,
                y,
            )
        };

        if _ptr.is_null(){
            None
        }
        else{
            Some(Self { ptr: _ptr })
        }
    }
}

impl DrawExtPDF for LinePDF {
    fn draw(&self, ctx: Option<ContextPDF>) {
        if let Some(_ctx) = ctx {
            unsafe {
                line_draw(self.ptr, _ctx.ptr);
            }
        }
    }
}


impl Drop for LinePDF {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                line_free(self.ptr);
            }
        }
    }
}
