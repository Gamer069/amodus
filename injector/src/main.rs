use std::path::PathBuf;
use std::process::Command;
use std::fs;
use steamlocate::SteamDir;

/// :)
const AMOGUS_APP_ID: u32 = 945360;

/// CHANGE IF FORK
const RUNTIME_URL: &str = "https://github.com/Gamer069/amodus/releases/latest/download/amodus_runtime.dll";

fn find_among_us() -> Option<PathBuf> {
	let steamdir = SteamDir::locate().ok()?;

	// Among Us App ID is 945360
	let (app, library) = steamdir.find_app(AMOGUS_APP_ID).ok()??;

	// Get the game path
	let game_path = library.resolve_app_dir(&app).join("Among Us.exe");

	if game_path.exists() {
		Some(game_path)
	} else {
		None
	}
}

fn find_or_download_runtime(among_us_dir: &PathBuf) -> PathBuf {
	// Check current directory first
	let cwd_runtime = std::env::current_dir().unwrap().join("amodus_runtime.dll");
	if cwd_runtime.exists() {
		println!("[Amodus] Found runtime in current directory");
		return cwd_runtime;
	}
	
	// Check Among Us directory
	let game_runtime = among_us_dir.join("amodus_runtime.dll");
	if game_runtime.exists() {
		println!("[Amodus] Found runtime in game directory");
		return game_runtime;
	}
	
	// Download to current directory
	println!("[Amodus] Runtime not found, downloading from {}", RUNTIME_URL);
	download_runtime(&cwd_runtime);
	println!("[Amodus] Runtime downloaded");
	
	cwd_runtime
}

fn download_runtime(dest: &PathBuf) {
	let response = reqwest::blocking::get(RUNTIME_URL)
		.expect("Failed to download runtime");
	
	let bytes = response.bytes().expect("Failed to read runtime");
	fs::write(dest, bytes).expect("Failed to save runtime");
}

fn main() {
	match find_among_us() {
		Some(path) => {
			println!("[Amodus] Found Among Us at \"{}\"", path.display());
			
			let game_dir = path.parent().unwrap().to_path_buf();
			let runtime_dll = find_or_download_runtime(&game_dir);

			// Launch and inject
			#[cfg(target_os = "windows")]
			launch_windows(&path, &runtime_dll);

			#[cfg(target_os = "linux")]
			launch_wine(&path, &runtime_dll);
		}
		None => {
			eprintln!("[Amodus] Could not find existing Among Us installation!");
			eprintln!("Make sure the game is installed via Steam.");
		}
	}
}

#[cfg(target_os = "windows")]
fn launch_windows(exe: &PathBuf, dll: &PathBuf) {
	// Copy runtime as version.dll
	let game_dir = exe.parent().unwrap();
	fs::copy(dll, game_dir.join("version.dll")).expect("Failed to copy runtime");
	
	println!("[Amodus] Launching Among Us...");
	let mut child = Command::new(exe)
		.spawn()
		.expect("Failed to launch Among Us");
	
	child.wait().unwrap();
	println!("[Amodus] Among Us closed!");
}

#[cfg(target_os = "linux")]
fn launch_wine(exe: &PathBuf, dll: &PathBuf) {
	// Copy runtime as version.dll
	let game_dir = exe.parent().unwrap();
	fs::copy(dll, game_dir.join("version.dll")).expect("Failed to copy runtime");
	
	println!("[Amodus] Launching Among Us...");
	
	Command::new("wine")
		.current_dir(game_dir)
		.arg("Among Us.exe")
		.env("WINEDLLOVERRIDES", "version=n")
		.env("WINEDEBUG", "+loaddll")			// Debug output
		.spawn()
		.expect("Failed to launch Among Us")
		.wait()
		.unwrap();
	
	println!("[Amodus] Among Us closed");
}
