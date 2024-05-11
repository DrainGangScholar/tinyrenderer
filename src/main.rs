use image::{codecs::tga::TgaEncoder, *};
use std::io::{BufRead, BufReader};
use std::usize;
use std::{fs::File, io::BufWriter};
const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;
const CHANNELS: usize = 3;
fn save_tga(
    buffer: &[u8],
    width: usize,
    height: usize,
    filename: &str,
) -> Result<(), std::io::Error> {
    let output_file = File::create(filename)?;
    let writer = BufWriter::new(output_file);
    let encoder = TgaEncoder::new(writer);
    encoder
        .encode(
            &buffer,
            width as u32,
            height as u32,
            ExtendedColorType::Rgb8,
        )
        .unwrap();
    Ok(())
}
pub struct Vec3 {
    r: u8,
    g: u8,
    b: u8,
}
pub enum Color {
    Red,
    Green,
    Blue,
    Black,
    White,
}
impl Color {
    pub fn vec3(&self) -> Option<Vec3> {
        match self {
            Color::Red => return Some(Vec3 { r: 255, g: 0, b: 0 }),
            Color::Green => return Some(Vec3 { r: 0, g: 255, b: 0 }),
            Color::Blue => return Some(Vec3 { r: 0, g: 0, b: 255 }),
            Color::Black => return Some(Vec3 { r: 0, g: 0, b: 0 }),
            Color::White => {
                return Some(Vec3 {
                    r: 255,
                    g: 255,
                    b: 255,
                })
            }
        };
        None
    }
}
pub struct ColorBuffer {
    buffer: Vec<u8>,
    width: usize,
    height: usize,
    channels: usize,
}
impl ColorBuffer {
    fn set(self: &mut Self, x: usize, y: usize, color: &Vec3) {
        let index = ((y * self.width + x) * self.channels);
        self.buffer[index] = color.r;
        self.buffer[index + 1] = color.g;
        self.buffer[index + 2] = color.b;
    }
}
struct Point {
    x: i32,
    y: i32,
}
fn line(p0: &Point, p1: &Point, buffer: &mut ColorBuffer, color: &Vec3) {
    let x0 = p0.x;
    let y0 = p0.y;
    let x1 = p1.x;
    let y1 = p1.y;

    let dx = i32::abs(x0 - x1);
    let dy = i32::abs(y0 - y1);

    let mut err = dx - dy;
    let mut err2;

    let mut x = x0;
    let mut y = y0;

    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    while x != x1 || y != y1 {
        buffer.set(x as usize, y as usize, color);
        err2 = 2 * err;
        if err2 > -dy {
            err -= dy;
            x += sx;
        }
        if err2 < dx {
            err += dx;
            y += sy;
        }
    }
}
#[derive(Clone, Copy, Debug)]
pub struct Vec3f {
    x: f64,
    y: f64,
    z: f64,
}
#[derive(Clone, Debug)]
pub struct Face {
    indices: Vec<usize>,
}
#[derive(Clone, Debug)]
pub struct Model {
    pub vertices: Vec<Vec3f>,
    pub faces: Vec<Face>,
}
impl Model {
    pub fn num_faces(self: &Self) -> usize {
        self.faces.len()
    }
    pub fn num_vertices(self: &Self) -> usize {
        self.vertices.len()
    }
}
impl IntoIterator for Model {
    type Item = Face;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.faces.into_iter()
    }
}
impl Model {
    fn new(filename: String) -> Option<Model> {
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        let mut vertices: Vec<Vec3f> = vec![];
        let mut faces: Vec<Face> = vec![];
        for _line in reader.lines() {
            let line = _line.unwrap();
            let parts: Vec<&str> = line.split_whitespace().collect();
            if !parts.is_empty() && parts[0] == "v" {
                vertices.push(Vec3f {
                    x: parts[1]
                        .parse()
                        .expect("Failed to parse x component of vertex"),
                    y: parts[2]
                        .parse()
                        .expect("Failed to parse y component of vertex"),
                    z: parts[3]
                        .parse()
                        .expect("Failed to parse z component of vertex"),
                });
            } else if !parts.is_empty() && parts[0] == "f" {
                let mut _faces: Vec<usize> = vec![];
                for part in parts.iter().skip(1) {
                    let index: usize = part
                        .split("/")
                        .next()
                        .unwrap()
                        .parse()
                        .expect("Couldn't parse face vertex index");
                    _faces.push(index - 1);
                }
                faces.push(Face { indices: _faces });
            }
        }
        Some(Model { vertices, faces })
    }
    pub fn draw(self: &Self, img: &mut ColorBuffer) {
        for face in &self.faces {
            for i in 0..3 {
                let vec0: Vec3f = self.vertices[face.indices[i]];
                let vec1: Vec3f = self.vertices[face.indices[(i + 1) % 3]];
                let p0: Point = Point {
                    x: (vec0.x as i32) * (img.width as i32) / 2,
                    y: (vec0.y as i32) * (img.height as i32) / 2,
                };
                let p1: Point = Point {
                    x: (vec1.x as i32) * (img.width as i32) / 2,
                    y: (vec1.y as i32) * (img.height as i32) / 2,
                };
                line(&p0, &p1, img, &Color::Green.vec3().unwrap());
            }
        }
        save_tga(&img.buffer, WIDTH, HEIGHT, "lol.tga").unwrap();
    }
}
fn main() {
    let mut img = ColorBuffer {
        buffer: vec![0; WIDTH * HEIGHT * CHANNELS],
        width: WIDTH,
        height: HEIGHT,
        channels: CHANNELS,
    };

    let filename = String::from("naruto.obj");
    let model = Model::new(filename).unwrap();
    model.draw(&mut img);
    save_tga(&img.buffer, WIDTH, HEIGHT, "lol.tga").unwrap();
}
