use std::io::prelude::*;
use std::fs::File;
use std::f32;

#[derive(Copy, Clone)]
struct Color(f32, f32, f32);


impl Color {
    fn as_bytes(self) -> [u8; 3] {
        [
            (255.0 * 0.0_f32.max(1.0_f32.min(self.0))) as u8,
            (255.0 * 0.0_f32.max(1.0_f32.min(self.1))) as u8,
            (255.0 * 0.0_f32.max(1.0_f32.min(self.2))) as u8
        ]
    }
    fn mult_sca(&self, scalar: f32) -> Color {
        Color(
            self.0 * scalar,
            self.1 * scalar,
            self.2 * scalar
            )
    }
}

#[derive(Copy, Clone)]
struct Point(f32, f32, f32);

impl Point {
    fn sub(&self, subtrahend: &Point) -> Point {
        Point(
            self.0 - subtrahend.0,
            self.1 - subtrahend.1,
            self.2 - subtrahend.2
            )
    }

    fn add(&self, addend: &Point) -> Point {
        Point(
            self.0 + addend.0,
            self.1 + addend.1,
            self.2 + addend.2
            )
    }

    fn mult_sca(&self, scalar: f32) -> Point {
        Point(
            self.0 * scalar,
            self.1 * scalar,
            self.2 * scalar
            )
    }

    fn mult(&self, other: &Point) -> f32 {
        (self.0 * other.0) + (self.1 * other.1) + (self.2 * other.2)
    }

    fn norm(&self) -> f32 {
        (self.0 * self.0 + self.1 * self.1 + self.2 * self.2).sqrt()
    }

    fn normalize(&self) -> Point {
        self.mult_sca(1.0 / self.norm())
    }
}

#[derive(Copy, Clone)]
struct Material {
    diffuse_color: Color
}


struct Sphere {
    center: Point,
    radius: f32,
    material: Material,
}

impl Sphere {
    fn ray_intersect(&self, orig: &Point, dir: &Point) -> (bool, f32) {
        //Nasty vector math
        let L = self.center.sub(orig);
        let tca = L.mult(dir);
        let d2 = L.mult(&L) - tca * tca;
        if d2 > self.radius * self.radius  {
            return (false, 0.0_f32)
        }
        let thc = (self.radius * self.radius - d2).sqrt();
        let mut t0 = tca - thc;
        let t1 = tca + thc;
        if t0 < 0.0 {
            t0 = t1;
        }
        let result = if t0 < 0.0 {
            false
        }else{
            true
        };
        (result, t0)
    }
}

struct Light {
    position: Point,
    intensity: f32,
}

impl Light {
    fn new(p: Point, i: f32) -> Light {
        Light{position: p, intensity: i}
    }
}
        
fn scene_intersect(origin: &Point, dir: &Point, spheres: &Vec<Sphere>) -> (bool, Material, Point, Point) {
    let mut spheres_dist =f32::MAX;
    let mut hit = Point(0.0, 0.0, 0.0);
    let mut N = Point(0.0, 0.0, 0.0);
    let mut material = Material{diffuse_color: Color(0.0, 0.0, 0.0)};
    for sphere in spheres {
        let (intersect, dist) = sphere.ray_intersect(origin, dir);
        if intersect && dist < spheres_dist {
            spheres_dist = dist;
            hit = origin.add(&dir.mult_sca(dist));
            N = hit.sub(&sphere.center).normalize();
            material = sphere.material;
        }
    }
    (spheres_dist < 1000.0, material, hit, N)
}


fn cast_ray(origin: &Point, dir: &Point, spheres: &Vec<Sphere>, lights: &Vec<Light>) -> Color {
    let (hit, material, point, N) = scene_intersect(origin, dir, spheres);
    if !hit {
        Color(0.2, 0.7, 0.8) //Backgorund color
    }else{
        let mut diffuse_light_intensity: f32 = 0.0;
        for light in lights {
            let light_dir: Point = light.position.sub(&point).normalize();
            diffuse_light_intensity += light.intensity * f32::max(0.0, light_dir.mult(&N));
        }
        material.diffuse_color.mult_sca(diffuse_light_intensity)
    }
}


fn render(spheres: &Vec<Sphere>, lights: &Vec<Light>) {
    let width = 1024;
    let height = 768;
    let fov = f32::consts::FRAC_PI_2;

    let mut framebuffer : Vec<Color> = Vec::new();

    for j in 0..height {
        for i in 0..width {
            let x = (2.0 * (i as f32 + 0.5) / (width as f32) - 1.0) * (fov / 2.0).tan() * width as f32 / (height as f32);
            let y = -(2.0 * (j as f32 + 0.5) / (height as f32) - 1.0) * (fov / 2.0).tan();
            let dir = Point(x, y, -1.0).normalize();
            framebuffer.push(cast_ray(&Point(0.0,0.0,0.0), &dir, &spheres, lights));

        }
    }

    let mut out = File::create("out.ppm").unwrap();

    out.write_fmt(format_args!("P6\n{} {}\n255\n", width, height)).unwrap();

    for pixel in framebuffer.iter() {
        out.write(&pixel.as_bytes()).unwrap();
    }
}




fn main() {
    let ivory = Material{diffuse_color: Color(0.4, 0.4, 0.3)};
    let red_rubber = Material{diffuse_color: Color(0.3, 0.1, 0.1)};

    let spheres = vec![
        Sphere{center: Point(-3.0, 0.0, -16.0), radius: 2.0, material: ivory},
        Sphere{center: Point(-1.0, -1.5, -12.0), radius: 2.0, material: red_rubber},
        Sphere{center: Point(1.5, -0.5, -18.0), radius: 2.0, material: red_rubber},
        Sphere{center: Point(7.0, 5.0, -18.0), radius: 2.0, material: ivory},
    ];

    let lights = vec![
        Light::new(Point(-20.0, 20.0, 20.0), 1.5),
    ];

    render(&spheres, &lights);
}
