use std::ffi::c_void;

use windows_sys::{
  Win32::{
    Foundation::*,
    System::LibraryLoader::*,
    Graphics::{ Gdi::*, OpenGL::* },
  },
  *
};

use triforce_window::window::{WindowBuilder, Window};

use crate::render::RenderAttributes;

type WglCreateContextAttribsARB = extern "system" fn(HDC, HGLRC, *const i32) -> HGLRC;
type WglChoosePixelFormatARB = extern "system" fn(HDC, *const i32, *const f32, u32, *mut i32, *mut u32) -> i32;

const WGL_DRAW_TO_WINDOW_ARB: i32 = 0x2001;
const WGL_SUPPORT_OPENGL_ARB: i32 = 0x2010;
const WGL_DOUBLE_BUFFER_ARB: i32 = 0x2011;
const WGL_COLOR_BITS_ARB: i32 = 0x2014;
const WGL_DEPTH_BITS_ARB: i32 = 0x2022;

const WGL_CONTEXT_MAJOR_VERSION_ARB: i32 = 0x2091;
const WGL_CONTEXT_MINOR_VERSION_ARB: i32 = 0x2092;
const WGL_CONTEXT_FLAGS_ARB: i32 = 0x2094;
const WGL_CONTEXT_CORE_PROFILE_BIT_ARB: i32 = 0x00000001;

#[derive(Debug)]
pub struct Render {
  pub gl_context: HGLRC,
  pub gl_library: HMODULE,
  pub device_context: HDC,
}

impl Render {
  pub fn new(window: &Window, attributes: RenderAttributes) -> Self {
    let dummy_window = WindowBuilder::new()
      .with_size((0, 0))
      .build();
    
    let pixel_format_descriptor = PIXELFORMATDESCRIPTOR {
      nSize: std::mem::size_of::<PIXELFORMATDESCRIPTOR>() as u16,
      nVersion: 1,
      dwFlags: PFD_DRAW_TO_WINDOW | PFD_SUPPORT_OPENGL | PFD_DOUBLEBUFFER,
      iPixelType: PFD_TYPE_RGBA,
      cColorBits: attributes.color_bits,
      cRedBits: 0,
      cRedShift: 0,
      cGreenBits: 0,
      cGreenShift: 0,
      cBlueBits: 0,
      cBlueShift: 0,
      cAlphaBits: 0,
      cAlphaShift: 0,
      cAccumBits: 0,
      cAccumRedBits: 0,
      cAccumGreenBits: 0,
      cAccumBlueBits: 0,
      cAccumAlphaBits: 0,
      cDepthBits: attributes.depth_bits,
      cStencilBits: 0,
      cAuxBuffers: 0,
      iLayerType: PFD_MAIN_PLANE,
      bReserved: 0,
      dwLayerMask: 0,
      dwVisibleMask: 0,
      dwDamageMask: 0,
    };

    let dummy_handle = dummy_window.get_id().try_into().unwrap();
    let dummy_device_context = unsafe { GetDC(dummy_handle) };

    let pixel_format = unsafe { ChoosePixelFormat(dummy_device_context, &pixel_format_descriptor) };

    if pixel_format == 0 {
      panic!("Failed to choose pixel format");
    }
    
    let pixel_format_was_choose = unsafe { SetPixelFormat(dummy_device_context, pixel_format, &pixel_format_descriptor) } != 0;

    if !pixel_format_was_choose {
      panic!("Failed to set pixel format");
    }

    let dummy_gl_context = unsafe { wglCreateContext(dummy_device_context) };

    if dummy_gl_context == 0 {
      panic!("Failed to create OpenGL context");
    }

    let dummy_gl_context_was_made_current = unsafe { wglMakeCurrent(dummy_device_context, dummy_gl_context) } != 0;

    if !dummy_gl_context_was_made_current {
      panic!("Failed to make OpenGL context current");
    }

    let wgl_choose_pixel_format_arb: WglChoosePixelFormatARB = unsafe {
      let symbol = s!("wglChoosePixelFormatARB");
      let address = wglGetProcAddress(symbol);

      match address {
        Some(addr) => std::mem::transmute(addr),
        None => panic!("Failed to load wglChoosePixelFormatARB"),
      }
    };

    let wgl_create_context_attribs_arb: WglCreateContextAttribsARB = unsafe {
      let symbol = s!("wglCreateContextAttribsARB");
      let address = wglGetProcAddress(symbol);

      match address {
        Some(addr) => std::mem::transmute(addr),
        None => panic!("Failed to load wglCreateContextAttribsARB"),
      }
    };

    unsafe { wglMakeCurrent(0, 0) };
    unsafe { ReleaseDC(dummy_handle, dummy_device_context) };
    unsafe { wglDeleteContext(dummy_gl_context) };
    dummy_window.destroy();

    //attach to window
    let handle = window.get_id().try_into().unwrap();
    let device_context = unsafe { GetDC(handle) };

    let mut pixel_format = 0;
    let mut pixel_format_number = 0;
    let max_pixel_format_number = 1;

    let i_pixel_format_attribs = [
      [WGL_DRAW_TO_WINDOW_ARB, 1],
      [WGL_SUPPORT_OPENGL_ARB, 1],
      [WGL_DOUBLE_BUFFER_ARB, 1],
      [WGL_COLOR_BITS_ARB, attributes.color_bits as i32],
      [WGL_DEPTH_BITS_ARB, attributes.depth_bits as i32],
      [0, 0],
    ];

    let pixel_format_was_choose = wgl_choose_pixel_format_arb(
      device_context,
      i_pixel_format_attribs.as_ptr().cast(),
      std::ptr::null(),
      max_pixel_format_number,
      &mut pixel_format,
      &mut pixel_format_number,
    ) != 0;

    if !pixel_format_was_choose {
      panic!("Could not choose a pixel format");
    };

    let mut pixel_format_descriptor = unsafe { std::mem::zeroed::<PIXELFORMATDESCRIPTOR>() };

    let pixel_format_was_set = unsafe { SetPixelFormat(device_context, pixel_format, &mut pixel_format_descriptor) } != 0;

    if !pixel_format_was_set {
      panic!("Could not set the pixel format");
    }

    let i_context_attribs = [
        [WGL_CONTEXT_MAJOR_VERSION_ARB, attributes.version.0 as i32],
        [WGL_CONTEXT_MINOR_VERSION_ARB, attributes.version.1 as i32],
        [WGL_CONTEXT_FLAGS_ARB, WGL_CONTEXT_CORE_PROFILE_BIT_ARB],
        [0, 0],
      ];

    let gl_context = wgl_create_context_attribs_arb(
      device_context,
      0,
      i_context_attribs.as_ptr().cast(),
    );

    if gl_context == 0 {
      panic!("Could not create the OpenGL context");
    }

    let gl_context_was_made_current = unsafe { wglMakeCurrent(device_context, gl_context) } != 0;

    if !gl_context_was_made_current {
      panic!("Could not make the OpenGL context current");
    }

    let gl_library = unsafe { GetModuleHandleA(s!("OpenGL32.dll")) };

    Self {
      gl_context,
      gl_library,
      device_context,
    }
  }

  pub fn make_current(&self) {
    unsafe { wglMakeCurrent(self.device_context, self.gl_context) };
  }

  pub fn swap_buffers(&self) {
    unsafe { SwapBuffers(self.device_context) };
  }

  pub fn get_proc_address(&self, name: &str) -> *const c_void {
    let address = unsafe { wglGetProcAddress(name.as_ptr()) };

    match address {
      Some(addr) => unsafe { std::mem::transmute(addr) },
      None => unsafe { std::mem::transmute(GetProcAddress(self.gl_library, name.as_ptr())) },
    }
  }
}
