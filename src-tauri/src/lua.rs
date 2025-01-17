pub fn init_lua() -> anyhow::Result<mlua::Lua> {
    let lua = mlua::Lua::new();

    let f = lua.create_function(|_, ()| -> mlua::Result<i32> {
        println!("running 69");
        Ok(69)
    })?;

    lua.globals().set("rust_func", f)?;

    Ok(lua)
}
