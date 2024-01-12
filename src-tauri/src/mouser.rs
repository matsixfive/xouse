use windows::Win32::UI::Input::KeyboardAndMouse as kbm;
use gilrs::{Gilrs, Button, Event, EventType, ev::Axis};
use gilrs::ff;
use std::thread::sleep;
use anyhow::Result;

fn ease(x: f32) -> f32 {
    x
}

const POLL_TIME_MS: u64 = 5;
const TRIGGER_SPEED_MULT: f32 = 3.0;

pub fn start(window: tauri::Window) -> Result<()> {
    let mut gilrs = Gilrs::new().unwrap();

    let support_ff = gilrs
        .gamepads()
        .filter_map(|(id, gp)| if gp.is_ff_supported() { Some(id) } else { None })
        .collect::<Vec<_>>();

    let duration = ff::Ticks::from_ms(1);
    let effect = ff::EffectBuilder::new()
    .add_effect(ff::BaseEffect {
        kind: ff::BaseEffectType::Weak { magnitude: 60_000 },
        scheduling: ff::Replay { after: ff::Ticks::from_ms(0), play_for: duration, with_delay: ff::Ticks::from_ms(0) },
        ..Default::default()
    })
    .gamepads(&support_ff)
        .finish(&mut gilrs).unwrap();
    effect.set_repeat(ff::Repeat::For(duration)).unwrap();


    let mut l_stick_x = 0.0;
    let mut l_stick_y = 0.0;
    let mut r_stick_x = 0.0;
    let mut r_stick_y = 0.0;

    let mut cur_x_rem = 0.0;
    let mut cur_y_rem = 0.0;

    let mut cur_speed = 6.0;

    loop {
        while let Some(event) = gilrs.next_event()  {
            match event {
                Event { event: EventType::ButtonPressed(button, _), .. } => {
                    match button {
                        Button::South => {
                            unsafe {
                                kbm::mouse_event(kbm::MOUSEEVENTF_LEFTDOWN, 0, 0, 0, 0);
                            }
                            println!("Left Click!");
                        }
                        Button::East => {
                            unsafe {
                                kbm::mouse_event(kbm::MOUSEEVENTF_RIGHTDOWN, 0, 0, 0, 0);
                            }
                            println!("Right Click!");
                        }

                        Button::DPadUp => {
                            cur_speed += 1.0;
                            println!("Speed: {}", cur_speed);
                            effect.play().unwrap();
                            window.emit("speed-up", cur_speed);
                        }
                        Button::DPadDown => {
                            if cur_speed > 1.0 {
                                cur_speed -= 1.0;
                                println!("Speed: {}", cur_speed);
                                effect.play().unwrap();
                                window.emit("speed-down", cur_speed);
                            }
                        }
                        Button::LeftTrigger2 => {
                            cur_speed /= TRIGGER_SPEED_MULT;
                        }
                        Button::RightTrigger2 => {
                            cur_speed *= TRIGGER_SPEED_MULT;
                        }

                        Button::Start => {
                            println!("Escape!");
                        }
                        _  => {}
                    }
                }

                Event { event: EventType::ButtonReleased(button, _), .. } => {
                    match button {
                        Button::South => {
                            unsafe {
                                kbm::mouse_event(kbm::MOUSEEVENTF_LEFTUP, 0, 0, 0, 0);
                            }
                            println!("Left Release!");
                        }
                        Button::East => {
                            unsafe {
                                kbm::mouse_event(kbm::MOUSEEVENTF_RIGHTUP, 0, 0, 0, 0);
                            }
                            println!("Right Release!");
                        }
                        Button::LeftTrigger2 => {
                            cur_speed *= TRIGGER_SPEED_MULT;
                        }
                        Button::RightTrigger2 => {
                            cur_speed /= TRIGGER_SPEED_MULT;
                        }
                        _  => {}
                    }
                }

                Event { event: EventType::AxisChanged(axis, value, ..), ..} => {
                    match axis {
                        Axis::LeftStickX => l_stick_x = value,
                        Axis::LeftStickY => l_stick_y = value,
                        Axis::RightStickX => r_stick_x = value,
                        Axis::RightStickY => r_stick_y = value,
                        _ => (),
                    }
                }
                _ => (),
            };
        }


        let new_x = ease(l_stick_x) * cur_speed * POLL_TIME_MS as f32;
        let new_y = -ease(l_stick_y) * cur_speed * POLL_TIME_MS as f32;
        cur_x_rem += new_x % 1.0;
        cur_y_rem += new_y % 1.0;
        let mut dx = new_x.floor() as i32;
        let mut dy = new_y.floor() as i32;
        if cur_x_rem >= 1.0 {
            dx += 1;
            cur_x_rem -= 1.0;
        }
        if cur_y_rem >= 1.0 {
            dy += 1;
            cur_y_rem -= 1.0;
        }

        // println!("dx: {:2.3}, dy: {:2.3}", new_x, new_y);

        unsafe {kbm::mouse_event(kbm::MOUSEEVENTF_MOVE, dx, dy, 0, 0);}

        sleep(std::time::Duration::from_millis(POLL_TIME_MS));
    }

    Ok(())
}
