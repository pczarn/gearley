local lunajson = require 'lunajson'
local gearley_lua = require 'gearley_lua'
local table = lunajson.decode(gearley_lua.get_default_grammar("a ::= b c d;"))

local function get_keys(t)
    local keyset={}
    local n=0

    for k,v in pairs(t) do
    n=n+1
    keyset[n]=k
    end
    return keyset
end

print(lunajson.encode(get_keys(table)))
