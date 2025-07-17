
# gmsv\_rembuild

A Garry's Mod **binary module** that removes `_build*` suffixes from `game.GetMap()`.

---

## 🚀 What is this?

This module ensures that map names returned by `game.GetMap()` are stripped of any `_build*` suffix (e.g., `gm_construct_build23` becomes `gm_construct`).
This helps maintain compatibility across multiple map versions for **permaprops**, **door configs**, and other server-side addons.

---

## 📦 How to Use

1. **Install the module**:

   * Place the compiled binary in your Garry's Mod `garrysmod/lua/bin/` folder.

2. **Load it early**:

   * Create a file in:

     ```
     /garrysmod/lua/autorun/server/rembuild_loader.lua
     ```
   * Inside that file, add:

     ```lua
     require("rembuild")
     ```

3. ✅ Make sure it loads **before any addon** that relies on `game.GetMap()`!

> ⚠️ This is a **server-side module** only. Installing on server clients will have also map with build.

---

## 🧠 Why is this useful?

Many maps release with version suffixes like `_build42`, which break compatibility with:

* Saved props (permaprops)
* Door/area configurations
* Other map-specific data

This module normalizes map names so configs work across **all versions** of the same base map.

---

## 🔧 Building from Source
You can download module from [releases](https://github.com/Valiant-Game/gmsv_rembuild/releases). Or build own.

1. Install Rust and set to nightly:

   ```bash
   rustup default nightly-2024-07-19
   ```

2. Install required toolchains:

   * **Windows**: C++ build tools (e.g., MSVC)
   * **Linux**: `gcc`, `g++`, `build-essential`

3. Follow build instructions for GMod Rust binary modules:
   [https://wiki.facepunch.com/gmod/Creating\_Binary\_Modules:\_Rust](https://wiki.facepunch.com/gmod/Creating_Binary_Modules:_Rust)
