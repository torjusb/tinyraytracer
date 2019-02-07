use std::fs::File;
use std::io::prelude::*;

fn render() -> std::io::Result<()> {
    const WIDTH: usize = 1024;
    const HEIGHT: usize = 768;

    // Initialize the frame buffer with empty [r,g,b] arrays
    let mut framebuffer = vec![[0.0, 0.0, 0.0]; WIDTH * HEIGHT];

    for j in 0..HEIGHT {
        for i in 0..WIDTH {
            framebuffer[i + j * WIDTH] = [
                (j as f32) / (HEIGHT as f32),
                (i as f32) / (WIDTH as f32),
                0.0,
            ];
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
    render()
}
