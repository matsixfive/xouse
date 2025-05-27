use std::sync::{Arc, Mutex};

use mlua::Value;
use tauri::WebviewWindow;

use crate::{
    actions::{Action, ActionFn, ActionInterface, Rumble},
    config::Config,
};

pub fn init_lua<R>(ctx: LuaInterface<R>) -> anyhow::Result<mlua::Lua>
where
    R: Fn() -> Result<(), gilrs::ff::Error> + Send + Sync + Clone + 'static,
{
    let lua = mlua::Lua::new();

    let metatable = lua.create_table()?;

    // __index: When Lua reads a global
    let index_config = ctx.config.clone();
    metatable.set(
        "__index",
        lua.create_function(move |_, (_table, key): (mlua::Table, mlua::String)| {
            match key.to_str() {
                Ok("speed") => {
                    let speed = index_config.lock().unwrap().speed;
                    Ok(mlua::Value::Number(speed.into()))
                }
                _ => Ok(mlua::Value::Nil),
            }
        })?,
    )?;

    // __newindex: When Lua assigns to a global
    let newindex_config = ctx.config.clone();
    metatable.set("__newindex", lua.create_function(
        move |_, (table, key, value): (mlua::Table, mlua::String, mlua::Value)| {
            match key.to_str() {
                Ok("speed") => {
                    match value {
                        Value::Number(f) => {
                            let config = &mut newindex_config.lock().unwrap();
                            config.speed = f as f32;
                            println!("speed assigned to {}", f);
                        }
                        Value::Integer(i) => {
                            let config = &mut newindex_config.lock().unwrap();
                            config.speed = i as f32;
                            println!("speed assigned to {}", i);
                        }
                        _ => {
                            log::warn!(target: "lua_callback_log", "Invalid type for speed: {:?}", value);
                            return Err(mlua::Error::FromLuaConversionError {
                                from: value.type_name(),
                                to: "number",
                                message: Some("Expected a number or integer for speed".to_string()),
                            });
                        }
                    }
                }
                _ => {
                    table.raw_set(key, value)?;
                }
            }
            Ok(())
        },
    )?)?;

    lua.globals().set_metatable(Some(metatable));

    let log = lua.create_function(move |_, input: Option<String>| -> mlua::Result<()> {
        match input {
            Some(s) => log::info!(target: "lua_log", "{}", s),
            None => log::info!(target: "lua_log", "log called with no argument or nil"),
        }
        Ok(())
    })?;
    lua.globals().set("consolelog", log)?;

    let rumble = lua.create_function(move |_, _: mlua::Value| -> mlua::Result<()> {
        _ = Action::Rumble.down(&ctx.clone().into());
        _ = Action::Rumble.up(&ctx.clone().into());
        Ok(())
    })?;
    lua.globals().set("rumble", rumble)?;


    Ok(lua)
}

#[derive(Clone)]
pub struct LuaInterface<R>
where
    R: Fn() -> Result<(), gilrs::ff::Error> + Send + Sync + Clone + 'static,
{
    pub config: Arc<Mutex<Config>>,
    pub window: WebviewWindow,
    pub rumble: Option<Rumble<R>>,
}

impl<R> From<LuaInterface<R>> for ActionInterface<'_, R>
where
    R: Fn() -> Result<(), gilrs::ff::Error> + Send + Sync + Clone + 'static,
{
    fn from(val: LuaInterface<R>) -> Self {
        ActionInterface {
            config: val.config,
            window: val.window,
            rumble: val.rumble,
            lua: None,
        }
    }
}
