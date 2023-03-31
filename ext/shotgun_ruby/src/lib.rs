use rb_sys::*;
use std::ffi::{CStr, CString};
use std::os::raw::c_long;

use std::fs::File;
use std::path::Path;
use x11::xlib;


mod util;
mod xwrap;
use crate::xwrap::Display;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

unsafe extern "C" fn take_screenshot(_klass: RubyValue, mut winid: RubyValue, mut filepath: RubyValue) -> RubyValue {

    let window_id = CStr::from_ptr(rb_string_value_cstr(&mut winid)).to_str().unwrap();
    let path = CStr::from_ptr(rb_string_value_cstr(&mut filepath)).to_str().unwrap();

    let display = match Display::open(None) {
        Some(d) => d,
        None => {
            eprintln!("Failed to open display");
            return 1;
        }
    };

    let window = match util::parse_int::<xlib::Window>(&window_id){
        Ok(r) => r,
        Err(_) => {
            eprintln!("Window ID is not a valid integer");
            eprintln!("Accepted values are decimal, hex (0x*), octal (0o*) and binary (0b*)");
            return 1;
        }
    };

    let output_ext = "pam";
    let output_format = image::ImageOutputFormat::Pnm(image::pnm::PNMSubtype::ArbitraryMap);
    
    let window_rect = display.get_window_rect(window);
    let sel = util::Rect {
        x: 0,
        y: 0,
        w: window_rect.w,
        h: window_rect.h,
    };

    let image = match display.get_image(window, sel, xwrap::ALL_PLANES, xlib::ZPixmap) {
        Some(i) => i,
        None => {
            eprintln!("Failed to get image from X");
            return 1;
        }
    };

    let image = match image.into_image_buffer() {
        Some(i) => image::DynamicImage::ImageRgba8(i),
        None => {
            eprintln!(
                "Failed to convert captured framebuffer, only 24/32 \
                      bit (A)RGB8 is supported"
            );
            return 1;
        }
    };

    match File::create(&Path::new(&path)) {
        Ok(mut f) => image
            .write_to(&mut f, output_format)
            .expect("Writing to file failed"),
        Err(e) => {
            eprintln!("Failed to create {}: {}", path, e);
            return 1;
        }
    }

    let output = CString::new(path).unwrap();
    let size = path.len() as c_long;

    rb_utf8_str_new(output.as_ptr(), size)
}


#[allow(non_snake_case)]
#[no_mangle]
extern "C" fn Init_shotgun_ruby() {
    let name = CString::new("ShotgunRuby").unwrap();
    let function_name = CString::new("screenshot").unwrap();

    unsafe {
        let klass = rb_define_module(name.as_ptr());
        let callback = std::mem::transmute::<
            unsafe extern "C" fn(RubyValue, RubyValue, RubyValue) -> RubyValue,
            unsafe extern "C" fn() -> RubyValue,
        >(take_screenshot);
        rb_define_module_function(klass, function_name.as_ptr(), Some(callback), 2);
    }
}
