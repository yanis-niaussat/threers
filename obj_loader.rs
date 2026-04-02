use super::core::{Mesh, Triangle, Vertex};
use super::vector::{YVec2, YVec3};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

pub struct ObjLoader;

impl ObjLoader {
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> io::Result<Mesh> {
        let file = File::open(path)?;
        Self::load(BufReader::new(file))
    }

    pub fn load<R: BufRead>(reader: R) -> io::Result<Mesh> {
        let mut positions = Vec::new();
        let mut texcoords = Vec::new();
        let mut normals = Vec::new();

        let mut out_vertices = Vec::new();
        let mut out_triangles = Vec::new();

        for line in reader.lines() {
            let line = line?;
            let tokens: Vec<&str> = line.split_whitespace().collect();
            if tokens.is_empty() {
                continue;
            }

            match tokens[0] {
                "v" => {
                    if tokens.len() >= 4 {
                        let x: f32 = tokens[1].parse().unwrap_or(0.0);
                        let y: f32 = tokens[2].parse().unwrap_or(0.0);
                        let z: f32 = tokens[3].parse().unwrap_or(0.0);
                        positions.push(YVec3::new(x, y, z));
                    }
                }
                "vt" => {
                    if tokens.len() >= 3 {
                        let u: f32 = tokens[1].parse().unwrap_or(0.0);
                        let v: f32 = tokens[2].parse().unwrap_or(0.0);
                        texcoords.push(YVec2::new(u, 1.0 - v)); // Flip V usually
                    }
                }
                "vn" => {
                    if tokens.len() >= 4 {
                        let x: f32 = tokens[1].parse().unwrap_or(0.0);
                        let y: f32 = tokens[2].parse().unwrap_or(0.0);
                        let z: f32 = tokens[3].parse().unwrap_or(0.0);
                        normals.push(YVec3::new(x, y, z));
                    }
                }
                "f" => {
                    // Face parsing. Assume triangle for simplicity (3 vertices).
                    if tokens.len() >= 4 {
                        let mut face_indices = Vec::new();
                        for i in 1..=3 {
                            let parts: Vec<&str> = tokens[i].split('/').collect();
                            let pos_idx = parts[0].parse::<usize>().unwrap_or(1).saturating_sub(1);
                            let tex_idx = if parts.len() > 1 && !parts[1].is_empty() {
                                parts[1].parse::<usize>().unwrap_or(1).saturating_sub(1)
                            } else {
                                0
                            };
                            let norm_idx = if parts.len() > 2 && !parts[2].is_empty() {
                                parts[2].parse::<usize>().unwrap_or(1).saturating_sub(1)
                            } else {
                                0
                            };

                            let pos = positions
                                .get(pos_idx)
                                .copied()
                                .unwrap_or(YVec3::new(0.0, 0.0, 0.0));
                            let uv = texcoords
                                .get(tex_idx)
                                .copied()
                                .unwrap_or(YVec2::new(0.0, 0.0));
                            let norm = normals
                                .get(norm_idx)
                                .copied()
                                .unwrap_or(YVec3::new(0.0, 0.0, 1.0));

                            let vertex = Vertex {
                                position: pos,
                                uv,
                                normal: norm,
                            };
                            out_vertices.push(vertex);
                            face_indices.push(out_vertices.len() - 1);
                        }
                        out_triangles.push(Triangle {
                            indices: [face_indices[0], face_indices[1], face_indices[2]],
                        });
                    }
                }
                _ => {}
            }
        }

        Ok(Mesh {
            vertices: out_vertices,
            triangles: out_triangles,
        })
    }
}
