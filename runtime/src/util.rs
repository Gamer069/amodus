use std::ffi::{CString, OsString, c_void};
use std::fs::{OpenOptions, create_dir_all};
use std::io::Write;
use std::os::windows::ffi::OsStringExt;
use std::path::PathBuf;
use std::ptr::null_mut;
use std::sync::OnceLock;

use crate::proxy::{self, GetModuleHandleA, GetProcAddress};

#[allow(dead_code)]
pub const DLL_PROCESS_ATTACH: u32 = 1;

#[allow(dead_code)]
pub const DLL_PROCESS_DETACH: u32 = 0;

#[allow(dead_code)]
pub const DLL_THREAD_ATTACH: u32 = 2;

#[allow(dead_code)]
pub const DLL_THREAD_DETACH: u32 = 3;

pub static AMODUS_DIR: OnceLock<PathBuf> = OnceLock::new();

pub fn get_amodus_dir() -> PathBuf {
	AMODUS_DIR.get_or_init(|| unsafe {
		// --- SHGetKnownFolderPath ---
		let mut path_ptr: *mut u16 = null_mut();
		let hr = proxy::SHGetKnownFolderPath(
			&proxy::FOLDERID_LOCAL_APPDATA,
			0,
			null_mut(),
			&mut path_ptr,
		);

		if hr < 0 || path_ptr.is_null() {
			// fallback if FOLDERID fails
			PathBuf::from("C:\\Amodus")
		} else {
			// calculate UTF-16 string length
			let mut len = 0;
			while *path_ptr.add(len) != 0 {
				len += 1;
			}

			let slice = std::slice::from_raw_parts(path_ptr, len);
			let base: OsString = OsString::from_wide(slice);

			proxy::CoTaskMemFree(path_ptr as *mut c_void);

			let log_dir = PathBuf::from(base).join("Amodus");
			let _ = create_dir_all(&log_dir); // ensure directory exists
			log_dir
		}
	}).clone()
}

pub fn unity_main_exists() -> bool {
	unsafe {
		let main = CString::new("UnityMain").unwrap();
		let player = CString::new("UnityPlayer.dll").unwrap();

		// Get handle to UnityPlayer.dll if it’s loaded
		let unity_player = GetModuleHandleA(
			// pointer casting: BAD
			player.as_ptr() as *const u8
		);

		if unity_player.is_null() {
			// DLL not loaded yet
			return false;
		}

		// Try to get the UnityMain export
		let unity_main = GetProcAddress(
			unity_player,
			main.as_ptr() as *const u8,
		);

		!unity_main.is_null()
	}
}


pub fn log(msg: &str) {
	let amodus = get_amodus_dir();
	let log_path = amodus.join("amodus.log");

	let _ = create_dir_all(&amodus);

	if let Ok(mut file) = OpenOptions::new()
		.create(true)
		.append(true)
		.open(log_path)
	{
		let _ = writeln!(file, "{}", msg);
	}
}

