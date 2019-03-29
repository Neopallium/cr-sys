extern crate cr_sys;

use std::os::raw::{c_int};

use cr_sys::*;

#[no_mangle]
pub fn cr_main(_ctx: &mut cr_plugin, _cr_op: c_int) -> c_int {
    // Test "guest" feature.
    #[cfg(not(guest))]
    {
        // This is just to show if the guest was compiled with host-side code.
        println!("Guest compiled with host-side code.");
        // This code will only run if the "guest" feature is not used.
        let mut plugin = cr_sys::cr_plugin::new();

        let s_fullpath = std::ffi::CString::new("libtest.so").unwrap();
        unsafe { host::cr_plugin_load(&mut plugin, s_fullpath.as_ptr());}
        println!("- plugin = {:?}", plugin);
    }

    println!("Hello from rust plugin. test2");
    return 0;
}

