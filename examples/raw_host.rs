extern crate cr_sys;

use std::env;

use std::time::Duration;
use std::thread;

use cr_sys::*;

fn main() {
    // build the libraw_guest.so file from the samples of cr.h
    let mut plugin_name = env::current_exe().expect("Failed to get current path");
    println!("Path of this executable is: {}", plugin_name.display());
    plugin_name.set_file_name("libraw_guest.so");

    let plugin_name = plugin_name.to_str().unwrap();
    println!("Call cr_plugin_load(ctx, {:?})", plugin_name);
    let mut plugin = cr_sys::cr_plugin::new();
    let s_fullpath = std::ffi::CString::new(plugin_name).unwrap();
    unsafe { host::cr_plugin_load(&mut plugin, s_fullpath.as_ptr()) };

    loop {
        println!("Run Update:");
        let rc = unsafe { host::cr_plugin_update(&mut plugin, true)};
        println!("cr_plugin_update(ctx, true) = {}", rc);
        if rc != 0 { break }
        thread::sleep(Duration::from_millis(200));
    }
    println!("Call cr_plugin_close(ctx)");
    unsafe { host::cr_plugin_close(&mut plugin)};
    println!("exit");
}

