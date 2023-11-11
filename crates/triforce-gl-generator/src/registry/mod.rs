use crate::parser::Parser;

pub const GL_XML: &'static [u8] = include_bytes!("../documents/gl.xml");

pub struct Registry {

}

impl Registry {
  pub fn new() -> Self {
    let path = GL_XML;

    let parsed = Parser::from_xml(path);

    Self {
      
    }
  }
}
