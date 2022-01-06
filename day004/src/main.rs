use image::Rgb;
use noise::{NoiseFn, Perlin, Seedable};
use scarlet::{prelude::*, colormap::*};

fn main() {
    let width: u32 = 1080;
    let height: u32 = 1080;

    let mut rng = Perlin::new().set_seed(2);

    //let colors = ListedColorMap::viridis();

    // let colors = ListedColorMap::new(
    //     [
    //         [0xF1 as f64/255.0, 0xFA as f64/255.0, 0xEE as f64/255.0], 
    //         [0xA8 as f64/255.0, 0xDA as f64/255.0, 0xDC as f64/255.0],
    //         [0x45 as f64/255.0, 0x7B as f64/255.0, 0x9D as f64/255.0],
    //         [0x1D as f64/255.0, 0x35 as f64/255.0, 0x57 as f64/255.0],
    //         [0xE6 as f64/255.0, 0x39 as f64/255.0, 0x46 as f64/255.0],
    //     ].into_iter()
    // );

    let colors = ListedColorMap::new(
        [
            [0.2298057,0.298717966,0.753683153],
            [0.26623388,0.353094838,0.801466763],
            [0.30386891,0.406535296,0.84495867],
            [0.342804478,0.458757618,0.883725899],
            [0.38301334,0.50941904,0.917387822],
            [0.424369608,0.558148092,0.945619588],
            [0.46666708,0.604562568,0.968154911],
            [0.509635204,0.648280772,0.98478814],
            [0.552953156,0.688929332,0.995375608],
            [0.596262162,0.726149107,0.999836203],            
            [0.639176211,0.759599947,0.998151185],
            [0.681291281,0.788964712,0.990363227],
            [0.722193294,0.813952739,0.976574709],
            [0.761464949,0.834302879,0.956945269],
            [0.798691636,0.849786142,0.931688648],
            [0.833466556,0.860207984,0.901068838],
            [0.865395197,0.86541021,0.865395561],
            [0.897787179,0.848937047,0.820880546],
            [0.924127593,0.827384882,0.774508472],
            [0.944468518,0.800927443,0.726736146],
            [0.958852946,0.769767752,0.678007945],
            [0.96732803,0.734132809,0.628751763],
            [0.969954137,0.694266682,0.579375448],
            [0.966811177,0.650421156,0.530263762],
            [0.958003065,0.602842431,0.481775914],
            [0.943660866,0.551750968,0.434243684],
            [0.923944917,0.49730856,0.387970225],
            [0.89904617,0.439559467,0.343229596],
            [0.869186849,0.378313092,0.300267182],
            [0.834620542,0.312874446,0.259301199],
            [0.795631745,0.24128379,0.220525627],
            [0.752534934,0.157246067,0.184115123],
            [0.705673158,0.01555616,0.150232812],
        ].into_iter()
    );

    let mut image = image::RgbImage::new(width, height);

    for x in 0..width {
        for y in 0..height {
            image.put_pixel(
                x,
                y,
                f(
                    x as f64 / width as f64,
                    y as f64 / height as f64,
                    5.0,
                    &mut rng,
                    &colors,
                ),
            );
        }
    }

    image.save("images/day004a.png").unwrap();

    for x in 0..width {
        for y in 0..height {
            image.put_pixel(
                x,
                y,
                f(
                    x as f64 / width as f64,
                    y as f64 / height as f64,
                    15.0,
                    &mut rng,
                    &colors,
                ),
            );
        }
    }

    image.save("images/day004b.png").unwrap();

    for x in 0..width {
        for y in 0..height {
            image.put_pixel(
                x,
                y,
                f(
                    x as f64 / width as f64,
                    y as f64 / height as f64,
                    45.0,
                    &mut rng,
                    &colors,
                ),
            );
        }
    }

    image.save("images/day004c.png").unwrap();
}

fn f<R: NoiseFn<[f64; 2]>>(x: f64, y: f64, scale: f64, rng: &mut R, colors: &ListedColorMap) -> Rgb<u8> {
//    let index = (rng.get([scale * x, scale * y]) / 2.0 * 9.0).floor() as usize;
    let index = rng.get([scale * x, scale * y]);
    let output: RGBColor = colors.transform_single(index);
    Rgb([output.int_r(), output.int_g(), output.int_b()])
}
