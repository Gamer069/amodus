use std::{fs::File, path::Path};

use tar::{Archive, Builder};
use zstd::Encoder;

/// Represents a `.sus` mod archive.
pub struct Sus;

impl Sus {
	pub fn new() -> Self {
		Self
	}

	/// Constructs a `.sus` archive from a wasm file, a mod.toml, and an optional `res/` directory
	pub fn construct(
		&self,
		verbose: bool,
		wasm: &Path,
		mod_toml: &Path,
		res_dir: Option<&Path>,
		output: &Path,
	) {
		log!("Creating .sus archive: {}", output.display());

		let f = File::create(output).unwrap_or_else(|e| {
			eprintln!("Failed to create {}: {}", output.display(), e);
			std::process::exit(-1);
		});

		let encoder = Encoder::new(f, 3).unwrap_or_else(|e| {
			eprintln!("Failed to create zstd encoder: {}", e);
			std::process::exit(-1);
		});

		let mut archive = Builder::new(encoder);

		// Append res/ directory if provided
		if let Some(res) = res_dir {
			if res.exists() {
				vlog!(verbose, "Appending {} to archive...", res.display());
				archive
					.append_dir_all("res", res)
					.unwrap_or_else(|e| panic!("Failed to append res/: {}", e));
			}
		}

		// Append wasm
		if !wasm.exists() {
			eprintln!("WASM file not found: {}!", wasm.display());
			std::process::exit(-1);
		}
		vlog!(verbose, "Appending {} as mod.wasm...", wasm.display());
		archive
			.append_path_with_name(wasm, "mod.wasm")
			.unwrap_or_else(|e| panic!("Failed to append wasm: {}", e));

		// Append mod.toml
		if !mod_toml.exists() {
			eprintln!("mod.toml not found: {}!", mod_toml.display());
			std::process::exit(-1);
		}
		vlog!(verbose, "Appending {}...", mod_toml.display());
		archive
			.append_path(mod_toml)
			.unwrap_or_else(|e| panic!("Failed to append mod.toml: {}", e));

		archive.finish().unwrap_or_else(|e| panic!("Failed to finish archive: {}", e));
		let encoder = archive.into_inner().unwrap();
		encoder.finish().unwrap();

		log!("Created .sus archive: {}", output.display());
	}

	/// Extracts a `.sus` archive to a target directory
	pub fn extract(&self, verbose: bool, sus_file: &Path, target_dir: &Path) {
		if !sus_file.exists() {
			eprintln!("File not found: {}", sus_file.display());
			std::process::exit(-1);
		}

		log!("Extracting {} to {}...", sus_file.display(), target_dir.display());

		let f = File::open(sus_file).unwrap_or_else(|e| {
			eprintln!("Failed to open {}: {}", sus_file.display(), e);
			std::process::exit(-1);
		});

		let decoder = zstd::Decoder::new(f).unwrap_or_else(|e| {
			eprintln!("Failed to create zstd decoder: {}", e);
			std::process::exit(-1);
		});

		let mut archive = Archive::new(decoder);
		archive
			.unpack(target_dir)
			.unwrap_or_else(|e| panic!("Failed to extract archive: {}", e));

		vlog!(verbose, "Extraction complete to {}", target_dir.display());
	}
}

// Macros for logging
#[macro_export]
macro_rules! log {
	($($arg:tt)*) => {
		println!($($arg)*);
	};
}

#[macro_export]
macro_rules! vlog {
	($verbose:expr, $($arg:tt)*) => {
		if $verbose {
			println!($($arg)*);
		}
	};
}

