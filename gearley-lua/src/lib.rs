use cfg::Cfg;
use mlua::prelude::*;

#[mlua::lua_module]
fn gearley_lua(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    exports.set("get_default_grammar", lua.create_function(get_default_grammar)?)?;
    Ok(exports)
}

use cfg_load::CfgLoadExt;
use gearley_default_grammar::DefaultGrammar;
use miniserde::json;

pub fn get_default_grammar(_: &Lua, bnf: String) -> LuaResult<String> {
    match Cfg::load(&bnf) {
        Ok(cfg) => {
            let default_grammar = DefaultGrammar::from_grammar(cfg);
            Ok(json::to_string(&default_grammar))
        }
        Err(err) => Err(err.into_lua_err())
    }
}
