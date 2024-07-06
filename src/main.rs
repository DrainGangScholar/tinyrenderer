use image::codecs::tga::TgaEncoder;
use image::ExtendedColorType;
use std::io::{BufRead, BufReader};
use std::{fs::File, io::BufWriter};

const WIDTH: usize = 2500;
const HEIGHT: usize = 4500;
const CHANNELS: usize = 3;
const BYTES: usize = WIDTH * HEIGHT * CHANNELS;

fn save_tga(
    buffer: &[u8],
    alpha: u8,
    width: usize,
    height: usize,
    filename: &str,
) -> Result<(), std::io::Error> {
    let mut flipped_buffer = vec![alpha; BYTES];
    let width_channels = width * CHANNELS;
    for y in 0..height {
        let src = y * width_channels;
        let dest = (height - y - 1) * width_channels;
        flipped_buffer[dest..dest + width_channels]
            .copy_from_slice(&buffer[src..src + width_channels]);
    }
    let output_file = File::create(filename)?;
    let writer = BufWriter::new(output_file);
    let encoder = TgaEncoder::new(writer);
    encoder
        .encode(
            &flipped_buffer,
            width as u32,
            height as u32,
            //ExtendedColorType::Rgba8,
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
        let color = match self {
            Color::Red => Vec3 { r: 255, g: 0, b: 0 },
            Color::Green => Vec3 { r: 0, g: 255, b: 0 },
            Color::Blue => Vec3 { r: 0, g: 0, b: 255 },
            Color::Black => Vec3 { r: 0, g: 0, b: 0 },
            Color::White => Vec3 {
                r: 255,
                g: 255,
                b: 255,
            },
        };
        Some(color)
    }
}

pub struct ColorBuffer {
    buffer: Vec<u8>,
    width: usize,
    height: usize,
    channels: usize,
}

struct Point {
    x: i64,
    y: i64,
}

fn line(p0: &Point, p1: &Point, buffer: &mut Vec<u8>, alpha: u8, color: &Vec3) {
    let x0 = p0.x;
    let y0 = p0.y;
    let x1 = p1.x;
    let y1 = p1.y;

    let dx = i64::abs(x0 - x1);
    let dy = i64::abs(y0 - y1);

    let mut err = dx - dy;
    let mut err2;

    let mut x = x0;
    let mut y = y0;

    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    while x != x1 || y != y1 {
        if x >= 0 && x < WIDTH as i64 && y >= 0 && y < HEIGHT as i64 {
            let index = ((y * WIDTH as i64 + x) * CHANNELS as i64) as usize;
            buffer[index] = color.r;
            buffer[index + 1] = color.g;
            buffer[index + 2] = color.b;
            //buffer[index + 3] = alpha;
        }

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
    pub indices: Vec<usize>,
}

#[derive(Clone, Debug)]
pub struct Model {
    pub vertices: Vec<Vec3f>,
    pub faces: Vec<Face>,
    pub file_name: String,
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
        let file = File::open(&filename).unwrap();
        let reader = BufReader::new(file);
        let mut vertices: Vec<Vec3f> = vec![];
        let mut faces: Vec<Face> = vec![];
        let mut x: f64;
        let mut y: f64;
        let mut z: f64;
        for _line in reader.lines() {
            let line = _line.unwrap();
            let parts: Vec<&str> = line.split_whitespace().collect();
            if !parts.is_empty() && parts[0] == "v" {
                x = parts[1]
                    .parse()
                    .expect("Failed to parse x component of vertex");
                y = parts[2]
                    .parse()
                    .expect("Failed to parse x component of vertex");
                z = parts[2]
                    .parse()
                    .expect("Failed to parse x component of vertex");

                vertices.push(Vec3f { x, y, z });
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
        let text: Vec<&str> = filename.split("/").collect();
        let (file_name, _) = text[2].split_once(".").unwrap();
        Some(Model {
            vertices,
            faces,
            file_name: file_name.to_string(),
        })
    }

    fn normalize_vertices(self: &mut Self) {
        let (min_x, max_x) = &self
            .vertices
            .iter()
            .map(|v| v.x)
            .fold((f64::INFINITY, f64::NEG_INFINITY), |(min, max), x| {
                (min.min(x), max.max(x))
            });
        let (min_y, max_y) = &self
            .vertices
            .iter()
            .map(|v| v.y)
            .fold((f64::INFINITY, f64::NEG_INFINITY), |(min, max), y| {
                (min.min(y), max.max(y))
            });
        let (min_z, max_z) = &self
            .vertices
            .iter()
            .map(|v| v.z)
            .fold((f64::INFINITY, f64::NEG_INFINITY), |(min, max), z| {
                (min.min(z), max.max(z))
            });

        let range_x = max_x - min_x;
        let range_y = max_y - min_y;
        let range_z = max_z - min_z;

        let mut normalized_vertices = Vec::with_capacity(self.vertices.len());
        for vertex in &self.vertices {
            let normalized_x = (vertex.x - min_x) / range_x;
            let normalized_y = (vertex.y - min_y) / range_y;
            let normalized_z = (vertex.z - min_z) / range_z;
            normalized_vertices.push(Vec3f {
                x: normalized_x,
                y: normalized_y,
                z: normalized_z,
            });
        }

        self.vertices = normalized_vertices.to_vec();
    }
    pub fn draw(self: &Self, img: &mut ColorBuffer, alpha: u8, color: Vec3) {
        for face in &self.faces {
            for i in 0..3 {
                let vec0: Vec3f = self.vertices[face.indices[i]];
                let vec1: Vec3f = self.vertices[face.indices[(i + 1) % 3]];

                //let x0 = (((vec0.x +1) * (img.width as f64)) 2.0) as i64;
                let x0 = ((vec0.x) * (img.width as f64)) as i64;
                let y0 = ((vec0.y) * (img.height as f64)) as i64;
                let x1 = ((vec1.x) * (img.width as f64)) as i64;
                let y1 = ((vec1.y) * (img.height as f64)) as i64;

                let p0: Point = Point { x: x0, y: y0 };
                let p1: Point = Point { x: x1, y: y1 };

                line(&p0, &p1, &mut img.buffer, alpha, &color);
            }
        }
        let img_name = format!("./tga/{}.tga", &self.file_name);
        save_tga(&img.buffer, alpha, WIDTH, HEIGHT, &img_name).unwrap();
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut img = ColorBuffer {
        buffer: vec![0; WIDTH * HEIGHT * CHANNELS],
        width: WIDTH,
        height: HEIGHT,
        channels: CHANNELS,
    };
    let mut filename: String;
    if args.len() > 1 {
        filename = args[1].to_owned();
        filename.insert_str(0, "./obj/");
    } else {
        filename = String::from("./obj/teddy_bear.obj");
    }
    let mut model = Model::new(filename).unwrap();
    model.normalize_vertices();
    let alpha = 255;
    model.draw(&mut img, alpha, Color::Black.vec3().unwrap());
}
