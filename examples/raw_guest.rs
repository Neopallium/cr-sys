extern crate cr_sys;

use std::os::raw::c_int;

use std::thread;
use std::time::Duration;

use std::panic;

mod raw_state;

use raw_state::*;

use cr_sys::*;

use cr_sys::cr_op::*;

#[no_mangle]
pub fn cr_main(ctx: &mut cr_plugin, cr_op: cr_op) -> c_int {
    // Using the raw bindings we must catch unwind.
    match panic::catch_unwind(panic::AssertUnwindSafe(|| plugin_main(ctx, cr_op))) {
        Ok(rc) => rc,
        Err(e) => {
            // signal failure, host will rollback.
            ctx.failure = cr_failure::CR_SEGFAULT;
            println!("CAUGHT PANIC: {:?}", e);
            -1
        }
    }
}

pub fn plugin_main(ctx: &mut cr_plugin, cr_op: cr_op) -> c_int {
    // Test "guest" feature.
    #[cfg(not(guest))]
    {
        // This is just to show if the guest was compiled with host-side code.
        println!("Guest compiled with host-side code.");
        // This code will only run if the "guest" feature is not used.
        let mut plugin = cr_plugin::new();

        let s_fullpath = std::ffi::CString::new("libtest.so").unwrap();
        unsafe {
            cr_plugin_load(&mut plugin, s_fullpath.as_ptr());
        }
        println!("- plugin = {:?}", plugin);
    }

    println!("Hello from rust plugin. test1");

    match cr_op {
        CR_LOAD => {
            println!("Plugin load. version = {}", ctx.version);
        }
        CR_STEP => {
            let version = ctx.version;
            let state = unsafe { &mut *(ctx.userdata as *mut RawState) };
            state.counter += 1;
            println!(
                "Plugin step. count = {}. version = {}",
                state.counter, version
            );

            // slow down the printing.
            thread::sleep(Duration::from_millis(800));

            //panic!("test");
        }
        CR_UNLOAD => {
            println!("Plugin unload. version = {}", ctx.version);
        }
        CR_CLOSE => {
            println!("Plugin close. version = {}", ctx.version);
        }
    }

    return 0;
}
