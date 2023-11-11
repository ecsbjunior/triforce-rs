use triforce_render::render::RenderBuilder;
use triforce_window::window::WindowBuilder;

fn main() {
  let mut window = WindowBuilder::new()
    .with_title("Triforce Window\0")
    .with_size((1280, 768))
    .build();

  let render = RenderBuilder::new()
    .build(&window);

  // gl::load_with(|name| render.get_proc_address(name));
  // gl::clear_color(0.0, 0.0, 0.0, 1.0);
  
  while !window.get_should_close() {
    render.make_current();

    window.handle_events();
    
    render.swap_buffers();
  }
}
