use gmod::lua::*;
use std::sync::Mutex;
use std::sync::OnceLock;

#[macro_use]
extern crate gmod;

static OLD_MAP: OnceLock<Mutex<String>> = OnceLock::new();

unsafe extern "C-unwind" fn GetMap(lua: State) -> i32 {
    let old_map_guard = OLD_MAP.get().unwrap().lock().unwrap();
    lua.push_string(old_map_guard.split("_build").next().unwrap_or("unknown"));
    1
}

#[gmod13_open]
unsafe fn gmod13_open(lua: State) -> i32 {
    OLD_MAP.get_or_init(|| Mutex::new(String::from("gm_construct")));

    lua.get_global(lua_string!("game"));
    if lua.is_nil(-1) {
        lua.pop();
        lua.new_table();
    }

    lua.get_field(-1, lua_string!("GetMap"));
    if lua.is_function(-1) {
        lua.pcall(0, 1, 0);
        let map_str = lua.check_string(-1).to_string();
        *OLD_MAP.get().unwrap().lock().unwrap() = map_str;
        lua.pop();
    } else {
        lua.pop();
    }

    lua.push_function(GetMap);
    lua.set_field(-2, lua_string!("GetMap"));
    lua.set_global(lua_string!("game"));

    0
}

#[gmod13_close]
fn gmod13_close(_lua: State) -> i32 {
    0
}
