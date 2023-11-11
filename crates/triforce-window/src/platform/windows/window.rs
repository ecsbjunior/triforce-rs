use windows_sys::{
  Win32::{
    Foundation::*,
    UI::WindowsAndMessaging::*,
    System::{Threading::*, LibraryLoader::*},
  },
  *,
};

use crate::window::WindowAttributes;

const GE_CLOSE_WINDOW: u32 = WM_USER + 1;

#[derive(Debug)]
pub struct Window {
  handle: HWND,
  should_close: bool,
}

impl Window {
  pub fn new(attributes: WindowAttributes) -> Self {
    unsafe {
      let instance = Self::get_instance();
      let class_name = Self::get_class_name();

      let handle = CreateWindowExA(
        WINDOW_EX_STYLE::default(),
        class_name,
        attributes.title.as_ptr(),
        WS_OVERLAPPEDWINDOW | WS_VISIBLE,
        CW_USEDEFAULT,
        CW_USEDEFAULT,
        attributes.size.0,
        attributes.size.1,
        0,
        0,
        instance,
        std::ptr::null(),
      );

      Self {
        handle,
        should_close: attributes.should_close,
      }
    }
  }

  pub fn handle_events(&mut self) {
    unsafe {
      let mut data = std::mem::zeroed::<MSG>();

      while PeekMessageA(&mut data, 0, 0, 0, PM_REMOVE) != 0 {
        let handle = data.wParam.try_into().expect("could not convert wparam to handle");

        if self.handle == handle && data.message == GE_CLOSE_WINDOW {
          self.should_close = true;
        }

        TranslateMessage(&data);
        DispatchMessageA(&data);
      }
    }
  }

  pub fn get_id(&self) -> i32 {
    self.handle.try_into().unwrap()
  }

  pub fn get_should_close(&self) -> bool {
    self.should_close
  }

  pub fn destroy(self) {
    unsafe {
      DestroyWindow(self.handle);
    }
  }

  fn get_instance() -> HMODULE {
    let instance = unsafe { GetModuleHandleA(std::ptr::null()) };

    if instance == 0 {
      panic!("Failed to get instance");
    }

    instance
  }

  fn get_class_name() -> *const u8 {
    unsafe {
      let instance = Window::get_instance();

      let class_name = s!("TRIFORCE_WINDOW_CLASS_NAME");

      let mut window_class = std::mem::zeroed::<WNDCLASSA>();

      if GetClassInfoA(instance, class_name, &mut window_class) == 0 {
        window_class = WNDCLASSA {
          hIcon: 0,
          cbClsExtra: 0,
          cbWndExtra: 0,
          hbrBackground: 0,
          hInstance: instance,
          lpszMenuName: std::ptr::null(),
          lpfnWndProc: Some(window_callback),
          hCursor: LoadCursorW(0, IDC_ARROW),
          lpszClassName: class_name,
          style: CS_OWNDC | CS_HREDRAW | CS_VREDRAW,
        };

        RegisterClassA(&window_class);
      }

      class_name
    }
  }
}

pub extern "system" fn window_callback(handle: HWND, message: u32, wparam: WPARAM, lparam: LPARAM,) -> LRESULT {
  unsafe {
    let current_thread_id = GetCurrentThreadId();

    match message {
      WM_DESTROY => {
        let word_param = handle
          .try_into()
          .expect("could not convert handle to wparam");

        PostThreadMessageA(current_thread_id, GE_CLOSE_WINDOW, word_param, 0);

        LRESULT::default()
      }
      _ => DefWindowProcA(handle, message, wparam, lparam),
    }
  }
}
