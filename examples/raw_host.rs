extern crate cr_sys;

use std::env;

use std::thread;
use std::time::Duration;

use cr_sys::*;

mod raw_state;

use raw_state::*;

fn main() {
    // build the libraw_guest.so file from the samples of cr.h
    let mut plugin_name = env::current_exe().expect("Failed to get current path");
    println!("Path of this executable is: {}", plugin_name.display());
    plugin_name.set_file_name("libraw_guest.so");

    // Plugin
    let mut plugin = cr_sys::cr_plugin::new();
    let mut state = RawState { counter: 0 };
    plugin.userdata = &mut state as *mut _ as *mut ::std::os::raw::c_void;

    let plugin_name = plugin_name.to_str().unwrap();
    println!("Call cr_plugin_load(ctx, {:?})", plugin_name);
    let s_fullpath = std::ffi::CString::new(plugin_name).unwrap();
    unsafe { cr_plugin_load(&mut plugin, s_fullpath.as_ptr()) };

    loop {
        println!("Run Update: cr_failure={:?}", plugin.failure);
        let rc = unsafe { cr_plugin_update(&mut plugin, true) };
        println!("cr_plugin_update(ctx, true) = {}", rc);
        let file = unsafe { cr_plugin_get_filename(&mut plugin)};
        println!("Plugin current filename = {:?}", file);
        if rc != 0 {
            // Allow plugin to signal host to exit.
            if rc == -10 {
                break;
            }
            println!("Plugin error: {:?}", plugin.failure);
        }
        thread::sleep(Duration::from_millis(200));
    }
    println!("Call cr_plugin_close(ctx)");
    unsafe { cr_plugin_close(&mut plugin) };
    println!("exit");
}
