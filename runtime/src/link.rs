use crate::util;
use wasmtime::{Caller, Error, Linker};

use crate::load::AmodusModState;

pub fn link(linker: &mut Linker<AmodusModState>) -> Result<&mut Linker<AmodusModState>, Error> {
    linker.func_wrap(
        "env",
        "amodus_log",
        amodus_log_host
    )?;

    Ok(linker)
}

fn amodus_log_host<'a>(
    caller: Caller<'a, AmodusModState>,
    kind: u32,
    ptr: u32,
    len: u32,
) {
    let state = caller.data();
    let name = state.mod_toml.meta.display_name.clone();
    let version = state.mod_toml.meta.display_version.clone();

    if let Some(mem) = &caller.data().mem {
        let data = mem.data(&caller);
        let start = ptr as usize;
        let end = start + len as usize;
        if end <= data.len() {
            let s = std::str::from_utf8(&data[start..end]).unwrap_or("<invalid utf8>");
            util::log(&format!("[{} {}][{}] {}", name, version, kind, s));
        }
    }
}
