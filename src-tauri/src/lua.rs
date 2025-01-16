pub fn test_lua() -> anyhow::Result<()> {
    let lua = mlua::Lua::new();
    let f = lua.create_function(|_, ()| -> mlua::Result<()> {
        panic!("test panic");
    })?;
    lua.globals().set("rust_func", f)?;

    let _ = lua
        .load(
            r#"
    local status, err = pcall(rust_func)
    print(err) -- prints: test panic
    error(err) -- propagate panic
"#,
        )
        .exec();

    unreachable!()
}
