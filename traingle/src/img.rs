pub struct Img(image::ImageBuffer<image::Rgb<u8>, Vec<u8>>);

impl Img {
    pub fn new(img: image::ImageBuffer<image::Rgb<u8>, Vec<u8>>) -> Img {
        Img(img)
    }
    pub fn dimensions(&self) -> (f32, f32) {
        let (x, y) = self.0.dimensions();
        (x as f32, y as f32)
    }
    pub fn get_pixel(&self, x: u32, y: u32) -> image::Rgb<u8> {
        *self.0.get_pixel(x, y)
    }
}
