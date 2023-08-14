pub const CONFIG: Config = Config {
    input_path: "resources/city.jpg",
    output_path: "city.png",
    resize_factor: 50.,
    font_size: 50,
    render_svg: true,
};


pub struct Config<'a> {
    pub input_path: &'a str,
    pub output_path: &'a str,
    pub resize_factor: f32,
    pub font_size: u32,
    pub render_svg: bool
}