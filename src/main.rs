use std::fs::File;
use std::io::prelude::*;

mod vector;

use vector::{Vec2f, Vec3f};

#[derive(Copy, Clone)]
struct Material {
    diffuse_color: Vec3f,
    albedo: Vec2f,
    specular_exponent: f32,
}

impl Material {
    fn new(albedo: Vec2f, diffuse_color: Vec3f, specular_exponent: f32) -> Self {
        Self {
            albedo,
            diffuse_color,
            specular_exponent,
        }
    }
}

struct Light {
    position: Vec3f,
    intensity: f32,
}

impl Light {
    fn new(position: Vec3f, intensity: f32) -> Self {
        Self {
            position,
            intensity,
        }
    }
}

struct Sphere {
    center: Vec3f,
    radius: f32,
    material: Material,
}

impl Sphere {
    fn new(center: Vec3f, radius: f32, material: Material) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }

    fn ray_intersect(&self, orig: &Vec3f, dir: &Vec3f) -> Option<f32> {
        let l = self.center - *orig;
        let tca = l.dot(&dir);
        let d2 = l.dot(&l) - tca * tca;
        let radius2 = self.radius * self.radius;
        if d2 > radius2 {
            return None;
        }
        let thc = (radius2 - d2).sqrt();
        let t0 = tca - thc;
        let t1 = tca + thc;
        if t0 < 0.0 && t1 < 0.0 {
            None
        } else if t0 < 0.0 {
            Some(t1)
        } else if t1 < 0.0 {
            Some(t0)
        } else {
            let distance = if t0 < t1 { t0 } else { t1 };
            Some(distance)
        }
    }
}

fn reflect(light_dir: &Vec3f, n: &Vec3f) -> Vec3f {
    *light_dir * (*n * 2.0 * (*light_dir * *n))
}

fn scene_intersect<'a>(
    orig: &Vec3f,
    dir: &Vec3f,
    spheres: &'a Vec<Sphere>,
) -> Option<(&'a Sphere, Vec3f, Vec3f)> {
    // Find the closest intersecting sphere
    let closest_intersecting = spheres
        .iter()
        // Are there other methods which can be used, so we only need to
        // iterate a single time?
        .filter_map(|sphere| match sphere.ray_intersect(&orig, &dir) {
            Some(distance) => Some((distance, sphere)),
            None => None,
        })
        .min_by_key(|(distance, _)| *distance as u32);

    match closest_intersecting {
        Some((distance, sphere)) => {
            let hit = *orig + (*dir * distance);
            let n = (hit - sphere.center).normalize();
            Some((sphere, n, hit))
        }
        None => None,
    }
}

fn cast_ray(orig: &Vec3f, dir: &Vec3f, spheres: &Vec<Sphere>, lights: &Vec<Light>) -> Vec3f {
    match scene_intersect(orig, dir, spheres) {
        Some((sphere, n, hit)) => {
            let mut diffuse_light_intensity = 0.0;
            let mut specular_light_intensity = 0.0;
            for light in lights {
                let light_dir = (light.position - hit).normalize();

                diffuse_light_intensity += light.intensity * 0.0_f32.max(light_dir.dot(&n));
                specular_light_intensity += (0.0_f32
                    .max((-reflect(&-light_dir, &n)).dot(dir))
                    .powf(sphere.material.specular_exponent))
                    * light.intensity;
            }

            // return material.diffuse_color * diffuse_light_intensity * material.albedo[0] +
            // Vec3f(1., 1., 1.)*specular_light_intensity * material.albedo[1];
            let material = sphere.material;
            let r = ((material.diffuse_color * diffuse_light_intensity) * material.albedo.0)
                + (Vec3f::new(1.0, 1.0, 1.0) * (specular_light_intensity * material.albedo.1));
            r
        }
        None => Vec3f::new(0.2, 0.7, 0.8), // Background color
    }
}

fn render(spheres: &Vec<Sphere>, lights: &Vec<Light>) -> std::io::Result<()> {
    const WIDTH: usize = 1024;
    const HEIGHT: usize = 768;
    const FOV: f32 = std::f32::consts::PI / 2.0;

    // Initialize the frame buffer with empty [r,g,b] arrays
    let mut framebuffer = vec![Vec3f::new(0.0, 0.0, 0.0); WIDTH * HEIGHT];

    for j in 0..HEIGHT {
        for i in 0..WIDTH {
            let x =
                (2.0 * (i as f32 + 0.5) / WIDTH as f32 - 1.0) * (FOV / 2.0).tan() * WIDTH as f32
                    / HEIGHT as f32;
            let y = -(2.0 * (j as f32 + 0.5) / HEIGHT as f32 - 1.0) * (FOV / 2.0).tan();
            let dir = Vec3f::new(x, y, -1.0).normalize();
            framebuffer[i + j * WIDTH] =
                cast_ray(&Vec3f::new(0.0, 0.0, 0.0), &dir, &spheres, &lights);
        }
    }

    let mut f = File::create("out.ppm")?;

    // Write the header
    write!(f, "P6\n{} {}\n255\n", &WIDTH, &HEIGHT)?;

    for frame in framebuffer.iter().take(HEIGHT * WIDTH) {
        for i in 0..3 {
            let z = match i {
                0 => frame.0,
                1 => frame.1,
                2 => frame.2,
                _ => 0.0,
            };
            let color = (255.0 * 0.0_f32.max(1.0_f32.min(z))) as u8;
            f.write_all(&[color])?;
        }
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    let ivory = Material::new(Vec2f::new(0.6, 0.3), Vec3f::new(0.4, 0.4, 0.3), 50.0);
    let red_rubber = Material::new(Vec2f::new(0.9, 0.1), Vec3f::new(0.3, 0.1, 0.1), 10.0);

    let mut spheres = vec![];
    spheres.push(Sphere::new(Vec3f::new(7., 5., -18.), 4.0, ivory));
    spheres.push(Sphere::new(Vec3f::new(-3.0, 0.0, -16.0), 2.0, ivory));
    spheres.push(Sphere::new(Vec3f::new(-1.0, -1.5, -12.), 2.0, red_rubber));
    spheres.push(Sphere::new(Vec3f::new(1.5, -0.5, -18.), 3.0, red_rubber));

    let mut lights = vec![];
    lights.push(Light::new(Vec3f::new(-20., 20., 20.), 1.5));
    lights.push(Light::new(Vec3f::new(30., 50., -25.), 1.8));
    lights.push(Light::new(Vec3f::new(30., 20., 30.), 1.7));

    render(&spheres, &lights)
}
