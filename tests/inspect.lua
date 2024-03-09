-- print out global variables for luaproc

function dump(o)
    if type(o) == 'table' then
       local s = '{ '
       for k,v in pairs(o) do
          if type(k) ~= 'number' then k = '"'..k..'"' end
          s = s .. '['..k..'] = ' .. dump(v) .. ','
       end
       return s .. '} '
    else
       return tostring(o)
    end
 end

-- if meta then
--     print(string.format("ident ====> %s", meta.ident))
--     for i,attr in ipairs(meta.attributes) do
--         print(string.format("   attribute %d ====> '%s':'%s'", i, attr.name, attr.inner))
--     end
--     print(string.format("impl generics ====> %s", meta.generics.impl))
--     print(string.format("type generics ====> %s", meta.generics.type))
--     print(string.format("where clause ====> %s", meta.generics.where))
-- end

-- if struct then
--     print(string.format("struct ident ====> %s", struct.name))
-- end

print(dump(struct))



code = ""

