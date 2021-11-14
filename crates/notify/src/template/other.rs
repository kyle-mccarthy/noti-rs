use super::{private::Sealed, Error, Register, Template, TemplateEngine};

pub trait OtherTemplate: std::any::Any {}

// impl<'a, T: OtherTemplate> Register<T> for TemplateEngine<'a> {}

// impl<T: OtherTemplate> Template for T {
//     fn register(engine: &mut super::TemplateEngine) -> Result<(), Error> {
//         unimplemented!()
//     }
// }
