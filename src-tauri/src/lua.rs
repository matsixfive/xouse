use std::sync::{Arc, Mutex};

use tauri::WebviewWindow;

use crate::{
    actions::{ActionFn, ActionInterface, Rumble},
    config::Config,
};

pub fn init_lua<R>(ctx: LuaInterface<R>) -> anyhow::Result<mlua::Lua>
where
    R: Fn() -> Result<(), gilrs::ff::Error> + Send + Sync + Clone + 'static,
{
    let lua = mlua::Lua::new();

    let f = lua.create_function(|_, ()| -> mlua::Result<i32> {
        println!("running 69");
        Ok(69)
    })?;
    lua.globals().set("rust_func", f)?;

    let set_speed = lua.create_function(move |_, speed: f32| -> mlua::Result<()> {
        crate::actions::Action::SetSpeed(speed)
            .down(&ctx.clone().into())
            .map_err(|e| mlua::Error::external(e.to_string()))
    })?;
    lua.globals().set("set_speed", set_speed)?;

    let log = lua.create_function(move |_, input: String| -> mlua::Result<()> {
        log::info!(target: "lua_log", "{}", input);
        Ok(())
    })?;
    lua.globals().set("log", log)?;

    lua.globals().set("number", 1)?;

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
