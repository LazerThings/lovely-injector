use libloading::Library;
use lovely_core::sys::LuaLib;
use std::sync::LazyLock;

// This path might need adjustment for Apple Arcade
static LUA_LIB: LazyLock<Library> = LazyLock::new(|| unsafe {
    // Try several possible locations for the Lua library
    Library::new("../Frameworks/Lua.framework/Versions/A/Lua")
        .or_else(|_| Library::new("libluajit.dylib"))
        .or_else(|_| Library::new("liblua.dylib"))
        .unwrap()
});

pub unsafe fn get_lualib() -> LuaLib {
    LuaLib {
        lua_call: *LUA_LIB.get(b"lua_call").unwrap(),
        lua_pcall: *LUA_LIB.get(b"lua_pcall").unwrap(),
        lua_getfield: *LUA_LIB.get(b"lua_getfield").unwrap(),
        lua_setfield: *LUA_LIB.get(b"lua_setfield").unwrap(),
        lua_gettop: *LUA_LIB.get(b"lua_gettop").unwrap(),
        lua_settop: *LUA_LIB.get(b"lua_settop").unwrap(),
        lua_pushvalue: *LUA_LIB.get(b"lua_pushvalue").unwrap(),
        lua_pushcclosure: *LUA_LIB.get(b"lua_pushcclosure").unwrap(),
        lua_tolstring: *LUA_LIB.get(b"lua_tolstring").unwrap(),
    }
}