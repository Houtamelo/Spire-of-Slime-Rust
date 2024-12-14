# Update cycle
This repository **is not** used by the Spire team, this clone of the main project is kept here merely for transparency.

This repository is kept updated with the main project with a minimum delay of 30 days, this means that updates to the game that are younger than 30 days old will not be reflected here.
This delay is in place to make sure players can't simply read the source code to find secrets/easter-eggs that were introduced in recent updates, it's essentially a "spoiler-prevention" measure.

# Building from source
This setup assumes you're familiar with Godot 4.3, as well as using Rust.

### Requirements
- Godot Editor 4.3
- A custom compiler of the Rust programming language: [Rust Unchained](https://github.com/Rust-Unchained/rust_unchained).
  The project **will not** compile with any version of the standard compiler.

### Building
- Clone this repository.
- Run `cargo build`, then `cargo build --release` in the root of this repository.
- Open the project using the Godot Editor. There should be no errors in the editor output.
- Build the project normally using godot, choose the export template that correponds to the platform you want to build for.


# Licensing

Some files in this repository have licensing, depending on their types.

### Code - Files with extension ".rs" or ".gd"
Code files are licensed under AGPL 3.0, the license is in the file "LICENSE_CODE_ONLY.md".

### Godot Resources - Files with extension ".tscn" or ".tres"
Godot resources are licensed under AGPL 3.0, the license is in the file "LICENSE_CODE_ONLY.md".

Note that the license **does not** include the assets that those resources require.

### Everything else
No license is provided to any other files (such as Image or Audio), you may contact the author of each file you wish to acquire a license to for.

If you don't know who the author of a specific file is, contact me(Houtamelo).
