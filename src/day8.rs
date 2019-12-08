use itertools::Itertools;
use colored::{ColoredString, Colorize};

type Pixel = u8;
type Row = Vec<Pixel>;
type Layer = Vec<Row>;
type Image = Vec<Layer>;

fn layers(img: &Image) -> impl Iterator<Item = &Layer> {
    img.into_iter()
}

fn all_pixels(layer: &Layer) -> impl Iterator<Item = &Pixel> + '_ {
    layer.into_iter().flatten()
}

#[aoc(day8, part1)]
pub fn solve_part1(input: &str) -> isize {
    let image = parse_image(input, 25, 6);
    let layer_with_fewest_zeros = layers(&image).min_by_key(|layer| {
        all_pixels(layer).filter(|pixel| **pixel == 0).count()
    }).unwrap();
    let mut num_of_ones = 0;
    let mut num_of_twos = 0;
    for pixel in all_pixels(layer_with_fewest_zeros) {
        if *pixel == 1 {
            num_of_ones += 1
        } else if *pixel == 2 {
            num_of_twos += 1
        }
    }
    num_of_ones * num_of_twos
}

#[aoc(day8, part2)]
pub fn solve_part2(input: &str) -> String {
    let width_px = 25;
    let height_px = 6;
    let image = parse_image(input, width_px, height_px);

    // fill layer with all transparent
    let mut combined_layer = (0..height_px).map(|_| {
        (0..width_px).map(|_| 2).collect()
    }).collect::<Layer>();

    for x in 0..width_px {
        for y in 0..height_px {
            for layer in layers(&image) {
                let px = layer[y][x];
                if px == 0 {
                    // black
                    combined_layer[y][x] = 0;
                    break;
                } else if px == 1 {
                    combined_layer[y][x] = 1;
                    break;
                }
            }
        }
    }

    format!("\n{}", combined_layer.iter().map(|row| row.iter().map(|px| color(*px)).join("")).join("\n"))
}

fn color(px: u8) -> ColoredString {
    if px == 0 {
        " ".on_black()
    } else if px == 1 {
        " ".on_white()
    } else if px == 2 {
        " ".on_blue()
    } else {
        panic!("didn't get a valid px value")
    }
}

fn parse_image(input: &str, width_px: usize, height_px: usize) -> Image {
    let input = input.trim().lines().nth(0).unwrap();
    let num_pixels_per_layer = width_px * height_px;
    if input.len() % num_pixels_per_layer != 0 {
        panic!("expected multiple of {} pixels, but input length was {}", num_pixels_per_layer, input.len());
    }
    let pixels = input.chars().map(|s| s.to_digit(10).unwrap() as u8);
    let mut image = vec![];
    for all_pixels_for_a_layer in pixels.chunks(num_pixels_per_layer).into_iter() {
        let mut layer = vec![];
        for all_pixels_for_a_row in all_pixels_for_a_layer.chunks(width_px).into_iter() {
            layer.push(all_pixels_for_a_row.into_iter().collect());
        }
        image.push(layer);
    }
    image
}

#[test]
fn p1() {
    println!("{:?}", parse_image("123456789012", 3, 2))
}
