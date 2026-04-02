use crate::three_rs::{
    camera::Camera,
    core::{Mesh, Transform},
    matrix::Matrix,
    rasterizer::{Rasterizer, Texture},
    vector::YVec3,
    widget::{RenderMode, Three3DWidget},
};

pub struct Point3D {
    pub position: YVec3,
    pub color: YVec3,
}

pub struct Line3D {
    pub start: YVec3,
    pub end: YVec3,
    pub color: YVec3,
}

pub struct Model {
    pub mesh: Mesh,
    pub transform: Transform,
    pub base_color: YVec3,
    pub texture: Option<Texture>,
}

impl Model {
    pub fn new(mesh: Mesh) -> Self {
        Self {
            mesh,
            transform: Transform::default(),
            base_color: YVec3::new(1.0, 1.0, 1.0),
            texture: None,
        }
    }

    pub fn with_texture(mut self, texture: Texture) -> Self {
        self.texture = Some(texture);
        self
    }

    pub fn with_color(mut self, color: YVec3) -> Self {
        self.base_color = color;
        self
    }

    pub fn orbit(&mut self, radius: f32, angle: f32, height: f32) {
        self.transform.position.x += radius * angle.sin();
        self.transform.position.z -= radius * angle.cos();
        self.transform.position.y += height;
    }
}

pub struct LightPoint {
    pub intensity: f32,
    pub transform: Transform,
    pub base_color: YVec3,
}

impl LightPoint {
    pub fn new(intensity: f32, transform: Transform, base_color: Option<YVec3>) -> Self {
        if let Some(c) = base_color {
            Self {
                intensity,
                transform,
                base_color: c,
            }
        } else {
            Self {
                intensity,
                transform,
                base_color: YVec3::new(1.0, 1.0, 1.0),
            }
        }
    }
}

pub struct Scene {
    pub rasterizer: Rasterizer,
    pub camera: Camera,
    pub models: Vec<Model>,
    pub lights: Vec<LightPoint>,
    pub points: Vec<Point3D>,
    pub lines: Vec<Line3D>,
    pub background_color: YVec3,
}

impl Scene {
    pub fn new(width: usize, height: usize, camera: Camera, ambient_light: Option<f32>) -> Self {
        Self {
            rasterizer: Rasterizer::new(width, height, ambient_light),
            camera,
            models: Vec::new(),
            lights: Vec::new(),
            points: Vec::new(),
            lines: Vec::new(),
            background_color: YVec3::new(0.05, 0.05, 0.15),
        }
    }

    pub fn add_model(&mut self, model: Model) {
        self.models.push(model);
    }

    pub fn add_light(&mut self, light: LightPoint) {
        self.lights.push(light);
    }

    pub fn add_point(&mut self, p: Point3D) {
        self.points.push(p);
    }

    pub fn add_line(&mut self, l: Line3D) {
        self.lines.push(l);
    }

    pub fn add_circle(&mut self, center: YVec3, radius: f32, color: YVec3, segments: usize) {
        for i in 0..segments {
            let a0 = (i as f32 / segments as f32) * std::f32::consts::TAU;
            let a1 = ((i + 1) as f32 / segments as f32) * std::f32::consts::TAU;
            self.add_line(Line3D {
                start: center + YVec3::new(a0.sin() * radius, 0.0, a0.cos() * radius),
                end: center + YVec3::new(a1.sin() * radius, 0.0, a1.cos() * radius),
                color,
            });
        }
    }

    pub fn clear_primitives(&mut self) {
        self.points.clear();
        self.lines.clear();
    }

    /// Renders the scene to the internal rasterizer
    pub fn render(&mut self, area_width: usize, area_height: usize, _mode: RenderMode) {
        let (rw, rh) = (area_width, area_height);
        self.rasterizer.resize(rw, rh);
        self.rasterizer.clear(self.background_color);

        // Aspect ratio du terminal prend en compte le fait qu'un caractère est environ 2x plus haut que large
        self.camera.aspect = area_width as f32 / (area_height as f32 * 2.0);

        let view = self.camera.view_matrix();
        let proj = self.camera.projection_matrix();

        for model in &self.models {
            let world = model.transform.world_matrix();
            let wv = world.multiply(&view);
            let mvp = wv.multiply(&proj);
            self.rasterizer.draw_mesh(
                &model.mesh,
                &mvp,
                &world,
                model.base_color,
                model.texture.as_ref(),
                &self.lights,
            );
        }

        // --- POINTS ---
        for point in &self.points {
            let p_clip = proj * (view * point.position);
            if Rasterizer::is_back_culled(&p_clip) {
                continue;
            }
            let p_screen = self.rasterizer.to_screen_space(&p_clip);
            self.rasterizer.draw_point(p_screen, point.color);
        }

        // --- LINES ---
        for line in &self.lines {
            // Project start and end
            let start_v = view * line.start;
            let end_v = view * line.end;

            // Simple clipping (both must be in front)
            if start_v.z > -0.1 || end_v.z > -0.1 {
                continue;
            }

            let start_c = proj * start_v;
            let end_c = proj * end_v;

            let s_screen = self.rasterizer.to_screen_space(&start_c);
            let e_screen = self.rasterizer.to_screen_space(&end_c);
            self.rasterizer
                .draw_line(s_screen, e_screen, line.color, line.color);
        }
    }

    /// Provides a Ratatui widget to draw the scene
    pub fn to_widget(&self, mode: RenderMode) -> Three3DWidget<'_> {
        Three3DWidget::new(&self.rasterizer, mode).with_transparent(false)
    }
}
