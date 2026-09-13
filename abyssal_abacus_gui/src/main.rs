//! Retro-styled GUI front end, built on egui/eframe (pure Rust, no system
//! GTK/Qt dependency needed at build or run time - so it compiles and ships
//! the same way on Linux, macOS, and Windows).
//!
//! Uses the exact same `abyssal_abacus_core::CalcEngine` the CLI's expression parser
//! is built from, so both front ends share one source of truth for the math.

use abyssal_abacus_core::{CalcEngine, Op};
use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([300.0, 430.0])
            .with_min_inner_size([220.0, 320.0])
            .with_decorations(false) // we draw our own retro title bar below
            .with_resizable(true),
        ..Default::default()
    };

    eframe::run_native(
        "Abyssal Abacus",
        options,
        Box::new(|cc| {
            setup_retro_style(&cc.egui_ctx);
            Ok(Box::new(CalcApp::default()))
        }),
    )
}

struct CalcApp {
    engine: CalcEngine,
}

impl Default for CalcApp {
    fn default() -> Self {
        Self {
            engine: CalcEngine::new(),
        }
    }
}

// Dark "black steel + blood red" palette.
mod colors {
    use egui::Color32;
    pub const TITLE_BAR: Color32 = Color32::from_rgb(14, 12, 12);
    pub const TITLE_BAR_ACCENT: Color32 = Color32::from_rgb(138, 15, 15);
    pub const BODY_BG: Color32 = Color32::from_rgb(24, 20, 19);
    pub const SCREEN_BG: Color32 = Color32::from_rgb(10, 8, 8);
    pub const SCREEN_TEXT: Color32 = Color32::from_rgb(224, 34, 34);
    pub const BUTTON: Color32 = Color32::from_rgb(46, 40, 38);
    pub const BUTTON_HOVER: Color32 = Color32::from_rgb(64, 56, 53);
    pub const BUTTON_TEXT: Color32 = Color32::from_rgb(214, 201, 190);
    pub const OP_BUTTON: Color32 = Color32::from_rgb(107, 18, 18);
    pub const OP_BUTTON_TEXT: Color32 = Color32::from_rgb(233, 219, 210);
    pub const BORDER: Color32 = Color32::from_rgb(6, 5, 5);
    // New: bevel highlight/shadow for an embossed key look, and a power LED.
    pub const BEVEL_LIGHT: Color32 = Color32::from_rgba_premultiplied(255, 255, 255, 30);
    pub const BEVEL_DARK: Color32 = Color32::from_rgba_premultiplied(0, 0, 0, 90);
    pub const POWER_LED: Color32 = Color32::from_rgb(255, 40, 40);
}

// Base ("design") layout size, in logical points, before scaling. Everything
// is computed relative to this so the whole calculator scales and re-centers
// as one piece when the window is resized, instead of the keypad and screen
// drifting apart the way they did with the old top-left-anchored layout.
mod dims {
    pub const TITLE_H: f32 = 28.0;
    pub const PAD: f32 = 12.0;
    pub const SCREEN_W: f32 = 260.0;
    pub const SCREEN_H: f32 = 50.0;
    pub const BTN_W: f32 = 56.0;
    pub const BTN_H: f32 = 44.0;
    pub const GAP: f32 = 6.0;
    pub const KEYPAD_W: f32 = BTN_W * 4.0 + GAP * 3.0;
    pub const KEYPAD_H: f32 = BTN_H * 5.0 + GAP * 4.0;
    pub const CONTENT_W: f32 = if SCREEN_W > KEYPAD_W {
        SCREEN_W + PAD * 2.0
    } else {
        KEYPAD_W + PAD * 2.0
    };
    pub const CONTENT_H: f32 = TITLE_H + PAD + SCREEN_H + PAD + KEYPAD_H + PAD;
    pub const MIN_SCALE: f32 = 0.6;
    pub const MAX_SCALE: f32 = 3.0;
}

fn setup_retro_style(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();
    style.spacing.item_spacing = egui::vec2(6.0, 6.0);
    ctx.set_style(style);
}

impl eframe::App for CalcApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        handle_keyboard(ctx, &mut self.engine);

        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(colors::BODY_BG))
            .show(ctx, |ui| {
                let avail = ui.available_size();
                let base_origin = ui.min_rect().min;

                // Title bar is pinned flush to the top-left corner and spans
                // the FULL window width, always - no blank margin above or
                // beside it, so you can grab it anywhere along the top edge.
                let approx_scale = (avail.x / dims::CONTENT_W)
                    .min(avail.y / dims::CONTENT_H)
                    .clamp(dims::MIN_SCALE, dims::MAX_SCALE);
                let title_h = dims::TITLE_H * approx_scale;
                draw_title_bar(ui, ctx, base_origin, avail.x, approx_scale);

                // Screen + keypad are scaled and centered in whatever space
                // is left below the title bar.
                let body_avail_h = (avail.y - title_h).max(0.0);
                let body_scale = (avail.x / dims::CONTENT_W)
                    .min(body_avail_h / (dims::CONTENT_H - dims::TITLE_H))
                    .clamp(dims::MIN_SCALE, dims::MAX_SCALE);

                let content_w = dims::CONTENT_W * body_scale;
                let content_h = (dims::CONTENT_H - dims::TITLE_H - dims::PAD) * body_scale;
                let body_base = base_origin
                    + egui::vec2(
                        ((avail.x - content_w) / 2.0).max(0.0),
                        title_h + ((body_avail_h - content_h) / 2.0).max(0.0),
                    );

                let screen_rect = egui::Rect::from_min_size(
                    egui::pos2(
                        body_base.x + (content_w - dims::SCREEN_W * body_scale) / 2.0,
                        body_base.y,
                    ),
                    egui::vec2(dims::SCREEN_W * body_scale, dims::SCREEN_H * body_scale),
                );
                draw_screen(ui, &self.engine, screen_rect);

                let keypad_top = body_base.y + dims::SCREEN_H * body_scale + dims::PAD * body_scale;
                let keypad_origin = egui::pos2(
                    body_base.x + (content_w - dims::KEYPAD_W * body_scale) / 2.0,
                    keypad_top,
                );
                draw_keypad(ui, &mut self.engine, keypad_origin, body_scale);

                draw_resize_grip(ui, ctx, base_origin, avail.x, avail.y);
            });
    }
}

/// Hand-drawn title bar (decorations are off, so we own this): bar, title
/// text, a small power LED, and min/maximize/close boxes. Dragging the bar
/// moves the window; double-clicking it toggles maximize, same as a normal
/// title bar would.
fn draw_title_bar(ui: &mut egui::Ui, ctx: &egui::Context, base: egui::Pos2, w: f32, scale: f32) {
    let h = dims::TITLE_H * scale;
    let rect = egui::Rect::from_min_size(base, egui::vec2(w, h));
    let response = ui.interact(rect, ui.id().with("titlebar"), egui::Sense::click_and_drag());

    let painter = ui.painter_at(rect);
    painter.rect_filled(rect, 0.0, colors::TITLE_BAR);
    painter.rect_filled(
        egui::Rect::from_min_size(rect.left_bottom() - egui::vec2(0.0, 2.0), egui::vec2(w, 2.0)),
        0.0,
        colors::TITLE_BAR_ACCENT,
    );
    painter.text(
        rect.left_center() + egui::vec2(8.0 * scale, 0.0),
        egui::Align2::LEFT_CENTER,
        "ABYSSAL ABACUS",
        egui::FontId::monospace(14.0 * scale),
        colors::SCREEN_TEXT,
    );

    // Small glowing power LED, top-right-ish, echoing the reference numpad.
    let led_center = rect.right_center() - egui::vec2(96.0 * scale, 0.0);
    painter.circle_filled(led_center, 3.0 * scale, colors::POWER_LED.gamma_multiply(0.35));
    painter.circle_filled(led_center, 2.0 * scale, colors::POWER_LED);

    let btn_size = egui::vec2(24.0 * scale, 18.0 * scale);
    let mut x = rect.right() - 6.0 * scale - btn_size.x;
    let is_maximized = ctx.input(|i| i.viewport().maximized.unwrap_or(false));
    let buttons: [(&str, egui::ViewportCommand); 3] = [
        ("x", egui::ViewportCommand::Close),
        ("\u{25a1}", egui::ViewportCommand::Maximized(!is_maximized)),
        ("_", egui::ViewportCommand::Minimized(true)),
    ];
    for (label, cmd) in buttons {
        let btn_rect =
            egui::Rect::from_min_size(egui::pos2(x, rect.center().y - btn_size.y / 2.0), btn_size);
        let resp = ui.interact(btn_rect, ui.id().with(label), egui::Sense::click());
        painter.rect_filled(
            btn_rect,
            2.0,
            if resp.hovered() {
                colors::TITLE_BAR_ACCENT
            } else {
                colors::TITLE_BAR
            },
        );
        painter.text(
            btn_rect.center(),
            egui::Align2::CENTER_CENTER,
            label,
            egui::FontId::monospace(12.0 * scale),
            colors::BUTTON_TEXT,
        );
        if resp.clicked() {
            ctx.send_viewport_cmd(cmd);
        }
        x -= btn_size.x + 4.0 * scale;
    }

    // Most X11 window managers ignore move/resize requests on a window
    // that's still flagged "maximized," even after you've dragged it
    // smaller via the resize grip - so un-maximize first, then drag.
    if response.drag_started() {
        if is_maximized {
            ctx.send_viewport_cmd(egui::ViewportCommand::Maximized(false));
        }
        ctx.send_viewport_cmd(egui::ViewportCommand::StartDrag);
    }
    if response.double_clicked() {
        ctx.send_viewport_cmd(egui::ViewportCommand::Maximized(!is_maximized));
    }
}

fn draw_screen(ui: &mut egui::Ui, engine: &CalcEngine, rect: egui::Rect) {
    let text = engine
        .error
        .clone()
        .unwrap_or_else(|| engine.display().to_string());
    let painter = ui.painter_at(rect.expand(4.0));

    painter.rect_filled(rect.expand(2.0), 7.0, colors::SCREEN_TEXT.gamma_multiply(0.25));
    painter.rect_filled(rect, 6.0, colors::SCREEN_BG);
    painter.rect_stroke(rect, 6.0, egui::Stroke::new(2.0_f32, colors::TITLE_BAR_ACCENT));

    let font_size = rect.height() * 0.5;
    painter.text(
        rect.right_center() - egui::vec2(10.0, 0.0),
        egui::Align2::RIGHT_CENTER,
        text,
        egui::FontId::monospace(font_size),
        colors::SCREEN_TEXT,
    );

    // Faint CRT/LED scanlines across the display for a bit more retro grit.
    let line_count = 6;
    for i in 0..line_count {
        let y = rect.top() + rect.height() * (i as f32 + 0.5) / line_count as f32;
        painter.line_segment(
            [egui::pos2(rect.left() + 4.0, y), egui::pos2(rect.right() - 4.0, y)],
            egui::Stroke::new(1.0_f32, egui::Color32::from_black_alpha(40)),
        );
    }
}

/// Draws one key with an embossed/beveled edge (lighter top-left, darker
/// bottom-right) instead of a flat stroke, for a chunkier physical-button
/// feel, and reports whether it was clicked.
fn retro_button(
    ui: &mut egui::Ui,
    id: &str,
    rect: egui::Rect,
    label: &str,
    fill: egui::Color32,
    text_color: egui::Color32,
    scale: f32,
) -> bool {
    let response = ui.interact(rect, ui.id().with(id), egui::Sense::click());
    let painter = ui.painter_at(rect);
    let bg = if response.hovered() { colors::BUTTON_HOVER } else { fill };
    let rounding = 8.0 * scale;

    painter.rect_filled(rect, rounding, bg);
    painter.rect_stroke(rect, rounding, egui::Stroke::new(1.5_f32, colors::BORDER));

    // Bevel: a light stroke tracing the top+left edges, a dark stroke
    // tracing the bottom+right edges. line_segment only takes 2 points,
    // so each edge is drawn as two separate segments instead of one polyline.
    let bevel_light = egui::Stroke::new(1.5 * scale, colors::BEVEL_LIGHT);
    painter.line_segment([rect.left_bottom(), rect.left_top()], bevel_light);
    painter.line_segment([rect.left_top(), rect.right_top()], bevel_light);

    let bevel_dark = egui::Stroke::new(1.5 * scale, colors::BEVEL_DARK);
    painter.line_segment([rect.left_bottom(), rect.right_bottom()], bevel_dark);
    painter.line_segment([rect.right_bottom(), rect.right_top()], bevel_dark);

    painter.text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        label,
        egui::FontId::monospace(18.0 * scale),
        text_color,
    );
    response.clicked()
}

fn draw_keypad(ui: &mut egui::Ui, engine: &mut CalcEngine, origin: egui::Pos2, scale: f32) {
    let btn_w = dims::BTN_W * scale;
    let btn_h = dims::BTN_H * scale;
    let gap = dims::GAP * scale;

    let cell = |col: usize, row: usize| -> egui::Rect {
        let x = origin.x + col as f32 * (btn_w + gap);
        let y = origin.y + row as f32 * (btn_h + gap);
        egui::Rect::from_min_size(egui::pos2(x, y), egui::vec2(btn_w, btn_h))
    };

    let rows: [[&str; 4]; 4] = [
        ["C", "\u{00b1}", "%", "/"],
        ["7", "8", "9", "\u{00d7}"],
        ["4", "5", "6", "-"],
        ["1", "2", "3", "+"],
    ];

    for (r, row) in rows.iter().enumerate() {
        for (c, label) in row.iter().enumerate() {
            let is_op = matches!(*label, "/" | "\u{00d7}" | "-" | "+" | "%");
            let (fill, text) = if is_op {
                (colors::OP_BUTTON, colors::OP_BUTTON_TEXT)
            } else {
                (colors::BUTTON, colors::BUTTON_TEXT)
            };
            if retro_button(ui, &format!("{r}-{c}"), cell(c, r), label, fill, text, scale) {
                press(engine, label);
            }
        }
    }

    // Bottom row: 0, ., and a double-wide =
    if retro_button(ui, "0", cell(0, 4), "0", colors::BUTTON, colors::BUTTON_TEXT, scale) {
        engine.input_digit('0');
    }
    if retro_button(ui, "dot", cell(1, 4), ".", colors::BUTTON, colors::BUTTON_TEXT, scale) {
        engine.input_dot();
    }
    let equals_rect = egui::Rect::from_min_size(
        cell(2, 4).min,
        egui::vec2(btn_w * 2.0 + gap, btn_h),
    );
    if retro_button(
        ui,
        "eq",
        equals_rect,
        "=",
        colors::OP_BUTTON,
        colors::OP_BUTTON_TEXT,
        scale,
    ) {
        engine.equals();
    }
}

/// A small draggable grip in the bottom-right corner that asks the window
/// manager to start an interactive resize. This works even on X11 window
/// managers that don't give undecorated windows a resize border of their
/// own, so you're not stuck at one size if edge-dragging doesn't respond.
fn draw_resize_grip(ui: &mut egui::Ui, ctx: &egui::Context, base: egui::Pos2, w: f32, h: f32) {
    let size = 16.0;
    let rect = egui::Rect::from_min_size(
        egui::pos2(base.x + w - size, base.y + h - size),
        egui::vec2(size, size),
    );
    let response = ui.interact(rect, ui.id().with("resize_grip"), egui::Sense::drag());
    let painter = ui.painter_at(rect);
    let color = if response.hovered() {
        colors::TITLE_BAR_ACCENT
    } else {
        colors::BORDER
    };
    for i in 0..3 {
        let offset = 4.0 + i as f32 * 4.0;
        painter.line_segment(
            [
                egui::pos2(rect.right() - offset, rect.bottom()),
                egui::pos2(rect.right(), rect.bottom() - offset),
            ],
            egui::Stroke::new(1.5_f32, color),
        );
    }
    if response.drag_started() {
        let is_maximized = ctx.input(|i| i.viewport().maximized.unwrap_or(false));
        if is_maximized {
            ctx.send_viewport_cmd(egui::ViewportCommand::Maximized(false));
        }
        ctx.send_viewport_cmd(egui::ViewportCommand::BeginResize(
            egui::viewport::ResizeDirection::SouthEast,
        ));
    }
}

fn press(engine: &mut CalcEngine, label: &str) {
    match label {
        "C" => engine.clear(),
        "\u{00b1}" => engine.toggle_sign(),
        "%" => engine.percent(),
        "/" => engine.input_operator(Op::Div),
        "\u{00d7}" => engine.input_operator(Op::Mul),
        "-" => engine.input_operator(Op::Sub),
        "+" => engine.input_operator(Op::Add),
        "." => engine.input_dot(),
        d if d.len() == 1 && d.chars().next().unwrap().is_ascii_digit() => {
            engine.input_digit(d.chars().next().unwrap())
        }
        _ => {}
    }
}

fn handle_keyboard(ctx: &egui::Context, engine: &mut CalcEngine) {
    ctx.input(|i| {
        for event in &i.events {
            match event {
                egui::Event::Text(text) => {
                    for c in text.chars() {
                        match c {
                            '0'..='9' => engine.input_digit(c),
                            '.' => engine.input_dot(),
                            '+' => engine.input_operator(Op::Add),
                            '-' => engine.input_operator(Op::Sub),
                            '*' => engine.input_operator(Op::Mul),
                            '/' => engine.input_operator(Op::Div),
                            '%' => engine.input_operator(Op::Rem),
                            '=' => engine.equals(),
                            'c' | 'C' => engine.clear(),
                            _ => {}
                        }
                    }
                }
                egui::Event::Key { key, pressed: true, .. } => match key {
                    egui::Key::Enter => engine.equals(),
                    egui::Key::Escape => engine.clear(),
                    egui::Key::Backspace => engine.backspace(),
                    _ => {}
                },
                _ => {}
            }
        }
    });
}
