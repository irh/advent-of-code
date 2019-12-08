// https://adventofcode.com/2019/day/8

use std::fmt;

type Pixel = u8;
type Layer = Vec<Pixel>;

struct Image {
    layers: Vec<Layer>,
    width: usize,
    height: usize,
}

impl Image {
    fn new(input: &str, width: usize, height: usize) -> Self {
        let mut layers = vec![];
        let mut i = input.trim().chars().peekable();

        while i.peek().is_some() {
            layers.push(
                i.by_ref()
                    .take(width * height)
                    .map(|x| {
                        x.to_digit(10)
                            .expect(&format!("Unexpected image data: {}", x))
                            as u8
                    })
                    .collect(),
            );
        }

        Self {
            layers,
            width,
            height,
        }
    }

    fn corruption_check(&self) -> usize {
        let layer = self
            .layers
            .iter()
            .min_by_key(|layer| layer.iter().filter(|x| **x == 0).count())
            .unwrap();
        layer.iter().filter(|x| **x == 1).count() * layer.iter().filter(|x| **x == 2).count()
    }

    fn iter(&self) -> ImageIter {
        ImageIter {
            image: self,
            position: 0,
        }
    }
}

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, pixel) in self.iter().enumerate() {
            if i > 0 && i % self.width == 0 {
                write!(f, "\n")?;
            }
            match pixel {
                0 => write!(f, "░")?,
                1 => write!(f, "█")?,
                _ => write!(f, " ")?,
            };
        }
        write!(f, "\n")
    }
}

struct ImageIter<'a> {
    image: &'a Image,
    position: usize,
}

impl<'a> Iterator for ImageIter<'a> {
    type Item = Pixel;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position == self.image.width * self.image.height {
            return None;
        }

        let mut result = Some(2);

        for layer in self.image.layers.iter() {
            let pixel = layer[self.position];
            if pixel < 2 {
                result = Some(pixel);
                break;
            }
        }

        self.position += 1;
        result
    }
}

fn main() {
    let image = Image::new(include_str!("input/8"), 25, 6);
    println!("Corruption check: {}", image.corruption_check());
    println!("{}", image);
}
