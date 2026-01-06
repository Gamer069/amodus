#![allow(non_snake_case)]

use std::os::raw::c_void;

// These hold the actual addresses of the real version.dll functions
#[unsafe(no_mangle)] static mut GetFileVersionInfoA_ptr: usize = 0;
#[unsafe(no_mangle)] static mut GetFileVersionInfoByHandle_ptr: usize = 0;
#[unsafe(no_mangle)] static mut GetFileVersionInfoExA_ptr: usize = 0;
#[unsafe(no_mangle)] static mut GetFileVersionInfoExW_ptr: usize = 0;
#[unsafe(no_mangle)] static mut GetFileVersionInfoSizeA_ptr: usize = 0;
#[unsafe(no_mangle)] static mut GetFileVersionInfoSizeExA_ptr: usize = 0;
#[unsafe(no_mangle)] static mut GetFileVersionInfoSizeExW_ptr: usize = 0;
#[unsafe(no_mangle)] static mut GetFileVersionInfoSizeW_ptr: usize = 0;
#[unsafe(no_mangle)] static mut GetFileVersionInfoW_ptr: usize = 0;
#[unsafe(no_mangle)] static mut VerFindFileA_ptr: usize = 0;
#[unsafe(no_mangle)] static mut VerFindFileW_ptr: usize = 0;
#[unsafe(no_mangle)] static mut VerInstallFileA_ptr: usize = 0;
#[unsafe(no_mangle)] static mut VerInstallFileW_ptr: usize = 0;
#[unsafe(no_mangle)] static mut VerLanguageNameA_ptr: usize = 0;
#[unsafe(no_mangle)] static mut VerLanguageNameW_ptr: usize = 0;
#[unsafe(no_mangle)] static mut VerQueryValueA_ptr: usize = 0;
#[unsafe(no_mangle)] static mut VerQueryValueW_ptr: usize = 0;

unsafe extern "system" {
	pub fn GetModuleHandleA(lpModuleName: *const u8) -> *mut std::ffi::c_void;
	pub fn GetProcAddress(hModule: *mut std::ffi::c_void, lpProcName: *const u8) -> *mut std::ffi::c_void;
}

// Proxy function exports
#[unsafe(no_mangle)]
pub extern "system" fn GetFileVersionInfoA(
	lptstrFilename: *const u8,
	dwHandle: u32,
	dwLen: u32,
	lpData: *mut u8
) -> i32 {
	unsafe {
		if GetFileVersionInfoA_ptr != 0 {
			let func: extern "system" fn(*const u8, u32, u32, *mut u8) -> i32 = 
				std::mem::transmute(GetFileVersionInfoA_ptr);
			func(lptstrFilename, dwHandle, dwLen, lpData)
		} else {
			0
		}
	}
}

#[unsafe(no_mangle)]
pub extern "system" fn GetFileVersionInfoByHandle(
	hMem: *mut std::ffi::c_void,
	lptstrFilename: *const u16,
	dwLen: u32,
	lpData: *mut u8
) -> i32 {
	unsafe {
		if GetFileVersionInfoByHandle_ptr != 0 {
			let func: extern "system" fn(*mut std::ffi::c_void, *const u16, u32, *mut u8) -> i32 = 
				std::mem::transmute(GetFileVersionInfoByHandle_ptr);
			func(hMem, lptstrFilename, dwLen, lpData)
		} else {
			0
		}
	}
}

#[unsafe(no_mangle)]
pub extern "system" fn GetFileVersionInfoExA(
	dwFlags: u32,
	lpwstrFilename: *const u8,
	dwHandle: u32,
	dwLen: u32,
	lpData: *mut u8
) -> i32 {
	unsafe {
		if GetFileVersionInfoExA_ptr != 0 {
			let func: extern "system" fn(u32, *const u8, u32, u32, *mut u8) -> i32 = 
				std::mem::transmute(GetFileVersionInfoExA_ptr);
			func(dwFlags, lpwstrFilename, dwHandle, dwLen, lpData)
		} else {
			0
		}
	}
}

#[unsafe(no_mangle)]
pub extern "system" fn GetFileVersionInfoExW(
	dwFlags: u32,
	lpwstrFilename: *const u16,
	dwHandle: u32,
	dwLen: u32,
	lpData: *mut u8
) -> i32 {
	unsafe {
		if GetFileVersionInfoExW_ptr != 0 {
			let func: extern "system" fn(u32, *const u16, u32, u32, *mut u8) -> i32 = 
				std::mem::transmute(GetFileVersionInfoExW_ptr);
			func(dwFlags, lpwstrFilename, dwHandle, dwLen, lpData)
		} else {
			0
		}
	}
}

#[unsafe(no_mangle)]
pub extern "system" fn GetFileVersionInfoSizeA(
	lptstrFilename: *const u8,
	lpdwHandle: *mut u32
) -> u32 {
	unsafe {
		if GetFileVersionInfoSizeA_ptr != 0 {
			let func: extern "system" fn(*const u8, *mut u32) -> u32 = 
				std::mem::transmute(GetFileVersionInfoSizeA_ptr);
			func(lptstrFilename, lpdwHandle)
		} else {
			0
		}
	}
}

#[unsafe(no_mangle)]
pub extern "system" fn GetFileVersionInfoSizeExA(
	dwFlags: u32,
	lpwstrFilename: *const u8,
	lpdwHandle: *mut u32
) -> u32 {
	unsafe {
		if GetFileVersionInfoSizeExA_ptr != 0 {
			let func: extern "system" fn(u32, *const u8, *mut u32) -> u32 = 
				std::mem::transmute(GetFileVersionInfoSizeExA_ptr);
			func(dwFlags, lpwstrFilename, lpdwHandle)
		} else {
			0
		}
	}
}

#[unsafe(no_mangle)]
pub extern "system" fn GetFileVersionInfoSizeExW(
	dwFlags: u32,
	lpwstrFilename: *const u16,
	lpdwHandle: *mut u32
) -> u32 {
	unsafe {
		if GetFileVersionInfoSizeExW_ptr != 0 {
			let func: extern "system" fn(u32, *const u16, *mut u32) -> u32 = 
				std::mem::transmute(GetFileVersionInfoSizeExW_ptr);
			func(dwFlags, lpwstrFilename, lpdwHandle)
		} else {
			0
		}
	}
}

#[unsafe(no_mangle)]
pub extern "system" fn GetFileVersionInfoSizeW(
	lptstrFilename: *const u16,
	lpdwHandle: *mut u32
) -> u32 {
	unsafe {
		if GetFileVersionInfoSizeW_ptr != 0 {
			let func: extern "system" fn(*const u16, *mut u32) -> u32 = 
				std::mem::transmute(GetFileVersionInfoSizeW_ptr);
			func(lptstrFilename, lpdwHandle)
		} else {
			0
		}
	}
}

#[unsafe(no_mangle)]
pub extern "system" fn GetFileVersionInfoW(
	lptstrFilename: *const u16,
	dwHandle: u32,
	dwLen: u32,
	lpData: *mut u8
) -> i32 {
	unsafe {
		if GetFileVersionInfoW_ptr != 0 {
			let func: extern "system" fn(*const u16, u32, u32, *mut u8) -> i32 = 
				std::mem::transmute(GetFileVersionInfoW_ptr);
			func(lptstrFilename, dwHandle, dwLen, lpData)
		} else {
			0
		}
	}
}

#[unsafe(no_mangle)]
pub extern "system" fn VerFindFileA(
	uFlags: u32,
	szFileName: *const u8,
	szWinDir: *const u8,
	szAppDir: *const u8,
	szCurDir: *mut u8,
	lpuCurDirLen: *mut u32,
	szDestDir: *mut u8,
	lpuDestDirLen: *mut u32
) -> u32 {
	unsafe {
		if VerFindFileA_ptr != 0 {
			let func: extern "system" fn(u32, *const u8, *const u8, *const u8, *mut u8, *mut u32, *mut u8, *mut u32) -> u32 = 
				std::mem::transmute(VerFindFileA_ptr);
			func(uFlags, szFileName, szWinDir, szAppDir, szCurDir, lpuCurDirLen, szDestDir, lpuDestDirLen)
		} else {
			0
		}
	}
}

#[unsafe(no_mangle)]
pub extern "system" fn VerFindFileW(
	uFlags: u32,
	szFileName: *const u16,
	szWinDir: *const u16,
	szAppDir: *const u16,
	szCurDir: *mut u16,
	lpuCurDirLen: *mut u32,
	szDestDir: *mut u16,
	lpuDestDirLen: *mut u32
) -> u32 {
	unsafe {
		if VerFindFileW_ptr != 0 {
			let func: extern "system" fn(u32, *const u16, *const u16, *const u16, *mut u16, *mut u32, *mut u16, *mut u32) -> u32 = 
				std::mem::transmute(VerFindFileW_ptr);
			func(uFlags, szFileName, szWinDir, szAppDir, szCurDir, lpuCurDirLen, szDestDir, lpuDestDirLen)
		} else {
			0
		}
	}
}

#[unsafe(no_mangle)]
pub extern "system" fn VerInstallFileA(
	uFlags: u32,
	szSrcFileName: *const u8,
	szDestFileName: *const u8,
	szSrcDir: *const u8,
	szDestDir: *const u8,
	szCurDir: *const u8,
	szTmpFile: *mut u8,
	lpuTmpFileLen: *mut u32
) -> u32 {
	unsafe {
		if VerInstallFileA_ptr != 0 {
			let func: extern "system" fn(u32, *const u8, *const u8, *const u8, *const u8, *const u8, *mut u8, *mut u32) -> u32 = 
				std::mem::transmute(VerInstallFileA_ptr);
			func(uFlags, szSrcFileName, szDestFileName, szSrcDir, szDestDir, szCurDir, szTmpFile, lpuTmpFileLen)
		} else {
			0
		}
	}
}

#[unsafe(no_mangle)]
pub extern "system" fn VerInstallFileW(
	uFlags: u32,
	szSrcFileName: *const u16,
	szDestFileName: *const u16,
	szSrcDir: *const u16,
	szDestDir: *const u16,
	szCurDir: *const u16,
	szTmpFile: *mut u16,
	lpuTmpFileLen: *mut u32
) -> u32 {
	unsafe {
		if VerInstallFileW_ptr != 0 {
			let func: extern "system" fn(u32, *const u16, *const u16, *const u16, *const u16, *const u16, *mut u16, *mut u32) -> u32 = 
				std::mem::transmute(VerInstallFileW_ptr);
			func(uFlags, szSrcFileName, szDestFileName, szSrcDir, szDestDir, szCurDir, szTmpFile, lpuTmpFileLen)
		} else {
			0
		}
	}
}

#[unsafe(no_mangle)]
pub extern "system" fn VerLanguageNameA(
	wLang: u32,
	szLang: *mut u8,
	cchLang: u32
) -> u32 {
	unsafe {
		if VerLanguageNameA_ptr != 0 {
			let func: extern "system" fn(u32, *mut u8, u32) -> u32 = 
				std::mem::transmute(VerLanguageNameA_ptr);
			func(wLang, szLang, cchLang)
		} else {
			0
		}
	}
}

#[unsafe(no_mangle)]
pub extern "system" fn VerLanguageNameW(
	wLang: u32,
	szLang: *mut u16,
	cchLang: u32
) -> u32 {
	unsafe {
		if VerLanguageNameW_ptr != 0 {
			let func: extern "system" fn(u32, *mut u16, u32) -> u32 = 
				std::mem::transmute(VerLanguageNameW_ptr);
			func(wLang, szLang, cchLang)
		} else {
			0
		}
	}
}

#[unsafe(no_mangle)]
pub extern "system" fn VerQueryValueA(
	pBlock: *const u8,
	lpSubBlock: *const u8,
	lplpBuffer: *mut *mut u8,
	puLen: *mut u32
) -> i32 {
	unsafe {
		if VerQueryValueA_ptr != 0 {
			let func: extern "system" fn(*const u8, *const u8, *mut *mut u8, *mut u32) -> i32 = 
				std::mem::transmute(VerQueryValueA_ptr);
			func(pBlock, lpSubBlock, lplpBuffer, puLen)
		} else {
			0
		}
	}
}

#[unsafe(no_mangle)]
pub extern "system" fn VerQueryValueW(
	pBlock: *const u8,
	lpSubBlock: *const u16,
	lplpBuffer: *mut *mut u8,
	puLen: *mut u32
) -> i32 {
	unsafe {
		if VerQueryValueW_ptr != 0 {
			let func: extern "system" fn(*const u8, *const u16, *mut *mut u8, *mut u32) -> i32 = 
				std::mem::transmute(VerQueryValueW_ptr);
			func(pBlock, lpSubBlock, lplpBuffer, puLen)
		} else {
			0
		}
	}
}

pub unsafe fn init_proxy() {
	unsafe {
		#[link(name = "kernel32")]
		unsafe extern "system" {
			fn LoadLibraryW(lpFileName: *const u16) -> *mut std::ffi::c_void;
			fn GetProcAddress(hModule: *mut std::ffi::c_void, lpProcName: *const u8) -> *mut std::ffi::c_void;
			fn GetSystemDirectoryW(lpBuffer: *mut u16, uSize: u32) -> u32;
		}

		const MAX_PATH: usize = 260;
		let mut system_path_buf = [0u16; MAX_PATH];
		let len = GetSystemDirectoryW(system_path_buf.as_mut_ptr(), MAX_PATH as u32);

		if len == 0 {
			return;
		}

		let mut dll_path = system_path_buf[..len as usize].to_vec();
		dll_path.extend_from_slice(&['\\' as u16, 'v' as u16, 'e' as u16, 'r' as u16, 's' as u16, 'i' as u16, 'o' as u16, 'n' as u16, '.' as u16, 'd' as u16, 'l' as u16, 'l' as u16, 0]);

		let real_dll = LoadLibraryW(dll_path.as_ptr());

		if real_dll.is_null() {
			return;
		}

		macro_rules! get_proc {
			($dll:expr, $ptr_var:ident, $func_name:literal) => {
				$ptr_var = GetProcAddress($dll, concat!($func_name, "\0").as_ptr()) as usize;
			};
		}

		get_proc!(real_dll, GetFileVersionInfoA_ptr, "GetFileVersionInfoA");
		get_proc!(real_dll, GetFileVersionInfoByHandle_ptr, "GetFileVersionInfoByHandle");
		get_proc!(real_dll, GetFileVersionInfoExA_ptr, "GetFileVersionInfoExA");
		get_proc!(real_dll, GetFileVersionInfoExW_ptr, "GetFileVersionInfoExW");
		get_proc!(real_dll, GetFileVersionInfoSizeA_ptr, "GetFileVersionInfoSizeA");
		get_proc!(real_dll, GetFileVersionInfoSizeExA_ptr, "GetFileVersionInfoSizeExA");
		get_proc!(real_dll, GetFileVersionInfoSizeExW_ptr, "GetFileVersionInfoSizeExW");
		get_proc!(real_dll, GetFileVersionInfoSizeW_ptr, "GetFileVersionInfoSizeW");
		get_proc!(real_dll, GetFileVersionInfoW_ptr, "GetFileVersionInfoW");
		get_proc!(real_dll, VerFindFileA_ptr, "VerFindFileA");
		get_proc!(real_dll, VerFindFileW_ptr, "VerFindFileW");
		get_proc!(real_dll, VerInstallFileA_ptr, "VerInstallFileA");
		get_proc!(real_dll, VerInstallFileW_ptr, "VerInstallFileW");
		get_proc!(real_dll, VerLanguageNameA_ptr, "VerLanguageNameA");
		get_proc!(real_dll, VerLanguageNameW_ptr, "VerLanguageNameW");
		get_proc!(real_dll, VerQueryValueA_ptr, "VerQueryValueA");
		get_proc!(real_dll, VerQueryValueW_ptr, "VerQueryValueW");
	}
}

#[repr(C)]
pub struct GUID {
	data1: u32,
	data2: u16,
	data3: u16,
	data4: [u8; 8],
}

// FOLDERID_LocalAppData
pub const FOLDERID_LOCAL_APPDATA: GUID = GUID {
	data1: 0xF1B32785,
	data2: 0x6FBA,
	data3: 0x4FCF,
	data4: [0x9D, 0x55, 0x7B, 0x8E, 0x7F, 0x15, 0x70, 0x91],
};

#[link(name = "shell32")]
unsafe extern "system" {
	pub fn SHGetKnownFolderPath(
		rfid: *const GUID,
		dwFlags: u32,
		hToken: *mut c_void,
		ppszPath: *mut *mut u16,
	) -> i32;
}

#[link(name = "ole32")]
unsafe extern "system" {
	pub fn CoTaskMemFree(pv: *mut c_void);
}

