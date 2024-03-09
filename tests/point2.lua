-- This script defines an addition function
--
-- struct Point2<T>
-- where
--     T: Default,
-- {
--     x: T,
--     y: T,
-- }

-- require './tests/dump'
-- print(Dump(struct))

-- unit tests
assert(struct.meta.ident == "Point2")
assert(struct.meta.attributes[1].name == "luaproc")
assert(struct.meta.attributes[1].inner == '"tests/point2.lua"')
assert(struct.meta.generics.impl == "< T >")
assert(struct.meta.generics.type == "< T >")
-- assert(struct.meta.generics.where == "where T : Copy, T : std :: ops :: Add < Output = T >, T : Default")

for i,f in ipairs(struct.fields) do
    assert(f.name == string.format("x%d", i-1))
    assert(f.type == "T")
end

-- defines the add function that sums all fields

-- build a table with all fields name
local fields = {}
for _,f in ipairs(struct.fields) do
  table.insert(fields, "pt." .. f.name)
end

local inner = table.concat(fields, "+")
print(inner)
code = string.format("pub fn add<T>(pt: &%s<T>) -> T where T: std::ops::Add<Output = T> + Copy + Default { %s }", struct.meta.ident, inner)

print(string.format("===> point2 code: %s", code))