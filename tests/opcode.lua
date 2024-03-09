
-- enum OpCode {
--     Query = 0, //[RFC1035]
--     IQuery = 1, // (Inverse Query, OBSOLETE)	[RFC3425]
--     Status = 2, // [RFC1035]
--     Unassigned = 3,
--     Notify = 4, // [RFC1996]
--     Update = 5, // [RFC2136]
--     DOS = 6,    // DNS Stateful Operations (DSO)	[RFC8490]
-- }

-- unit tests
assert(enum.meta.ident == "OpCode")
assert(enum.meta.attributes[1].name == "luaproc")
assert(enum.meta.attributes[1].inner == '"tests/opcode.lua"')
assert(enum.meta.attributes[2].name == "repr")
assert(enum.meta.attributes[2].inner == "u8")

assert(enum.variants[1].discriminant == "0")
assert(enum.variants[1].name == "Query")

assert(enum.variants[2].discriminant == "1")
assert(enum.variants[2].name == "IQuery")

assert(enum.variants[3].discriminant == "2")
assert(enum.variants[3].name == "Status")

assert(enum.variants[4].discriminant == "3")
assert(enum.variants[4].name == "Unassigned")

assert(enum.variants[5].discriminant == "4")
assert(enum.variants[5].name == "Notify")

assert(enum.variants[6].discriminant == "5")
assert(enum.variants[6].name == "Update")

assert(enum.variants[7].discriminant == "6")
assert(enum.variants[7].name == "DOS")

------------------------------------------------------------------------
-- implement FromStr
------------------------------------------------------------------------
local from_str = [[
impl std::str::FromStr for OpCode {
    type Err = String;

    // Required method
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            %s
            _ => Err(format!("{s} is not a valid variant for OpCode"))
        }
    }
}
]]

-- build match arms
local inner1 = ""

for _,v in ipairs(enum.variants) do
    inner1 = inner1 .. string.format('"%s" => Ok(OpCode::%s), \n', v.name, v.name)
end

------------------------------------------------------------------------
-- implement TryFrom
------------------------------------------------------------------------
local try_from = [[
impl std::convert::TryFrom<%s> for OpCode {
    type Error = String;

    fn try_from(value: %s) -> Result<Self, Self::Error> {
        match value {
            %s
            _ => Err(format!("no variant corresponding to value {value}"))
        }        
    }
}
]]

-- build match arms
local inner2 = ""

for _,v in ipairs(enum.variants) do
    inner2 = inner2 .. string.format('%s => Ok(OpCode::%s), \n', v.discriminant, v.name)
end

-- get the type associated to the #repr attribute
local type = nil
for _,a in ipairs(enum.meta.attributes) do
    if a.name == "repr" then
        type = a.inner
    end
end

-- return code to Rust compiler
code = string.format(from_str, inner1) .. string.format(try_from, type, type, inner2)

print(code)

