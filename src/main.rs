fn main() {
    ppm();
}

fn ppm() {
    let image_width = 256;
    let image_height = 256;

    // render
    print!("P3\n{} {}\n255\n", image_width, image_height);

    for j in 0..image_height {
        eprint!("\rScanlines remaining: {} \n", image_height - j);
        for i in 0..image_width {
            let r = i as f64 / (image_width - 1) as f64;
            let g = j as f64 / (image_height - 1) as f64;
            let b = 0.0;

            let ir = (255.99 * r) as i32;
            let ig = (255.99 * g) as i32;
            let ib = (255.99 * b) as i32;

            print!("{} {} {}\n", ir, ig, ib);
        }
    }
    eprintln!("\rDone.                 ")
}
