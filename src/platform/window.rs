#![windows_subsystem = "windows"]   // This make rust to not open a console for the app
#[cfg(windows)] extern crate winapi;
// Import the css color struct
use painting::Bitmap;

// to convert our Rust UTF-8 strings to Windows UTF-16 strings
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::iter::once;
//------------------------------------------------------------
use std::ptr::null_mut;
use std::io::Error;

// use is a "using namespace"
use self::winapi::shared::windef::{
    HWND, RECT, COLORREF
};
use self::winapi::shared::minwindef::{
    LRESULT, UINT,
    WPARAM, LPARAM,
};
use self::winapi::um::libloaderapi::GetModuleHandleW;

use self::winapi::um::wingdi::{
    CreateSolidBrush,
};

use self::winapi::um::winuser::{
    MSG, PAINTSTRUCT, WNDCLASSW,
    CS_OWNDC, CS_HREDRAW, CS_VREDRAW,
    CW_USEDEFAULT,
    WS_OVERLAPPEDWINDOW, WS_VISIBLE,
    WM_PAINT, WM_SIZE,
    DefWindowProcW, RegisterClassW, CreateWindowExW,
    TranslateMessage, DispatchMessageW, GetMessageW,
    BeginPaint, FillRect, EndPaint,
};
//--------------------------------------------------------------
unsafe extern "system" fn custom_win_proc(
    h_wnd: HWND, 
    msg: UINT, 
    w_param: WPARAM, 
    l_param: LPARAM
) -> LRESULT {
    match msg {
        WM_SIZE => {

            1 as isize
        }
        WM_PAINT => {
            let mut paint_struct = std::mem::MaybeUninit::<PAINTSTRUCT>::zeroed().assume_init();
            let paint_struct_ptr = &mut paint_struct as *mut PAINTSTRUCT;

            let hdc = BeginPaint(h_wnd, paint_struct_ptr);

            //let brush = CreatePatternBrush(buffer as HBITMAP);
            let brush = CreateSolidBrush(0xFF0099 as COLORREF);
            FillRect(hdc, &paint_struct.rcPaint as *const RECT, brush);

            EndPaint(h_wnd, paint_struct_ptr);
            
            1 as isize
        }
        _ => DefWindowProcW(h_wnd, msg, w_param, l_param)
    }
}

fn win32_string( value : &str ) -> Vec<u16> {
    OsStr::new( value ).encode_wide().chain( once( 0 ) ).collect()
}

pub struct Window {
    handle : HWND,
    width: i32,
    height: i32,
}

pub fn create_window( name : &str, title : &str, width: &i32, height: &i32) -> Result<Window, Error> {
    // convert the strings to win32 strings
    let name = win32_string( name );
    let title = win32_string( title );

    // tell rust compiler to overlook safety features
    unsafe {
        // Get the module handle
        let hinstance = GetModuleHandleW( null_mut() );

        // create the window class
        let wnd_class = WNDCLASSW {
            style : CS_OWNDC | CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc : Some( custom_win_proc ),
            hInstance : hinstance,
            lpszClassName : name.as_ptr(),
            cbClsExtra : 0,
            cbWndExtra : 0,
            hIcon: null_mut(),
            hCursor: null_mut(),
            hbrBackground: null_mut(),
            lpszMenuName: null_mut(),
        };

        RegisterClassW( &wnd_class );

        // Create the window
        let handle = CreateWindowExW(
            0,
            name.as_ptr(),
            title.as_ptr(),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            //WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            *width,
            *height,
            null_mut(),
            null_mut(),
            hinstance,
            null_mut() );

        if handle.is_null() {
            Err( Error::last_os_error() )
        } else {
            Ok( Window { handle: handle, width: *width, height: *height} )
        }
    }
}

impl Window {
    pub fn set_bitmap(&mut self, bitmap: & Bitmap) {
        //self.bitmap = bitmap;
    }

    pub fn handle_message(&self) -> bool {
        unsafe {
            let mut message : MSG = std::mem::MaybeUninit::<MSG>::uninit().assume_init();
            if GetMessageW( &mut message as *mut MSG, self.handle, 0, 0 ) > 0 {
                TranslateMessage( &message as *const MSG );
                DispatchMessageW( &message as *const MSG );

                true
            } else {
                false
            }
        }
    }
}