use std::fs::File;
use std::io::prelude::*;

type Vec3f = [f32; 3];

struct Sphere {
    center: Vec3f,
    radius: f32,
}

impl Sphere {
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

fn render(sphere: &Sphere) -> std::io::Result<()> {
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
            framebuffer[i + j * WIDTH] = sphere.cast_ray(&[0.0, 0.0, 0.0], &dir);
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
    let sphere = Sphere {
        center: [-3.0, 0.0, -16.0],
        radius: 2.0,
    };
    render(&sphere)
}
