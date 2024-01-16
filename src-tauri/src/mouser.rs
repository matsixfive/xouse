use anyhow::Result;
use gilrs::ff;
use gilrs::{ev::Axis, Event, EventType, Gilrs};
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use windows::Win32::UI::Input::KeyboardAndMouse as kbm;

use crate::actions::ActionType;
use crate::config::Config;

fn ease(x: f32) -> f32 {
    x
}

const POLL_TIME_MS: u64 = 5;
const UNIT_MULTIPLIER: f32 = 0.02;

pub fn start(window: tauri::Window, config_mx: Arc<Mutex<Config>>) -> Result<()> {
    let mut gilrs = Gilrs::new().unwrap();

    let support_ff = gilrs
        .gamepads()
        .filter_map(|(id, gp)| if gp.is_ff_supported() { Some(id) } else { None })
        .collect::<Vec<_>>();

    let duration = ff::Ticks::from_ms(1);
    let effect = ff::EffectBuilder::new()
        .add_effect(ff::BaseEffect {
            kind: ff::BaseEffectType::Weak { magnitude: u16::MAX / 2 },
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

    let rumble: Arc<Box<dyn Fn() + Send + Sync>> = Arc::new(Box::new(move || {let _ = effect.play();}));

    let mut l_stick_x = 0.0;
    let mut l_stick_y = 0.0;
    let mut r_stick_x = 0.0;
    let mut r_stick_y = 0.0;

    // remainders from last mouse move
    let mut rem_x = 0.0;
    let mut rem_y = 0.0;

    let config = config_mx.lock().unwrap();
    window.emit("speed-change", config.speed)?;
    std::mem::drop(config);

    loop {
        let mut config = config_mx.lock().unwrap();
        while let Some(event) = gilrs.next_event() {
            match event {
                Event {
                    event: EventType::ButtonPressed(button, _),
                    ..
                } => {
                    let action = config.actions.0.get(&button).cloned();
                    std::mem::drop(config);
                    match action {
                        Some(action) => match action.into() {
                            ActionType::Simple(f) => f(config_mx.clone(), &window, rumble.clone()),
                            ActionType::UpDown((f, _)) => f(config_mx.clone(), &window, rumble.clone()),
                        },
                        None => {}
                    }
                    config = config_mx.lock().unwrap();
                }

                Event {
                    event: EventType::ButtonReleased(button, _),
                    ..
                } => {
                    let action = config.actions.0.get(&button).cloned();
                    std::mem::drop(config);
                    match action {
                        Some(action) => match action.into() {
                            ActionType::Simple(_) => {}
                            ActionType::UpDown((_, f)) => f(config_mx.clone(), &window, rumble.clone()),
                        },
                        None => {}
                    }
                    config = config_mx.lock().unwrap();
                }
                Event {
                    event: EventType::AxisChanged(axis, value, ..),
                    ..
                } => match axis {
                    Axis::LeftStickX => l_stick_x = value,
                    Axis::LeftStickY => l_stick_y = value,
                    Axis::RightStickX => r_stick_x = value,
                    Axis::RightStickY => r_stick_y = value,
                    _ => (),
                },
                _ => (),
            };
        }

        let new_x = ease(l_stick_x)
            * config.speed
            * config.speed_mult
            * UNIT_MULTIPLIER
            * POLL_TIME_MS as f32
            + rem_x;
        let new_y = -ease(l_stick_y)
            * config.speed
            * config.speed_mult
            * UNIT_MULTIPLIER
            * POLL_TIME_MS as f32
            + rem_y;
        let (dx, x_rem) = head_and_tail(new_x);
        let (dy, y_rem) = head_and_tail(new_y);
        rem_x = x_rem;
        rem_y = y_rem;

        if (dx != 0) || (dy != 0) {
            unsafe {
                kbm::mouse_event(kbm::MOUSEEVENTF_MOVE, dx, dy, 0, 0);
            }
        }

        std::mem::drop(config);
        sleep(std::time::Duration::from_millis(POLL_TIME_MS));
    }
}

fn head_and_tail(num: f32) -> (i32, f32) {
    if num >= 0.0 {
        return (num.floor() as i32, num % 1.0);
    }
    (num.ceil() as i32, num % 1.0)
}
