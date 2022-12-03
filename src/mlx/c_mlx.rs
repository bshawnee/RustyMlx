use std::ffi::c_void;
use std::ffi::c_char;

#[link(name = "AppKit", kind="framework")]
#[link(name = "openGL", kind="framework")]
extern "C" {

    pub fn mlx_init() -> *mut c_void;
    /*
    **  needed before everything else.
    **  return (void *)0 if failed
    */

    /*
    ** Basic actions
    */

    pub fn mlx_new_window(
        mlx_ptr: *mut c_void,
        x: i32, 
        y: i32, 
        title: *mut c_char
    ) -> *mut c_void;
    /*
    **  return void *0 if failed
    */
    pub fn mlx_clear_window(
        mlx_ptr: *mut c_void,
        win_ptr: *mut c_void
    ) -> i32;
    pub fn mlx_pixel_put(
        mlx_ptr: *mut c_void, 
        win_ptr: *mut c_void, 
        x: i32, 
        y: i32, 
        color: i32
    ) -> i32;
    /*
    **  origin for x & y is top left corner of the window
    **  y down is positive
    **  color is 0x00RRGGBB
    */

    /*
    ** Image stuff
    */
    pub fn mlx_new_image(
        mlx_ptr: *mut c_void, 
        width: i32, 
        height: i32
    ) -> *mut c_void;
    /*
    **  return void *0 if failed
    */
    pub fn mlx_get_data_addr(
        img_ptr: *mut c_void,
        bits_per_pixel: *mut i32,
        size_line: *mut i32,
        endian: *mut i32
    ) -> *mut c_char;
    /*
    **  endian : 0 = sever X is little endian, 1 = big endian
    **  endian : useless on macos, client and graphical framework have the same endian
    */
    pub fn mlx_put_image_to_window(
        mlx_ptr: *mut c_void,
        win_ptr: *mut c_void,
        img_ptr: *mut c_void,
        x: i32,
        y: i32
    ) -> i32;
    pub fn mxl_get_color_value(
        mlx_ptr: *mut c_void,
        color: i32
    ) -> u32;


    /*
    ** dealing with Events
    */
    
    pub fn mlx_mouse_hook(
        win_ptr: *mut c_void, 
        func: unsafe extern "C" fn() -> i32,
        param: *mut c_void
    ) -> i32;
    pub fn mlx_key_hook(
        win_ptr: *mut c_void, 
        func: unsafe extern "C" fn() -> i32,
        param: *mut c_void
    ) -> i32;
    pub fn mlx_expose_hook(
        win_ptr: *mut c_void, 
        func: unsafe extern "C" fn() -> i32,
        param: *mut c_void
    ) -> i32;
    pub fn mlx_loop_hook(
        mlx_ptr: *mut c_void, 
        func: unsafe extern "C" fn() -> i32,
        param: *mut c_void
    ) -> i32;
    pub fn mlx_loop(
        mlx_ptr: *mut c_void
    ) -> i32;
    /*
    **  hook funct are called as follow :
    **
    **   expose_hook(void *param);
    **   key_hook(int keycode, void *param);
    **   mouse_hook(int button, int x,int y, void *param);
    **   loop_hook(void *param);
    **
    */

    /*
    **  Usually asked...
    */

    pub fn mlx_string_put(
        mlx_ptr: *mut c_void,
        win_ptr: *mut c_void,
        x: i32,
        y: i32,
        color: i32,
        string: *mut c_char
    ) -> i32;
    pub fn mlx_xpm_to_image(
        mlx_ptr: *mut c_void,
        xpm_data: *mut *mut c_char,
        width: *mut i32,
        height: *mut i32
    ) -> *mut c_void;
    pub fn mlx_xpm_file_to_image(
        mlx_ptr: *mut c_void,
        filename: *mut c_char,
        width: *mut i32,
        height: *mut i32
    ) -> *mut c_void;
    pub fn mlx_png_file_to_image(
        mlx_ptr: *mut c_void,
        filename: *mut c_char,
        width: *mut i32,
        height: *mut i32
    ) -> *mut c_void;
    pub fn mlx_destroy_window(
        mlx_ptr: *mut c_void,
        win_ptr: *mut c_void
    ) -> i32;
    pub fn mlx_destroy_image(
        mlx_ptr: *mut c_void,
        win_ptr: *mut c_void
    ) -> i32;
    
    /*
    **  generic hook system for all events, and minilibX functions that
    **  can be hooked. Some macro and defines from X11/X.h are needed here.
    */

    pub fn mlx_hook(
        win_ptr: *mut c_void,
        x_event: i32,
        x_mask: i32,
        func: unsafe extern "C" fn(*mut c_void) -> i32,
        param: *mut c_void
    ) -> i32;
    pub fn mlx_mouse_hide() -> i32;
    pub fn mlx_mouse_show() -> i32;
    pub fn mlx_mouse_move(
        win_ptr: *mut c_void,
        x: i32,
        y: i32
    ) -> i32;
    pub fn mlx_mouse_get_pos(
        win_ptr: *mut c_void,
        x: *mut i32,
        y: *mut i32
    ) -> i32;

    pub fn mlx_do_key_autorepeatoff(
        mlx_ptr: *mut c_void
    ) -> i32;
    pub fn mlx_do_key_autorepeaton(
        mlx_ptr: *mut c_void
    ) -> i32;
    pub fn mlx_do_sync(
        mlx_ptr: *mut c_void
    ) -> i32;
}
