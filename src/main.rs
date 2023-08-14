use std::{
    fs::{self},
    time::Instant,
};

mod config;
use config::CONFIG;

use resvg::tiny_skia;
use resvg::usvg;
use resvg::usvg::{fontdb, TreeParsing, TreeTextToPath};

use image::{GenericImageView, EncodableLayout};
use rayon::prelude::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};

fn map_range(from_range: (f32, f32), to_range: (f32, f32), s: f32) -> f32 {
    to_range.0 + (s - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0)
}

fn main() {
    let start = Instant::now();
    let density: &str = "Ñ@#W$9876543210?!abc;:+=-,._  ";
    let density_len: f32 = (density.chars().count() - 1) as f32;
    let mut img = image::open(CONFIG.input_path).expect("CANnOT OPEN KIGGY PIC :(");

    let (w, h) = (
        (img.width() as f32 / CONFIG.resize_factor) as u32,
        (img.height() as f32 / CONFIG.resize_factor) as u32,
    );
    img = img.thumbnail(w, h);
    let (w, h) = img.dimensions();
    let grayscale = img.grayscale();
    let bytes: &[u8] = grayscale.as_luma8().unwrap().as_bytes();

    let buffer: Vec<char> = bytes.par_iter().map(|pixel| {
        let luma = *pixel as f32;
        let idx = map_range((0., 255.), (density_len, 0.), luma) as usize;
        density.chars().nth(idx).unwrap()
    }).collect();

    let time = start.elapsed();
    println!("LUMA->CHAR IN {:?}", time);

    let mut header = String::new();
    header.push_str("<?xml version=\"1.0\" ?>\n");
    header.push_str(&*format!("<svg width=\"{}\" height=\"{}\" version=\"4.0\" xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\">\n", w * CONFIG.font_size, h * CONFIG.font_size));
    header.push_str("<rect width=\"100%\" height=\"100%\" fill=\"black\"/>\n");

    let svg: String = buffer.par_iter().enumerate().map(|(i,c)| {
        let x = i as u32 % w;
        let y = i as u32 / w;
        let svg_x = (i as u32 % w) * CONFIG.font_size;
        let svg_y = (i as u32 / w) * CONFIG.font_size;
        let [r,g,b, _] = img.get_pixel(x, y).0;
        format!("<text x=\"{}\" y=\"{}\" font-size=\"{}\" fill=\"rgb({},{},{})\" font-family=\"monospace\" letter-spacing=\"4\" xml:space=\"preserve\">{}</text>\n", svg_x, svg_y, CONFIG.font_size, r,g,b, c)
    }).collect::<String>();

    header.extend(svg.chars());
    header.push_str("</svg>");

    if CONFIG.render_svg {
        render_svg(header);
    } else {
        fs::write(CONFIG.output_path, header).expect("UNABLE TO WRITE SVG");
    }

    let time = start.elapsed() - time;
    println!(
        "{} IN {:?}",
        if CONFIG.render_svg {
            "RENDERED"
        } else {
            "WRITTEN"
        },
        time
    );
}

fn render_svg(buffer: String) {
    let rtree = {
        let opts = usvg::Options::default();
        let mut fontdb = fontdb::Database::new();
        fontdb.load_system_fonts();
        let mut tree = usvg::Tree::from_data(&buffer.as_bytes(), &opts).unwrap();
        tree.convert_text(&fontdb);
        resvg::Tree::from_usvg(&tree)
    };

    let pixmap_size = rtree.size.to_int_size();
    let mut pixmap = tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height()).unwrap();
    rtree.render(tiny_skia::Transform::default(), &mut pixmap.as_mut());
    pixmap
        .save_png(CONFIG.output_path)
        .expect("CANNOT SAVE PNG");
}
