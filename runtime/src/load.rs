use core::fmt;
use std::{collections::HashMap, fs::{DirEntry, create_dir_all}, io};

use susio::Sus;
use tempfile::TempDir;
use wasmtime::{Config, Engine, Instance, Linker, Memory, Module, OptLevel, Store, Strategy, TypedFunc};

use crate::{link, meta::ModToml, util};

// the one AND ONLY AMODUS!!!!
static mut AMODUS: *mut Amodus = core::ptr::null_mut();

pub fn init_amodus() -> &'static mut Amodus {
	unsafe {
		if AMODUS.is_null() {
			let amodus = Box::new(Amodus::new());
			AMODUS = Box::into_raw(amodus);
		}

		&mut *AMODUS
	}
}


#[unsafe(no_mangle)]
pub extern "C" fn amodus_runtime_init() {
	util::log("[Amodus] Runtime initialized!");

	let mod_dir = util::get_amodus_dir().join("mods");
	let _ = create_dir_all(&mod_dir);

	let amodus = init_amodus();

	for cur_mod in std::fs::read_dir(mod_dir).unwrap() {
		if let Ok(cur_mod) = cur_mod {
			let path = cur_mod.path();

			if path.extension().is_none_or(|x| x != "sus") {
				util::log(&format!("Mod '{:?}' not sus, skipping...", path));
				continue;
			}

			util::log(&format!("[Amodus] Loading mod {}...", path.display()));

			let amodus_mod = amodus.load(cur_mod);

			if let Err(ref err) = amodus_mod {
				util::log(&format!(
					"error: Failed to load mod '{:?}': {}",
					path,
					err.to_string()
				));
			}

			let amodus_mod = amodus_mod.unwrap();

			amodus.add(amodus_mod);
		}
	}

	let mut to_be_removed = vec![];

	for (i, amodus_mod) in amodus.mods_mut().iter_mut().enumerate() {
		let mod_init: Result<TypedFunc<(), ()>, wasmtime::Error> = amodus_mod.wasm_inst
			.get_typed_func::<(), ()>(&mut amodus_mod.wasm_store, "amodus_mod_init");

		if mod_init.is_err() {
			// TODO: implement Cargo.toml parsing with cargo_toml and format mod name
			util::log(&format!(
				"warn: mod doesn't export modinitializer... skipping...",
			));

			to_be_removed.push(i);
			continue;
		}

		let mod_init = mod_init.unwrap();

		let res = mod_init.call(&mut amodus_mod.wasm_store, ());

		if let Err(err) = res {
			// TODO: implement Cargo.toml parsing with cargo_toml and format mod name
			util::log(&format!(
				"warn: failed to call modinitializer for mod: {}... skipping...",
				err.to_string()
			));

			to_be_removed.push(i);
			continue;
		}
	}

	to_be_removed.sort_unstable();

	to_be_removed.iter().rev().for_each(|i| amodus.remove(*i));
}

pub struct Amodus {
	pub mods: Vec<AmodusMod>,

	pub engine: Engine,
}


#[allow(dead_code)]
impl Amodus {
	pub fn new() -> Self {
		let mut config = Config::new();

        // DONT consume fuel because consuming fuel is stupid and limits the stuff a particular mod
        // can achieve.
		config.consume_fuel(false);

		config.cranelift_opt_level(OptLevel::SpeedAndSize);
		config.wasm_multi_memory(true);
		config.strategy(Strategy::Cranelift);

		let engine = Engine::new(&config).expect("Failed to init engine, engine was very sus!");

		Self { mods: vec![], engine }
	}

	pub fn mods(&self) -> &[AmodusMod] {
		&self.mods
	}

	pub fn remove(&mut self, i: usize) {
		self.mods.remove(i);
	}

	pub fn mods_mut(&mut self) -> &mut [AmodusMod] {
		&mut self.mods
	}

	pub fn add(&mut self, amodus_mod: AmodusMod) {
		self.mods.push(amodus_mod);
	}

	/// Loads a .sus package into an AmodusMod instance.
	pub fn load(&self, e: DirEntry) -> Result<AmodusMod, AmodusError> {
		let sus: Sus = Sus::new();

		let dir = TempDir::new()?;
		let path = dir.path();

		// TODO: dont hardcode verbosity
		sus.extract(false, e.path().as_path(), path);

		let mod_toml = std::fs::read_to_string(path.join("mod.toml"))?;
        let mod_toml: ModToml = toml::from_str(&mod_toml)?;

		let wasm = std::fs::read(path.join("mod.wasm"))?;
		let wasm = Module::new(&self.engine, wasm)?;

		let mut wasm_store = Store::new(&self.engine, AmodusModState { mem: None, mod_toml });

        let mut wasm_linker = Linker::<AmodusModState>::new(&self.engine);

        link::link(&mut wasm_linker)?;

		let wasm_inst = wasm_linker.instantiate(&mut wasm_store, &wasm)?;

        let mem = wasm_inst
            .get_memory(&mut wasm_store, "memory")
            .expect("mod must be not sus and the mod must export memory called 'memory'");

        wasm_store.data_mut().mem = Some(mem);

		let res_dir = std::fs::read_dir(path.join("res"))?;

		let mut resources = HashMap::new();

		for res in res_dir {
			let res = res?;
			
			let string = res.file_name().into_string();

			if let Err(_) = string {
				util::log(
					&format!(
						"warn: Failed to convert res OS string {:?} to string; skipping resource...",
						string
					)
				);

				continue;
			}

			resources.insert(string.unwrap(), std::fs::read(res.path())?);
		}

		Ok(AmodusMod { wasm, wasm_store, wasm_inst, resources })
	}
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum AmodusError {
	Io(io::Error),
	Toml(toml::de::Error),
	Wasm(wasmtime::Error),
	ResourceNotFound(String),
	InvalidMod(String),
	Other(String),
}

impl fmt::Display for AmodusError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			AmodusError::Io(e) => write!(f, "I/O error: {}", e),
			AmodusError::Toml(e) => write!(f, "Failed to parse TOML: {}", e),
			AmodusError::Wasm(e) => write!(f, "WASM module error: {}", e),
			AmodusError::ResourceNotFound(name) => write!(f, "Resource not found: {}", name),
			AmodusError::InvalidMod(msg) => write!(f, "Invalid mod structure: {}", msg),
			AmodusError::Other(msg) => write!(f, "Other error: {}", msg),
		}
	}
}

impl std::error::Error for AmodusError {}

// Optional From conversions for easy `?` propagation
impl From<io::Error> for AmodusError {
	fn from(e: io::Error) -> Self {
		AmodusError::Io(e)
	}
}

impl From<toml::de::Error> for AmodusError {
    fn from(e: toml::de::Error) -> Self {
        AmodusError::Toml(e)
    }
}

impl From<wasmtime::Error> for AmodusError {
	fn from(e: wasmtime::Error) -> Self {
		AmodusError::Wasm(e)
	}
}

#[allow(dead_code)]
pub struct AmodusMod {
	/// wasm module for actual mod loading
	pub wasm: Module,

	/// wasm store
	pub wasm_store: Store<AmodusModState>,

	/// wasm instance
	pub wasm_inst: Instance,

	/// name -> contents mod res/ hashmap
	pub resources: HashMap<String, Vec<u8>>
}

#[allow(dead_code)]
pub struct AmodusModState {
	/// parsed mod.toml
	pub mod_toml: ModToml,

    /// memory
    pub mem: Option<Memory>,
}
