use egui_macroquad::egui;
use std::collections::VecDeque;

use macroquad::{
    hash,
    prelude::*,
    ui::{root_ui, widgets, Ui},
};

const MAX_HISTORY: usize = 240;
const BG: Color = Color::new(0.00, 0.0, 0.1, 0.1);

fn lerp(from: f32, to: f32, t: f32) -> f32 {
    from * (1.0 - t) + to * t
}

enum Function {
    Exact,
    Lerp { factor: f32 },
    DamperBad { damper: f32 },
    DamperExact { half_life: f32 },
    DamperExact2 { rate: f32 },
}

impl Function {
    fn execute(&self, from: f32, to: f32, dt: f32) -> f32 {
        match self {
            Function::Exact => to,
            Function::Lerp { factor } => lerp(from, to, *factor),
            Function::DamperBad { damper } => lerp(from, to, f32::clamp(damper * dt, 0.0, 1.0)),
            Function::DamperExact { half_life } => lerp(
                from,
                to,
                1.0 - f32::exp(-(f32::ln(2.0) * dt) / (half_life + 1e-5f32)),
            ),
            Function::DamperExact2 { rate } => lerp(to, from, f32::exp2(-rate * dt)),
        }
    }
}

impl Default for Function {
    fn default() -> Self {
        Self::Lerp { factor: 0.5 }
    }
}

impl Function {
    fn ui(&mut self, ui: &mut Ui) {
        let name = match self {
            Function::Exact => "Exact",
            Function::Lerp { .. } => "Lerp",
            Function::DamperBad { .. } => "Damper Bad",
            Function::DamperExact { .. } => "Damper Exact",
            Function::DamperExact2 { .. } => "Damper Exact 2",
        };
        ui.label(None, name);
        ui.separator();
        if ui.button(None, "Exact") {
            *self = Function::Exact;
        }

        if ui.button(None, "Lerp") {
            *self = Function::Lerp { factor: 0.5 };
        }

        if ui.button(None, "DamperBad") {
            *self = Function::DamperBad { damper: 30.0 };
        }

        if ui.button(None, "DamperExact") {
            *self = Function::DamperExact { half_life: 1.0 };
        }

        if ui.button(None, "DamperExact 2") {
            *self = Function::DamperExact2 { rate: 1.0 };
        }

        match self {
            Function::Exact => {}
            Function::Lerp { factor } => {
                ui.slider(hash!(), "Lerp factor", 0.01..1.0, factor);
            }
            Function::DamperBad { damper } => {
                ui.slider(hash!(), "Damper", 1.0..50.0, damper);
            }
            Function::DamperExact { half_life } => {
                ui.slider(hash!(), "Half life", 0.01..1.0, half_life);
            }
            Function::DamperExact2 { rate } => {
                ui.slider(hash!(), "rate", 0.01..50.0, rate);
            }
        }
    }
}

enum Simulation {
    Live,
    Compare { settings: CompareSettings },
}

pub struct CompareSettings {
    fuction: Function,
    first_framerate: f32,
    second_framerate: f32,
    simulating_time: f32,
}
impl Default for CompareSettings {
    fn default() -> Self {
        Self {
            fuction: Function::default(),
            first_framerate: 60.0,
            second_framerate: 15.0,
            simulating_time: 2.0,
        }
    }
}

#[macroquad::main("BasicShapes")]
async fn main() {
    let center = screen_height() / 2.0;

    let mut goal = center;
    let mut value = goal;

    let mut mode = Function::Lerp { factor: 0.5 };
    let mut target_fps = 60.0;

    let mut history = VecDeque::from([goal; MAX_HISTORY]);

    let mut sim = Simulation::Live;

    // let mut last_time = get_time();

    let mut wait_time = 0.0;

    loop {
        let target_dt = 1.0 / target_fps;
        let dt = get_frame_time();

        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("egui ❤ macroquad")
                .frame(
                    egui::Frame::window(&egui::Style::default()).shadow(egui::epaint::Shadow::NONE),
                )
                .show(egui_ctx, |ui| {
                    ui.label("Test");
                    ui.label("Test");
                    ui.label("Test");
                });
        });
        widgets::Window::new(hash!(), vec2(0., 0.), vec2(300., 400.))
            .label("Settings")
            .titlebar(true)
            .movable(true)
            .ui(&mut root_ui(), |ui| {
                if ui.button(None, "Live") {
                    sim = Simulation::Live
                }

                if ui.button(None, "Compare") {
                    sim = Simulation::Compare {
                        settings: CompareSettings::default(),
                    };
                }
                ui.separator();

                match sim {
                    Simulation::Compare {
                        ref mut settings, ..
                    } => {
                        mode.ui(ui);
                        ui.slider(hash!(), "Sim time", 0.1..5.0, &mut settings.simulating_time);
                        ui.slider(
                            hash!(),
                            "Frame rate 1",
                            10.0..240.0,
                            &mut settings.first_framerate,
                        );
                        ui.slider(
                            hash!(),
                            "Frame rate 2",
                            10.0..240.0,
                            &mut settings.second_framerate,
                        );
                    }
                    Simulation::Live => {
                        mode.ui(ui);
                        ui.slider(hash!(), "Target fps", 10.0..240.0, &mut target_fps);
                    }
                }
                ui.label(None, &format!("FPS {}", 1.0 / dt));
            });

        match sim {
            Simulation::Live => {
                // let time = get_time();
                // let dt = (time - last_time) as f32;
                // last_time = time;
                //
                // if wait_time > 0.0 {
                //     wait_time -= dt;
                //     continue;
                // }
                //
                // if dt < target_dt {
                //     wait_time = target_dt;
                // }

                clear_background(BG);

                // sleep(Duration::from_millis((target_dt * 1000.0) as u64));

                // if dt < target_dt {
                //     let diff = target_dt - dt;
                //     sleep(Duration::from_millis((diff * 1000.0) as u64));
                // }

                value = mode.execute(value, goal, dt);

                history.push_front(value);
                history.resize(MAX_HISTORY, center);

                if is_mouse_button_down(MouseButton::Right) {
                    goal = mouse_position().1;
                }
                let end = screen_width() * 0.95;
                let spacing = end / MAX_HISTORY as f32;
                let spacing_scaled = spacing * (MAX_HISTORY as f32 * target_dt);
                // let spacing = (screen_width() - 2.0 * gap_size) / MAX_HISTORY as f32;

                draw_circle(end, goal, 12.0, MAROON);

                for i in 0..MAX_HISTORY - 1 {
                    let position_start = end - i as f32 * spacing_scaled;
                    let position_end = end - (i + 1) as f32 * spacing_scaled;

                    let value_start = history[i];
                    let value_end = history[i + 1];

                    draw_line(
                        position_start,
                        value_start,
                        position_end,
                        value_end,
                        2.0,
                        BLUE,
                    );
                    draw_circle(position_start, history[i], 6.0, BLUE);
                }
            }
            Simulation::Compare { ref settings } => {
                clear_background(BG);
                let start = screen_height();
                let goal = 0.0;

                simulate(
                    settings.simulating_time,
                    settings.first_framerate,
                    start,
                    goal,
                    BLUE,
                    &mode,
                );

                simulate(
                    settings.simulating_time,
                    settings.second_framerate,
                    start,
                    goal,
                    ORANGE,
                    &mode,
                );
            }
        }

        // draw_text("HELLO", 20.0, 20.0, 30.0, DARKGRAY);

        egui_macroquad::draw();
        next_frame().await
    }
}

fn simulate(
    target_duration: f32,
    frame_rate: f32,
    start: f32,
    goal: f32,
    color: Color,
    f: &Function,
) {
    let mut values = Vec::new();
    let time_step = 1.0 / frame_rate;
    let steps = target_duration / time_step;

    let mut current = start;
    values.push(start);

    for _ in 0..steps as u32 {
        current = f.execute(current, goal, time_step);
        values.push(current);
    }

    let offset = 300.0;
    let width = screen_width() - offset;

    let spacing = width / steps;
    for idx in 0..values.len() - 1 {
        let position_start = offset + spacing * idx as f32;
        let position_end = offset + spacing * (idx + 1) as f32;

        draw_line(
            position_start,
            values[idx],
            position_end,
            values[idx + 1],
            3.0,
            color,
        );
        draw_circle(position_end, values[idx + 1], 6.0, color);
    }
}
