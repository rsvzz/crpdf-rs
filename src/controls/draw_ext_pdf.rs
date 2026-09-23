use crate::controls::ContextPDF;

pub trait DrawExtPDF {
    fn draw(&self, ctx: Option<ContextPDF>);
}