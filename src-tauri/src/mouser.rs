use anyhow::Result;
use gilrs::ff;
use gilrs::{ev::Axis, Event, EventType, Gilrs};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tauri::Emitter;
use windows::Win32::UI::Input::KeyboardAndMouse as kbm;

use crate::actions::ActionType;
use crate::actions2::{Action, SimpleActionFn, UpDownActionFn};
use crate::config::Config;

fn ease(x: f32) -> f32 {
    x
}

struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Default for Vec2<T>
where
    T: Default,
{
    fn default() -> Self {
        Self {
            x: T::default(),
            y: T::default(),
        }
    }
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

    let config = config_mx.lock().unwrap();
    window.emit("speed_change", config.speed)?;
    std::mem::drop(config);

    loop {
        let mut config = config_mx.lock().unwrap();
        // match config.gamepad_id {
        //     Some(id) => {
        //         config.gamepad_id = gilrs.connected_gamepad(id).map(|gp| gp.id());
        //     }
        //     None => {}
        // }
        // if gilrs.gamepads().count() == 0 {
        //     println!("None");
        //     config.gamepad_id = None;
        //
        //     l_stick = Vec2::<f32> { x: 0.0, y: 0.0 };
        //     r_stick = Vec2::<f32> { x: 0.0, y: 0.0 };
        //     remainder = Vec2::<f32> { x: 0.0, y: 0.0 };
        //
        //     continue;
        // } else {
        //     println!("Some");
        // }

        match (gilrs.gamepads().next(), config.gamepad_id) {
            (Some((conn_id, _)), None) => {
                config.gamepad_id = Some(conn_id);
                println!("Connected gamepad id: {}", conn_id);
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
                    std::mem::drop(config);
                    dbg!(&actions, &button);

                    let action_interface = crate::actions2::ActionInterface {
                        config: config_mx.clone(),
                        window: window.clone(),
                        rumble: None,
                    };
                    for action in actions {
                        match action {
                            Action::Simple(action) => action.call(&action_interface),
                            Action::UpDown(action) => action.down(&action_interface),
                        }
                    }
                    config = config_mx.lock().unwrap();
                }

                Event {
                    event: EventType::ButtonReleased(button, _),
                    ..
                } => {
                    let actions = config.actions[button].clone();
                    std::mem::drop(config);
                    dbg!(&actions, &button);

                    let action_interface = crate::actions2::ActionInterface {
                        config: config_mx.clone(),
                        window: window.clone(),
                        rumble: None,
                    };
                    for action in actions {
                        match action {
                            Action::UpDown(action) => action.up(&action_interface),
                            _ => (),
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
        let (dx, x_rem) = head_and_tail(new_x);
        let (dy, y_rem) = head_and_tail(new_y);
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
                head_and_tail(r_stick.y * 3.2 * config.speed_mult).0,
                0,
            );
            kbm::mouse_event(
                kbm::MOUSEEVENTF_HWHEEL,
                0,
                0,
                head_and_tail(r_stick.x * 3.2 * config.speed_mult).0,
                0,
            );
        }

        std::mem::drop(config);
        thread::sleep(Duration::from_millis(POLL_TIME_MS));
    }
}

fn head_and_tail(num: f32) -> (i32, f32) {
    if num >= 0.0 {
        return (num.floor() as i32, num % 1.0);
    }
    (num.ceil() as i32, num % 1.0)
}
