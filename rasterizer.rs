use crate::three_rs::scene::LightPoint;

use super::core::Mesh;
use super::matrix::YMat4;
use super::vector::{YVec2, YVec3};

#[derive(Clone)]
pub struct Texture {
    pub width: usize,
    pub height: usize,
    pub data: Vec<YVec3>, // RGB colors
}

impl Texture {
    pub fn new(width: usize, height: usize, data: Vec<YVec3>) -> Self {
        Self {
            width,
            height,
            data,
        }
    }

    pub fn from_file(mut file: std::fs::File) -> Result<Self, Box<dyn std::error::Error>> {
        use std::io::Read;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        let img = image::load_from_memory(&buffer)?;
        let img = img.to_rgb8();
        let width = img.width() as usize;
        let height = img.height() as usize;
        let mut data = Vec::with_capacity(width * height);
        for pixel in img.pixels() {
            data.push(YVec3::new(
                pixel[0] as f32 / 255.0,
                pixel[1] as f32 / 255.0,
                pixel[2] as f32 / 255.0,
            ));
        }
        Ok(Self::new(width, height, data))
    }

    pub fn sample(&self, uv: YVec2) -> YVec3 {
        let u = uv.x.clamp(0.0, 1.0);
        let v = uv.y.clamp(0.0, 1.0);
        let x = ((u * (self.width as f32 - 1.0)).round() as usize)
            .clamp(0, self.width.saturating_sub(1));
        let y = ((v * (self.height as f32 - 1.0)).round() as usize)
            .clamp(0, self.height.saturating_sub(1));
        self.data[y * self.width + x]
    }
}

pub struct Rasterizer {
    pub width: usize,
    pub height: usize,
    pub frame_buffer: Vec<YVec3>,
    pub z_buffer: Vec<f32>,
    pub ambient_light: Option<f32>,
}

impl Rasterizer {
    pub fn new(width: usize, height: usize, ambient_light: Option<f32>) -> Self {
        Self {
            width,
            height,
            frame_buffer: vec![YVec3::new(0.0, 0.0, 0.0); width * height],
            z_buffer: vec![f32::INFINITY; width * height],
            ambient_light,
        }
    }

    pub fn is_back_culled(p: &YVec3) -> bool {
        p.x < -1.0 || p.x > 1.0 || p.y < -1.0 || p.y > 1.0 || p.z < -1.0 || p.z > 1.0
    }

    pub fn to_screen_space(&self, p: &YVec3) -> YVec3 {
        YVec3::new(
            (p.x + 1.0) * 0.5 * self.width as f32,
            (1.0 - p.y) * 0.5 * self.height as f32,
            p.z,
        )
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        if width == self.width && height == self.height {
            return;
        }
        self.width = width;
        self.height = height;
        self.frame_buffer = vec![YVec3::new(0.0, 0.0, 0.0); width * height];
        self.z_buffer = vec![f32::INFINITY; width * height];
    }

    pub fn clear(&mut self, color: YVec3) {
        self.frame_buffer.fill(color);
        self.z_buffer.fill(f32::INFINITY);
    }

    fn edge_function(a: &YVec3, b: &YVec3, c: &YVec3) -> f32 {
        (c.x - a.x) * (b.y - a.y) - (c.y - a.y) * (b.x - a.x)
    }

    pub fn draw_triangle(
        &mut self,
        v0: YVec3,
        v1: YVec3,
        v2: YVec3,
        c0: YVec3,
        c1: YVec3,
        c2: YVec3, // Colors from Gouraud Shading
        uv0: YVec2,
        uv1: YVec2,
        uv2: YVec2, // UV coordinates of the vertices
        texture: Option<&Texture>,
    ) {
        let min_x = v0.x.min(v1.x).min(v2.x).max(0.0) as usize; // Bounding box of the triangle
        let min_y = v0.y.min(v1.y).min(v2.y).max(0.0) as usize;
        let max_x = v0.x.max(v1.x).max(v2.x).min((self.width - 1) as f32) as usize;
        let max_y = v0.y.max(v1.y).max(v2.y).min((self.height - 1) as f32) as usize;

        let area = Self::edge_function(&v0, &v1, &v2);
        if area == 0.0 {
            return;
        }

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let p = YVec3::new(x as f32 + 0.5, y as f32 + 0.5, 0.0);

                let w0 = Self::edge_function(&v1, &v2, &p);
                let w1 = Self::edge_function(&v2, &v0, &p);
                let w2 = Self::edge_function(&v0, &v1, &p);

                if w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0 {
                    // Check if the point is inside the triangle
                    let w0 = w0 / area;
                    let w1 = w1 / area;
                    let w2 = w2 / area;

                    let z = v0.z * w0 + v1.z * w1 + v2.z * w2;

                    let idx = y * self.width + x;
                    if z < self.z_buffer[idx] {
                        self.z_buffer[idx] = z;

                        let color = YVec3::new(
                            // Interpolation of the colors
                            c0.x * w0 + c1.x * w1 + c2.x * w2,
                            c0.y * w0 + c1.y * w1 + c2.y * w2,
                            c0.z * w0 + c1.z * w1 + c2.z * w2,
                        );

                        let final_color = if let Some(tex) = texture {
                            // Non-perspective correct mapping for simplicity initially
                            let u = uv0.x * w0 + uv1.x * w1 + uv2.x * w2;
                            let v = uv0.y * w0 + uv1.y * w1 + uv2.y * w2;
                            let tex_color = tex.sample(YVec2::new(u, v));
                            // Multiply tint from Gouraud light by texture color
                            YVec3::new(
                                color.x * tex_color.x,
                                color.y * tex_color.y,
                                color.z * tex_color.z,
                            )
                        } else {
                            color
                        };

                        self.frame_buffer[idx] = final_color;
                    }
                }
            }
        }
    }

    pub fn draw_mesh(
        &mut self,
        mesh: &Mesh,
        mvp: &YMat4,
        world: &YMat4,
        base_color: YVec3,
        texture: Option<&Texture>,
        lights: &[LightPoint],
    ) {
        let rw = self.width;
        let rh = self.height;

        for t in &mesh.triangles {
            let v0 = mesh.vertices[t.indices[0]];
            let v1 = mesh.vertices[t.indices[1]];
            let v2 = mesh.vertices[t.indices[2]];

            let mut sv0 = mvp.transform_vec3(&v0.position);
            let mut sv1 = mvp.transform_vec3(&v1.position);
            let mut sv2 = mvp.transform_vec3(&v2.position);

            sv0.x = (sv0.x + 1.0) * 0.5 * (rw as f32 - 1.0);
            sv0.y = (1.0 - sv0.y) * 0.5 * (rh as f32 - 1.0);
            sv1.x = (sv1.x + 1.0) * 0.5 * (rw as f32 - 1.0);
            sv1.y = (1.0 - sv1.y) * 0.5 * (rh as f32 - 1.0);
            sv2.x = (sv2.x + 1.0) * 0.5 * (rw as f32 - 1.0);
            sv2.y = (1.0 - sv2.y) * 0.5 * (rh as f32 - 1.0);

            // Per-vertex lighting (Gouraud shading)
            let n0 = world.transform_vec3(&v0.normal).normalize();
            let n1 = world.transform_vec3(&v1.normal).normalize();
            let n2 = world.transform_vec3(&v2.normal).normalize();

            // World Position
            let wp0 = world.transform_vec3(&v0.position);
            let wp1 = world.transform_vec3(&v1.position);
            let wp2 = world.transform_vec3(&v2.position);

            //
            // colors
            //
            let mut final_c0 = YVec3::new(0.0, 0.0, 0.0);
            let mut final_c1 = YVec3::new(0.0, 0.0, 0.0);
            let mut final_c2 = YVec3::new(0.0, 0.0, 0.0);

            // Ambient light (base minimum)
            let ambient = self.ambient_light.unwrap_or(0.15);
            final_c0 += base_color * ambient;
            final_c1 += base_color * ambient;
            final_c2 += base_color * ambient;

            for light in lights {
                // Vertex 0 - Red
                let l_dir0 = light.transform.position - wp0;
                let d0 = l_dir0.length();
                let l_dir0_n = l_dir0.normalize();
                let atten0 = light.intensity / (d0 * d0 + 1.0);
                let diff0 = n0.dot(&l_dir0_n).max(0.0) * atten0;
                final_c0 += YVec3::new(
                    base_color.x * light.base_color.x * diff0,
                    base_color.y * light.base_color.y * diff0,
                    base_color.z * light.base_color.z * diff0,
                );

                // Vertex 1 - Green
                let l_dir1 = light.transform.position - wp1; // distance between mesh point and light
                let d1 = l_dir1.length();
                let l_dir1_n = l_dir1.normalize();
                let atten1 = light.intensity / (d1 * d1 + 1.0); // intensity at mesh point : I/(dist² + 1)
                let diff1 = n1.dot(&l_dir1_n).max(0.0) * atten1;
                final_c1 += YVec3::new(
                    base_color.x * light.base_color.x * diff1,
                    base_color.y * light.base_color.y * diff1,
                    base_color.z * light.base_color.z * diff1,
                );

                // Vertex 2 - Blue
                let l_dir2 = light.transform.position - wp2;
                let d2 = l_dir2.length();
                let l_dir2_n = l_dir2.normalize();
                let atten2 = light.intensity / (d2 * d2 + 1.0);
                let diff2 = n2.dot(&l_dir2_n).max(0.0) * atten2;
                final_c2 += YVec3::new(
                    base_color.x * light.base_color.x * diff2,
                    base_color.y * light.base_color.y * diff2,
                    base_color.z * light.base_color.z * diff2,
                );
            }

            self.draw_triangle(
                sv0, sv1, sv2, final_c0, final_c1, final_c2, v0.uv, v1.uv, v2.uv, texture,
            );
        }
    }

    pub fn draw_point(&mut self, p: YVec3, color: YVec3) {
        let x = p.x as i32;
        let y = p.y as i32;
        if x < 0 || x >= self.width as i32 || y < 0 || y >= self.height as i32 {
            return;
        }
        let idx = (y as usize) * self.width + (x as usize);
        if p.z < self.z_buffer[idx] {
            self.z_buffer[idx] = p.z;
            self.frame_buffer[idx] = color;
        }
    }

    pub fn draw_line(&mut self, v0: YVec3, v1: YVec3, c0: YVec3, c1: YVec3) {
        let dx = (v1.x - v0.x).abs();
        let dy = (v1.y - v0.y).abs();
        let steps = dx.max(dy) as i32;

        if steps == 0 {
            self.draw_point(v0, c0);
            return;
        }

        let x_inc = (v1.x - v0.x) / steps as f32;
        let y_inc = (v1.y - v0.y) / steps as f32;
        let z_inc = (v1.z - v0.z) / steps as f32;
        let c_inc = (c1 - c0) / steps as f32;

        let mut curr_x = v0.x;
        let mut curr_y = v0.y;
        let mut curr_z = v0.z;
        let mut curr_c = c0;

        for _ in 0..=steps {
            self.draw_point(YVec3::new(curr_x, curr_y, curr_z), curr_c);
            curr_x += x_inc;
            curr_y += y_inc;
            curr_z += z_inc;
            curr_c += c_inc;
        }
    }
}
