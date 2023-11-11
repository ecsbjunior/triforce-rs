use std::ffi::c_void;

use triforce_window::window::Window;

use crate::platform;

#[derive(Debug)]
pub struct Render {
  pub context: platform::Render,
}

impl Render {
  pub fn make_current(&self) {
    self.context.make_current()
  }

  pub fn swap_buffers(&self) {
    self.context.swap_buffers()
  }

  pub fn get_proc_address(&self, name: &str) -> *const c_void {
    self.context.get_proc_address(name)
  }
}

#[derive(Debug)]
pub struct RenderAttributes {
  pub version: (u8, u8),
  pub color_bits: u8,
  pub depth_bits: u8,
}

impl Default for RenderAttributes {
  fn default() -> Self {
    Self {
      version: (4, 6),
      color_bits: 32,
      depth_bits: 24,
    }
  }
}

#[derive(Debug, Default)]
pub struct RenderBuilder {
  pub attributes: RenderAttributes,
}

impl RenderBuilder {
  pub fn new() -> Self {
    Default::default()
  }

  pub fn with_version(mut self, version: (u8, u8)) -> Self {
    self.attributes.version = version;
    self
  }

  pub fn with_color_bits(mut self, color_bits: u8) -> Self {
    self.attributes.color_bits = color_bits;
    self
  }

  pub fn with_depth_bits(mut self, depth_bits: u8) -> Self {
    self.attributes.depth_bits = depth_bits;
    self
  }

  pub fn build(self, window: &Window) -> Render {
    Render {
      context: platform::Render::new(window, self.attributes),
    }
  }
}
