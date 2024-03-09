--  print(dump(enum))

-- unit tests
assert(enum.meta.ident == "Choice")
assert(enum.meta.attributes[1].name == "luaproc")
assert(enum.meta.attributes[1].inner == '"tests/enum_2.lua"')
assert(enum.meta.attributes[2].name == "repr")
assert(enum.meta.attributes[2].inner == "u8")

assert(enum.variants[1].discriminant == "0")
assert(enum.variants[1].name == "Ok")
assert(enum.variants[1].attributes[1].name == "luaproc")
assert(enum.variants[1].attributes[1].inner == "foo")

assert(enum.variants[2].discriminant == "1 + 2")
assert(enum.variants[2].name == "Nok")