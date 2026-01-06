# Amodus

A powerful modding framework for Among Us, written in Rust.

## What is Amodus? 💡

Amodus lets you create Among Us mods using Rust instead of C#. It provides a clean API for adding custom roles, abilities, and game mechanics while handling all the complexity of interacting with the game's internals.
It injects mods with a version.dll proxy.

## Features ✨

- **Rust-powered**: Write mods in Rust with full type safety and performance
- **Simple API**: Clean, intuitive interface for creating roles and abilities
- **CLI tools**: Easy mod creation, building, and installation
- **Cross-platform**: Works on all platforms supported by Among Us

## 📖 Quick Start (for developers) 🚀
```bash
# Install Amodus CLI
cargo install amodus-cli

# Create a new mod
amodus new my-mod

# Build and install
cd my-mod
amodus build
amodus install
```

## 🎮 Quick Start (for end users) 🏁
```bash
..? i dont know what the quickstart for end users will be for now
probably something similar to an installer..?
```

## Example Mod

The following example mod adds a custom crewmate role called "Test Role" 
- When a player becomes Test Role, they gain a special ability called `shoot`.
- Using that special ability on a target player will:
    - Kill the target player, whether they are an impostor or a crewmate.
    - Never harm the shooter, making their actions unpredictable and chaotic.

```rust
use amodus::prelude::*;
use amodus::*;

#[role(name = "Test Role", impostor = false)]
pub struct TestRole;

impl TestRole {
    #[on(become)]
    fn on_become(&mut self, player: &Player) {
        println!("BECAME TEST ROLE!!");
    }

    #[ability(default_cooldown = 30, img = "shoot.png")]
    fn shoot(&mut self, target: &Player) {
        target.kill(); // kills
    }
}

#[mod_init]
pub fn init() {
    Roles::register::<TestRole>();
}
```

## Project Structure

- `cli/` - Command-line tool for managing mods
- `sdk/` - Core API and types for mod developers
- `proc/` - Procedural macros (`#[role]`, `#[ability]`, etc.)
- `runtime/` - Runtime version.dll proxy that loads Rust mods into Among Us
- `injector/` - Launcher that is capable of launching Among Us with mods 

## Contributing

Contributions are welcome! Please follow these guidelines:

- Use **tabs** for indentation (not spaces)
- Follow Rust naming conventions
- Add doc comments to all public items so users understand what they do
- Test your changes before submitting

## License

Licensed under either of:

- MIT license ([LICENSE-MIT](LICENSE-MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))

at your option.
