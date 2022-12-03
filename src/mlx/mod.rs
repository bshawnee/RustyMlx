use std::borrow::{Borrow, BorrowMut};
use std::collections::HashMap;
use std::ffi::c_void;
use std::ffi::CString;
use std::string;

use self::c_mlx::mlx_clear_window;
use self::c_mlx::mlx_destroy_window;
use self::c_mlx::mlx_get_data_addr;
use self::c_mlx::mlx_hook;
use self::c_mlx::mlx_init;
use self::c_mlx::mlx_loop;
use self::c_mlx::mlx_new_image;
use self::c_mlx::mlx_new_window;
use self::c_mlx::mlx_put_image_to_window;
mod c_mlx;

pub struct MlxColor {
    green: u8,
    reg: u8,
    blue: u8
}

pub struct MlxContext {
    mlx_ptr: std::ptr::NonNull<c_void>
}
#[repr(C)]
pub struct MlxWindow {
    mlx_ptr: std::ptr::NonNull<c_void>,
    win_ptr: std::ptr::NonNull<c_void>,
    on_close_closure: Box<dyn Fn() -> i32>,
    on_event_closure: HashMap<MlxEvent, Vec<Box<dyn Fn() -> i32>>>
}  

pub struct Vec2i {
    pub x: i32,
    pub y: i32
}
pub struct MlxImage {
    image_ptr: std::ptr::NonNull<c_void>,
    data_addr: std::ptr::NonNull<i8>,
    bits_per_pixel: i32,
    size_line: i32,
    endian: i32,
    size: Vec2i
}

pub enum MlxKey {
    KeyPress(char),
    KeyRelease(char)
}

pub enum Button {
    Button1,
    Button2,
    Button3,
    Button4
}

pub enum MlxMouse {
    KeyPress(Button),
    KeyRelease(Button)
}
#[derive(Hash, Eq, PartialEq, Copy, Clone)]
pub enum MlxEvent {
    MlxMouse,
    MlxKey,

}

impl Vec2i {
    pub fn new(x: i32, y: i32) ->Vec2i {
        Vec2i {
            x,
            y
        }
    }
}

extern "C" fn on_event(ptr: *mut c_void) -> i32 {
    unsafe {
        let mut struct_p = ptr as *mut MlxWindow;
        struct_p.on_event_closure
    }
}

extern "C" fn close_window(ptr: *mut c_void) -> i32 {
    unsafe {
        let mut struct_p = ptr as *mut MlxWindow;
        mlx_destroy_window((*struct_p).mlx_ptr.as_ptr(), (*struct_p).win_ptr.as_ptr());
        (*struct_p).on_close_closure.as_ref()()
    }
}

impl MlxContext {
    pub fn new() -> MlxContext {
        MlxContext {
            mlx_ptr: std::ptr::NonNull::new(unsafe { mlx_init() })
                .expect("mlx: cant init context. Null ptr \"mlx_init\"")
        }
    }

    pub fn new_image(&self, x: i32, y: i32) -> MlxImage {
        let image_ptr = std::ptr::NonNull::new( unsafe {
            mlx_new_image(self.mlx_ptr.as_ptr(), x, y)
        }).expect("mlx: can't create image. Null ptr \"mlx_new_image\"");
        
        let mut bits_per_pixel: i32 = 0;
        let mut size_line: i32 = 0;
        let mut endian: i32 = 0;
        let data_addr = std::ptr::NonNull::new(unsafe {
            mlx_get_data_addr(
                image_ptr.as_ptr(), 
                std::ptr::addr_of_mut!(bits_per_pixel),
                std::ptr::addr_of_mut!(size_line),
                std::ptr::addr_of_mut!(endian))
        }).expect("mlx: cant take row image. Null ptr \"mlx_get_data_addr\"");
        MlxImage {  
            image_ptr: image_ptr.to_owned(),
            data_addr: data_addr.clone(),
            bits_per_pixel: bits_per_pixel.clone(),
            size_line: size_line.clone(),
            endian: endian.clone(),
            size: Vec2i { 
                x,
                y
            }
        }
    }

    pub fn new_window(&self, x: i32, y: i32, title: CString) -> MlxWindow {
        let mut win = MlxWindow { 
            mlx_ptr: self.mlx_ptr.clone(),
            win_ptr: std::ptr::NonNull::new(unsafe {
                mlx_new_window(self.mlx_ptr.as_ptr(), x, y, title.into_raw())
            }).expect("mlx: can't run window. Null ptr \"mlx_new_window\""),
            on_close_closure: Box::new(|| {0}),
            on_event_closure: HashMap::new()
        };
        unsafe {
            mlx_hook(win.win_ptr.as_ptr(), 17, 1<<2 , close_window, &win as *const MlxWindow as *mut c_void);
        }
        win
    }

    pub fn start_loop(&self) {
        unsafe { 
            if mlx_loop(self.mlx_ptr.as_ptr()) == 0 {
                panic!("mlx: can't start main loop. Zero \"mlx_loop\"");
            }
         }
    }
}

impl MlxWindow {
    pub fn put_image(&self, image: &MlxImage, x: i32, y: i32) {
        unsafe {
            if mlx_put_image_to_window(
                self.mlx_ptr.as_ptr(),
                self.win_ptr.as_ptr(),
                image.image_ptr.as_ptr(),
                x,
                y) == 0 {
                panic!("mlx: can't put image into window. Zero \"mlx_put_image_to_window\"")
            }
        }
    }
    pub fn clear(&self) {
        unsafe {
            mlx_clear_window(self.mlx_ptr.as_ptr(), self.win_ptr.as_ptr());
        }
    }
    
    pub fn on_close(&mut self, f: Box<dyn Fn() -> i32>) {
        self.on_close_closure = f;
    }

    pub fn on_event(&mut self, event: MlxEvent, f : Box<dyn Fn() -> i32>) {
        self.on_event_closure
            .entry(event)
            .or_insert_with(Vec::new)
            .push(f)
    }
}

impl MlxColor {
    pub fn to_hex(&self) -> i32 {
        ((self.reg as i32 & 0xff) << 16) + ((self.green as i32 & 0xff) << 8) + (self.blue as i32 & 0xff)
    }

    pub fn new(r: u8, g: u8, b: u8) -> MlxColor {
        MlxColor { 
            green: r, 
            reg: g, 
            blue: b 
        }
    }
}

impl MlxImage {
    pub fn put_pixel(&self, x: i32, y: i32, color: &MlxColor) {
        unsafe {
        let mut ptr = 
            self.data_addr.as_ptr().add((y * self.size_line + x * 4) as usize);
            *(ptr as *mut i32) = color.to_hex() as i32;
        };
    }
}
