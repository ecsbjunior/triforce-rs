use crate::platform;

#[derive(Debug)]
pub struct Window {
  pub window: platform::Window,
}

impl Window {
  pub fn get_id(&self) -> i32 {
    self.window.get_id()
  }

  pub fn get_should_close(&self) -> bool {
    self.window.get_should_close()
  }

  pub fn destroy(self) {
    self.window.destroy()
  }

  pub fn handle_events(&mut self) {
    self.window.handle_events()
  }
}

#[derive(Debug)]
pub struct WindowAttributes {
  pub title: String,
  pub size: (i32, i32),
  pub should_close: bool,
}

impl Default for WindowAttributes {
  fn default() -> Self {
    Self {
      title: "Triforce Window".to_string(),
      size: (800, 600),
      should_close: false,
    }
  }
}

#[derive(Debug, Default)]
pub struct WindowBuilder {
  pub attributes: WindowAttributes,
}

impl WindowBuilder {
  pub fn new() -> Self {
    Default::default()
  }

  pub fn with_title(mut self, title: &str) -> Self {
    self.attributes.title = title.to_string();
    self
  }

  pub fn with_size(mut self, size: (i32, i32)) -> Self {
    self.attributes.size = size;
    self
  }

  pub fn build(self) -> Window {
    Window {
      window: platform::Window::new(self.attributes),
    }
  }
}
