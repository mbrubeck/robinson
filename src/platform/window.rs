#![windows_subsystem = "windows"]   // This make rust to not open a console for the app
#[cfg(windows)] extern crate winapi;

// to convert our Rust UTF-8 strings to Windows UTF-16 strings
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::iter::once;
//------------------------------------------------------------
use std::ptr::null_mut;
use std::io::Error;

// use is a "using namespace"
use self::winapi::shared::windef::HWND;
use self::winapi::um::libloaderapi::GetModuleHandleW;

use self::winapi::um::winuser::{
    MSG,
    WNDCLASSW,
    CS_OWNDC,
    CS_HREDRAW,
    CS_VREDRAW,
    CW_USEDEFAULT,
    WS_OVERLAPPEDWINDOW,
    WS_VISIBLE,
    DefWindowProcW,
    RegisterClassW,
    CreateWindowExW,
    TranslateMessage,
    DispatchMessageW,
    GetMessageW,
};
//--------------------------------------------------------------


fn win32_string( value : &str ) -> Vec<u16> {
    OsStr::new( value ).encode_wide().chain( once( 0 ) ).collect()
}

pub struct Window {
    handle : HWND,
}

pub fn create_window( name : &str, title : &str ) -> Result<Window, Error> {
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
            lpfnWndProc : Some( DefWindowProcW ),
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
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            null_mut(),
            null_mut(),
            hinstance,
            null_mut() );

        if handle.is_null() {
            Err( Error::last_os_error() )
        } else {
            Ok( Window { handle } )
        }
    }
}

pub fn handle_message( window : &mut Window ) -> bool {
    unsafe {
        let mut message : MSG = std::mem::MaybeUninit::<MSG>::uninit().assume_init();
        if GetMessageW( &mut message as *mut MSG, window.handle, 0, 0 ) > 0 {
            TranslateMessage( &message as *const MSG );
            DispatchMessageW( &message as *const MSG );

            true
        } else {
            false
        }
    }
}