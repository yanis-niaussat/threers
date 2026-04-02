use super::rasterizer::Rasterizer;
use ratatui::{buffer::Buffer, layout::Rect, style::Color, widgets::Widget};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RenderMode {
    Ascii,
    BackgroundColors,
}

pub struct Three3DWidget<'a> {
    pub rasterizer: &'a Rasterizer,
    pub mode: RenderMode,
    pub transparent: bool,
}

impl<'a> Three3DWidget<'a> {
    pub fn new(rasterizer: &'a Rasterizer, mode: RenderMode) -> Self {
        Self {
            rasterizer,
            mode,
            transparent: true,
        }
    }

    pub fn with_transparent(mut self, transparent: bool) -> Self {
        self.transparent = transparent;
        self
    }

    fn color_to_ascii(luminance: f32) -> char {
        let chars = [' ', '.', ':', '-', '=', '+', '*', '#', '%', '@'];
        let idx = (luminance * (chars.len() - 1) as f32).round() as usize;
        chars[idx.clamp(0, chars.len() - 1)]
    }

    fn color_to_ratatui_rgb(color: &super::vector::YVec3) -> Color {
        Color::Rgb(
            (color.x.clamp(0.0, 1.0) * 255.0) as u8,
            (color.y.clamp(0.0, 1.0) * 255.0) as u8,
            (color.z.clamp(0.0, 1.0) * 255.0) as u8,
        )
    }
}

impl<'a> Widget for Three3DWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        for y in 0..self.rasterizer.height.min(area.height as usize) {
            for x in 0..self.rasterizer.width.min(area.width as usize) {
                let color_vec = self.rasterizer.frame_buffer[y * self.rasterizer.width + x];
                if let Some(cell) = buf.cell_mut((area.x + x as u16, area.y + y as u16)) {
                    // Ignore background pixels si le widget est transparent
                    let z = self.rasterizer.z_buffer[y * self.rasterizer.width + x];
                    if self.transparent && z == f32::INFINITY {
                        continue; // Laisse passer le fond de l'UI
                    }

                    match self.mode {
                        RenderMode::Ascii => {
                            let luminance =
                                color_vec.x * 0.299 + color_vec.y * 0.587 + color_vec.z * 0.114;
                            let ch = Self::color_to_ascii(luminance);
                            cell.set_char(ch);
                            cell.set_fg(Color::White);
                            cell.set_bg(Color::Reset);
                        }
                        RenderMode::BackgroundColors => {
                            let tui_color = Self::color_to_ratatui_rgb(&color_vec);
                            cell.set_bg(tui_color);
                            cell.set_fg(Color::Reset);
                            cell.set_char(' ');
                        }
                    }
                }
            }
        }
    }
}
