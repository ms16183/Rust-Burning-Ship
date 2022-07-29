fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 複素平面上の中心
    let center = num_complex::Complex::new(-1.75 as f64, -0.02 as f64);
    // 中心からの距離
    let delta = num_complex::Complex::new(0.05 as f64, 0.05 as f64);
    // 画像サイズ
    let width = 1024;
    let height = 1024;
    // 反復回数
    let iter = 1000;
    // 発散の閾値
    let th = 10.0;

    let mut image = image::ImageBuffer::new(width, height);
    let grad = colorgrad::CustomGradient::new()
        .html_colors(&["black", "orangered", "white"])
        .domain(&[0.0, 0.1, 1.0])
        .build()?;

    for x in 0..width-1{
        for y in 0..height-1{
            let c = num_complex::Complex::new((center-delta).re + 2.0*delta.re/(width as f64)*(x as f64), (center-delta).im + 2.0*delta.im/(height as f64)*(y as f64));
            let mut z = num_complex::Complex::new(0.0 as f64, 0.0 as f64);
            let mut i = 0;
            while i < iter && z.norm() < th{
                z = num_complex::Complex::new(z.re.abs(), z.im.abs());
                z = z * z + c;
                i += 1;
            }
            let pix = image.get_pixel_mut(x, y);
            *pix = image::Rgba(grad.at(i as f64 / iter as f64).to_rgba8());
        }
    }
    image.save("./frac.png").unwrap();
    Ok(())
}
