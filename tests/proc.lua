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

 local function impl_default(struct)
   local s = ""
   local where = ""

   -- build each field as a default value depending on its type   
   for i,f in ipairs(struct.fields) do
      s = s .. string.format("%s: %s::default(), ", f.name, f.ty)
   end

   if struct.generics.whe ~= "" then
      where = string.format("%s", struct.generics.where)
   end

   impl = string.format([[
      impl%s Default for %s%s %s {
         fn default() -> Self {
            Self { %s }
         }
      }
   ]], struct.generics.impl, struct.name, struct.generics.type, where, s)
   print(impl)

   return impl
 end

print(dump(struct))

-- impl Default

code = impl_default(struct)

print(impl_default)