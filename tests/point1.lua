-- This Lua script implements a simple default point struct
--
-- struct struct {
--     x: u16,
--     y: u32,
-- }

-- unit tests
assert(struct.meta.ident == "Point1")
assert(struct.meta.attributes[1].name == "cfg")
assert(struct.meta.attributes[1].inner == 'target_os = "linux"')
assert(struct.meta.attributes[2].name == "luaproc")
assert(struct.meta.attributes[2].inner == '"tests/point1.lua"')
assert(struct.meta.generics.impl == "")
assert(struct.meta.generics.type == "")
assert(struct.meta.generics.where == "")

assert(struct.fields[1].name == "x")
assert(struct.fields[1].type == "u16")
assert(struct.fields[2].name == "y")
assert(struct.fields[2].type == "u32")

-- generate code this code:
--
-- impl Default for struct {
--     fn default() -> Self {
--         Self { x: u16::MAX, y: u32:MAX }
--     }
-- }

local fn = string.format(" Self { x: %s::MAX, y: %s::MAX }", struct.fields[1].type, struct.fields[2].type)
local impl = [[
impl Default for %s { 
    fn default() -> Self { %s } 
}
]]

-- return code to Rust
code = string.format(impl, struct.meta.ident, fn)

print(string.format("===> point1 code: %s", code))