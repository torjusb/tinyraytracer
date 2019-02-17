use std::fs::File;
use std::io::prelude::*;

type Vec3f = [f32; 3];

#[derive(Copy, Clone)]
struct Material {
    diffuse_color: Vec3f,
}

impl Material {
    fn new(diffuse_color: Vec3f) -> Self {
        Self { diffuse_color }
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
        let l = sub_vec(&self.center, &orig);
        let tca = dot_vec(&l, &dir);
        let d2 = dot_vec(&l, &l) - tca * tca;
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

    fn cast_ray(&self, orig: &Vec3f, dir: &Vec3f) -> Vec3f {
        let intersect = self.ray_intersect(&orig, &dir);
        match intersect {
            Some(_) => [0.4, 0.4, 0.3],
            None => [0.2, 0.7, 0.8],
        }
    }
}

fn scene_intersect<'a>(orig: &Vec3f, dir: &Vec3f, spheres: &'a Vec<Sphere>) -> Option<&'a Sphere> {
    // Find the closest intersecting sphere
    let closest_intersecting = spheres
        .iter()
        // Are there other methods which can be used, so we only need to
        // iterate a single time?
        .filter_map(|sphere| {
            if let Some(distance) = sphere.ray_intersect(&orig, &dir) {
                return Some((distance, sphere));
            }
            None
        })
        .min_by_key(|(distance, _)| *distance as u32);

    match closest_intersecting {
        Some((_, sphere)) => Some(sphere),
        None => None,
    }
}

fn cast_ray(orig: &Vec3f, dir: &Vec3f, spheres: &Vec<Sphere>) -> Vec3f {
    match scene_intersect(orig, dir, spheres) {
        Some(sphere) => sphere.material.diffuse_color,
        None => [0.2, 0.7, 0.8], // Background color
    }
}

fn sub_vec(a: &Vec3f, b: &Vec3f) -> Vec3f {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn mul_vec(a: &Vec3f, b: &Vec3f) -> Vec3f {
    [a[0] * b[0], a[1] * b[1], a[2] * b[2]]
}

fn dot_vec(a: &Vec3f, b: &Vec3f) -> f32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn len_vec(vec: &Vec3f) -> f32 {
    norm_vec(vec).sqrt()
}

fn norm_vec(vec: &Vec3f) -> f32 {
    (vec[0] * vec[0] + vec[1] * vec[1] + vec[2] * vec[2])
}

fn normalize_vec(vec: &Vec3f) -> Vec3f {
    let inv_len = len_vec(&vec).recip();
    [vec[0] * inv_len, vec[1] * inv_len, vec[2] * inv_len]
}

fn render(spheres: &Vec<Sphere>) -> std::io::Result<()> {
    const WIDTH: usize = 1024;
    const HEIGHT: usize = 768;
    const FOV: f32 = std::f32::consts::PI / 2.0;

    // Initialize the frame buffer with empty [r,g,b] arrays
    let mut framebuffer = vec![[0.0, 0.0, 0.0]; WIDTH * HEIGHT];

    for j in 0..HEIGHT {
        for i in 0..WIDTH {
            let x =
                (2.0 * (i as f32 + 0.5) / WIDTH as f32 - 1.0) * (FOV / 2.0).tan() * WIDTH as f32
                    / HEIGHT as f32;
            let y = -(2.0 * (j as f32 + 0.5) / HEIGHT as f32 - 1.0) * (FOV / 2.0).tan();
            let dir = normalize_vec(&[x, y, -1.0]);
            framebuffer[i + j * WIDTH] = cast_ray(&[0.0, 0.0, 0.0], &dir, &spheres);
        }
    }

    let mut f = File::create("out.ppm")?;

    // Write the header
    write!(f, "P6\n{} {}\n255\n", &WIDTH, &HEIGHT)?;

    for frame in framebuffer.iter().take(HEIGHT * WIDTH) {
        for c in frame.iter().take(3) {
            let color = (255.0 * 0.0_f32.max(1.0_f32.min(*c))) as u8;
            f.write_all(&[color])?;
        }
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    let ivory = Material::new([0.4, 0.4, 0.3]);
    let red_rubber = Material::new([0.3, 0.1, 0.1]);

    let mut spheres = vec![];
    spheres.push(Sphere::new([7., 5., -18.], 4.0, ivory));
    spheres.push(Sphere::new([-3.0, 0.0, -16.0], 2.0, ivory));
    spheres.push(Sphere::new([-1.0, -1.5, -12.], 2.0, red_rubber));
    spheres.push(Sphere::new([1.5, -0.5, -18.], 3.0, red_rubber));

    render(&spheres)
}
