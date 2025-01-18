use anyhow::Result;
use gilrs::ff;
use gilrs::{ev::Axis, Event, EventType, Gilrs};
use std::mem::drop;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tauri::Emitter;
use windows::Win32::UI::Input::KeyboardAndMouse as kbm;

use crate::actions::{Action, SimpleActionFn, UpDownActionFn};
use crate::config::Config;

fn ease(x: f32) -> f32 {
    x
}

#[derive(Debug, Clone, Copy, Default)]
struct Vec2<T> {
    pub x: T,
    pub y: T,
}

const POLL_TIME_MS: u64 = 1;
const UNIT_MULTIPLIER: f32 = 0.02;

pub fn start(window: tauri::WebviewWindow, config_mx: Arc<Mutex<Config>>) -> Result<()> {
    let mut gilrs = Gilrs::new().unwrap();

    // print all connected gamepads
    println!("Connected gamepads:");
    for (id, gp) in gilrs.gamepads() {
        println!("Connected gamepad id: {}", id);
        println!("Gamepad: {}", gp.name());
    }
    println!("End of connected gamepads");

    let support_ff = gilrs
        .gamepads()
        .filter_map(|(id, gp)| if gp.is_ff_supported() { Some(id) } else { None })
        .collect::<Vec<_>>();

    dbg!(&support_ff);

    let duration = ff::Ticks::from_ms(1);
    let effect = ff::EffectBuilder::new()
        .add_effect(ff::BaseEffect {
            kind: ff::BaseEffectType::Weak {
                magnitude: u16::MAX,
            },
            scheduling: ff::Replay {
                after: ff::Ticks::from_ms(0),
                play_for: duration,
                with_delay: ff::Ticks::from_ms(0),
            },
            ..Default::default()
        })
        .gamepads(&support_ff)
        .finish(&mut gilrs)
        .unwrap();
    effect.set_repeat(ff::Repeat::For(duration)).unwrap();

    let rumble: Arc<Box<dyn Fn() + Send + Sync>> = Arc::new(Box::new(move || {
        println!("rumbling");
        let _ = effect.play();
    }));

    let mut l_stick = Vec2::<f32> { x: 0.0, y: 0.0 };
    let mut r_stick = Vec2::<f32> { x: 0.0, y: 0.0 };

    let mut remainder = Vec2::<f32> { x: 0.0, y: 0.0 };

    crate::lock!();
    let config = config_mx.lock().unwrap();
    // window.emit("speed_change", config.speed)?;
    drop(config);

    let lua_ctx = crate::lua::init_lua().unwrap();

    loop {
        crate::lock!();
        let mut config = config_mx.lock().unwrap();

        match (gilrs.gamepads().next(), config.gamepad_id) {
            (Some((gamepad_id, gamepad)), None) => {
                config.gamepad_id = Some(gamepad_id);
                println!("Connected gamepad id: {}", gamepad_id);
                println!("Gamepad: {:?}", gamepad);
            }
            (None, _) => {
                config.gamepad_id = None;

                l_stick = Vec2::default();
                r_stick = Vec2::default();
                remainder = Vec2::default();
                println!("No gamepad connected");
                thread::sleep(Duration::from_millis(1000));
            }
            _ => {}
        }

        while let Some(event) = gilrs.next_event() {
            match config.gamepad_id {
                Some(id) => {
                    if event.id != id {
                        continue;
                    }
                }
                None => {
                    config.gamepad_id = Some(event.id);
                }
            }

            match event {
                Event {
                    event: EventType::ButtonPressed(button, _),
                    ..
                } => {
                    let actions = config.actions[button].clone();
                    // dbg!(&actions, &button);

                    let action_interface = crate::actions::ActionInterface {
                        config: config_mx.clone(),
                        window: window.clone(),
                        lua: &lua_ctx,
                        rumble: None,
                    };

                    drop(config);
                    for action in actions {
                        if let Err(e) = match action {
                            Action::Simple(action) => action.call(&action_interface),
                            Action::UpDown(action) => action.down(&action_interface),
                        } {
                            eprintln!("Error: {:?}", e);
                        }
                    }
                    config = config_mx.lock().unwrap();
                }

                Event {
                    event: EventType::ButtonReleased(button, _),
                    ..
                } => {
                    let actions = config.actions[button].clone();
                    // dbg!(&actions, &button);

                    let action_interface = crate::actions::ActionInterface {
                        config: config_mx.clone(),
                        window: window.clone(),
                        lua: &lua_ctx,
                        rumble: None,
                    };

                    drop(config);
                    for action in actions {
                        if let Err(e) = match action {
                            Action::UpDown(action) => action.up(&action_interface),
                            _ => Ok(()),
                        } {
                            eprintln!("Error: {:?}", e);
                        }
                    }
                    config = config_mx.lock().unwrap();
                }
                Event {
                    event: EventType::AxisChanged(axis, value, ..),
                    ..
                } => match axis {
                    Axis::LeftStickX => l_stick.x = value,
                    Axis::LeftStickY => l_stick.y = value,
                    Axis::RightStickX => r_stick.x = value,
                    Axis::RightStickY => r_stick.y = value,
                    _ => (),
                },
                _ => (),
            };
        }

        if config.gamepad_id.is_none() {
            continue;
        }

        let new_x = ease(l_stick.x)
            * config.speed
            * config.speed_mult
            * UNIT_MULTIPLIER
            * POLL_TIME_MS as f32
            + remainder.x;
        let new_y = -ease(l_stick.y)
            * config.speed
            * config.speed_mult
            * UNIT_MULTIPLIER
            * POLL_TIME_MS as f32
            + remainder.y;
        let (dx, x_rem) = integer_and_fractional(new_x);
        let (dy, y_rem) = integer_and_fractional(new_y);
        remainder.x = x_rem;
        remainder.y = y_rem;

        if (dx != 0) || (dy != 0) {
            unsafe {
                kbm::mouse_event(kbm::MOUSEEVENTF_MOVE, dx, dy, 0, 0);
            }
        }

        unsafe {
            kbm::mouse_event(
                kbm::MOUSEEVENTF_WHEEL,
                0,
                0,
                integer_and_fractional(r_stick.y * 3.2 * config.speed_mult).0,
                0,
            );
            kbm::mouse_event(
                kbm::MOUSEEVENTF_HWHEEL,
                0,
                0,
                integer_and_fractional(r_stick.x * 3.2 * config.speed_mult).0,
                0,
            );
        }

        drop(config);
        thread::sleep(Duration::from_millis(POLL_TIME_MS));
    }
}

fn integer_and_fractional(num: f32) -> (i32, f32) {
    if num >= 0.0 {
        // positive
        return (num.floor() as i32, num % 1.0);
    } else {
        // negative
        return (num.ceil() as i32, num % 1.0);
    }
}
