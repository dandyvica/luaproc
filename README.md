[![Actions](https://github.com/dandyvica/luaproc/actions/workflows/rust.yml/badge.svg)](https://github.com/dandyvica/luaproc/actions/workflows/rust.yml)

# Build Rust derive macros using Lua

## General idea
Rust procedural macros aren't a simple beast. The ```syn``` and ```proc_macro``` crates aren't easy to grasp, because it unveils the intricacies of the language itself. Creating derive procedural macros using those crates is definitely the way to go.

But it can take time to master, and it's sometimes frustrating.

This crate implements a way to build derive procedural macros using a simple ```Lua``` script. The idea is quite simple:

* Rust populates some internal structures with struct or enum information using the ```syn``` crate
* it then creates a Lua global variable called ```struct``` or ```enum```, which is a Lua table containing all relevant information
* the Lua script is called by Rust
* this script processed the global variable data, and returns a string in the ```code``` Lua global variable which is the string representation of the code you want to inject into Rust
* Rust uses those for compilation

## How ?
Just add ```#[derive(LuaProc)]``` before the desired struct or enum, with the ```#[luaproc("path_to_lua_script.lua")]```.

For example, the following defines a new Default implementation, having all its value to the integer maximum:

```rust
#[derive(Debug, LuaProc)]
#[cfg(target_os = "linux")]
#[luaproc("tests/point1.lua")]
struct Point1 {
    x: u16,
    y: u32,
}

// tests/point1.lua defines a specific Default impl different from the standard one
let pt1 = Point1::default();
assert_eq!(pt1.x, u16::MAX);
assert_eq!(pt1.y, u32::MAX);
```

and the corresponding Lua script:

```lua
local fn = string.format(" Self { x: %s::MAX, y: %s::MAX }", struct.fields[1].type, struct.fields[2].type)
local impl = [[
impl Default for %s { 
    fn default() -> Self { %s } 
}
]]

-- return code to Rust
code = string.format(impl, struct.meta.ident, fn)
```



## Caveats
The main drawback is that you have to recompile the Rust file having the ```LuaProc``` attribute whenever the Lua script is modified. One solution is:

```
$ cargo clean -p <my package> && cargo test
```

Also, beware it's a first version and not fully tested with all the bells and whistles 😜 which can decorate structs or enums 😃

Only for Lua 5.4.

Kudos to the ```mlua``` crate which is very powerful 👍

## Todo
* comprehensive tests
* definition of all fields of injected Lua tables

## Examples
The ```integration_test.rs``` file contains several examples with the corresponding Lua scripts.
