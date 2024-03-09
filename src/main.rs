fn main() {
    let width = 256;
    let height = 256;


    println!("P3\n{width} {height}\n255");

    for j in 0..height {
        for i in 0..width {
            let r: f64 = i as f64 / (width as f64 - 1.0);
            let g: f64 = j as f64 / (height as f64 - 1.0);
            let b: f64 = 0.0;

            let ir: usize = (255.999 * r) as usize;
            let ig: usize = (255.999 * g) as usize;
            let ib: usize = (255.999 * b) as usize;

            println!("{ir} {ig} {ib}");

        }
    }

}
