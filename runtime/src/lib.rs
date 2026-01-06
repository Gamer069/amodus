mod util;
mod proxy;
mod load;
mod meta;
mod link;

use std::os::raw::c_void;

use crate::util::DLL_PROCESS_ATTACH;

#[allow(non_snake_case)]
#[unsafe(export_name = "DllMain")]
pub extern "system" fn DllMain(_: *mut c_void, reason: u32, _: *mut c_void) -> i32 {
	if reason == DLL_PROCESS_ATTACH {
		unsafe {
			proxy::init_proxy();
		}

		// NOTE: HACK SO RUNTIME DOESN'T GET CALLED MULTIPLE TIMES
		if util::unity_main_exists() {
            let amodus = util::get_amodus_dir();
            let log_path = amodus.join("amodus.log");

            if log_path.exists() {
                let _ = std::fs::remove_file(log_path);
            }

			load::amodus_runtime_init();
		}
	}

	1
}

