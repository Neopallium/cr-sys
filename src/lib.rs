#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

impl cr_plugin {
    pub fn new() -> cr_plugin {
        cr_plugin {
            p: std::ptr::null_mut(),
            userdata: std::ptr::null_mut(),
            version: 0,
            failure: cr_failure::CR_NONE,
            next_version: 1,
            last_working_version: 0,
        }
    }
}

impl Default for cr_plugin {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(not(guest))]
pub unsafe fn cr_plugin_get_filename(ctx: *mut cr_plugin) -> std::path::PathBuf {
    let mut buf: [u8; 1024] = [0; 1024];
    let len = rust_cr_plugin_get_filename(ctx, buf.as_mut_ptr(), buf.len()-1);

    std::path::PathBuf::from(String::from_utf8_lossy(&buf[0..len]).to_string())
}
