use std::{env, fs, io::Write};

use crate::{eh, eh_direct, log, vlog, log_no};

const LIB_RS_TEMPLATE: &str = include_str!("../templates/lib.rs.template");
const CARGO_TOML_TEMPLATE: &str = include_str!("../templates/Cargo.toml.template");
const CARGO_CONFIG_TOML_TEMPLATE: &str = include_str!("../templates/config.toml.template");
const MOD_TOML_TEMPLATE: &str = include_str!("../templates/mod.toml.template");

pub fn new(v: bool, name: String, version: String, edition: String) {
	log!("Generating {}...", name);

	let cwd = env::current_dir();

	eh!(cwd, "Failed to get current working directory");

	let path = cwd.join(&name);
	let res = path.join("res");
	let src = path.join("src");
	let cargo = path.join(".cargo");

	if std::fs::exists(&path).unwrap_or(false) {
		log!("Project directory already exists...");
		log_no!("Recreate it? [Y/n] ");
		let mut line = String::new();
		eh_direct!(std::io::stdin().read_line(&mut line), "Failed to read line from stdin");
		let choice = line.trim().to_lowercase();

		if choice == "n" || choice == "no" {
			log!("Terminating...");
			return;
		}

		eh_direct!(std::fs::remove_dir_all(&path), "Failed to wipe project directories");
	}

	let lib_rs = format_template(LIB_RS_TEMPLATE, &name, &version, &edition);
	let cargo_toml = format_template(CARGO_TOML_TEMPLATE, &name, &version, &edition);
	let cargo_config_toml = format_template(CARGO_CONFIG_TOML_TEMPLATE, &name, &version, &edition);
	let mod_toml = format_template(MOD_TOML_TEMPLATE, &name, &version, &edition);

	// Creating
	vlog!(v, "Creating project directories...");

	eh_direct!(fs::create_dir(&path), "Failed to create project directory");
	vlog!(v, "Created project directory...");

	eh_direct!(fs::create_dir(&src), "Failed to create source directory");
	vlog!(v, "Created project source directory...");

	eh_direct!(fs::create_dir(&res), "Failed to create resource directory");
	vlog!(v, "Created project resource directory...");

	eh_direct!(fs::create_dir(&cargo), "Failed to create .cargo directory");
	vlog!(v, "Created project .cargo directory...");

	vlog!(v, "Created project directories!");

	// Writing
	vlog!(v, "Creating project files...");

	eh_direct!(fs::write(src.join("lib.rs"), lib_rs), "Failed to write lib.rs");
	vlog!(v, "Created project lib.rs file...");
	eh_direct!(fs::write(path.join("Cargo.toml"), cargo_toml), "Failed to write Cargo.toml");
	vlog!(v, "Created project Cargo.toml file...");
	eh_direct!(fs::write(path.join("mod.toml"), mod_toml), "Failed to write config.toml");
	vlog!(v, "Created project mod.toml file...");
	eh_direct!(fs::write(cargo.join("config.toml"), cargo_config_toml), "Failed to write config.toml");
	vlog!(v, "Created project .cargo/config.toml file...");

	vlog!(v, "Created project files!");
	
	log!("Generated mod project for {}", name);
}

pub fn format_template(template: &str, name: &str, version: &str, edition: &str) -> String {
	template.replace("{{MOD_NAME}}", name).replace("{{MOD_VERSION}}", version).replace("{{MOD_EDITION}}", edition)
}
