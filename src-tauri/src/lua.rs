pub fn test_lua() -> anyhow::Result<()> {
    let lua = mlua::Lua::new();
    let f = lua.create_function(|_, ()| -> mlua::Result<i32> {
        println!("running 69");
        Ok(69)
    })?;
    lua.globals().set("rust_func", f)?;

    let _ = lua
        .load(
            r#"
            local num = rust_func()
            print(num)
"#,
        )
        .exec();
    Ok(())
}
