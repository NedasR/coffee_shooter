use imgui::{Context, FontSource};
use raylib::prelude::Vector2;
pub struct ImGuiTextBuffer {
    lines: Vec<String>,
}

impl ImGuiTextBuffer {
    pub fn new() -> Self {
        Self { lines: Vec::new() }
    }

    pub fn clear(&mut self) {
        self.lines.clear();
    }

    pub fn push_text<T: Into<String>>(&mut self, text: T) {
        self.lines.push(text.into());
    }

    pub fn push_bool(&mut self, label: &str, value: bool) {
        self.lines.push(format!("{}: {}", label, value));
    }

    pub fn push_int(&mut self, label: &str, value: i32) {
        self.lines.push(format!("{}: {}", label, value));
    }

    pub fn push_float(&mut self, label: &str, value: f32) {
        self.lines.push(format!("{}: {:.2}", label, value));
    }

    pub fn push_vector2(&mut self, label: &str, value: Vector2) {
        self.lines
            .push(format!("{}: {:.2} {:.2}", label, value.x, value.y));
    }

    pub fn draw(&self, ui: &imgui::Ui) {
        for line in &self.lines {
            ui.text(line);
        }
    }
}

// A Class that helps draw debug info at runtime in imgui for the game