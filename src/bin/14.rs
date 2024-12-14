use std::{fs, path::Path};

use image::{GrayImage, Luma};
use regex::Regex;

advent_of_code::solution!(14);

struct Bot {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}

pub fn part_one(input: &str) -> Option<u64> {
    let re = Regex::new(r"p=(\d*),(\d*) v=(-?\d*),(-?\d*)").unwrap();

    // Sample & test
    let width = 11;
    let height = 7;

    // Real
    // let width = 101;
    // let height = 103;

    let mut quad = [0u64; 4];

    for [px, py, vx, vy] in re
        .captures_iter(input)
        .map(|c| c.extract().1.map(|x| x.parse::<i32>().unwrap()))
    {
        let x = (px + 100 * (width + vx)) % width;
        let y = (py + 100 * (height + vy)) % height;

        println!("p:{px},{py}  v:{vx},{vy}  -> {x},{y}");

        match x.cmp(&(width / 2)) {
            std::cmp::Ordering::Less => match y.cmp(&(height / 2)) {
                std::cmp::Ordering::Less => quad[0] += 1,
                std::cmp::Ordering::Greater => quad[1] += 1,
                std::cmp::Ordering::Equal => {}
            },
            std::cmp::Ordering::Greater => match y.cmp(&(height / 2)) {
                std::cmp::Ordering::Less => quad[2] += 1,
                std::cmp::Ordering::Greater => quad[3] += 1,
                std::cmp::Ordering::Equal => {}
            },
            std::cmp::Ordering::Equal => {}
        }
    }
    Some(quad[0] * quad[1] * quad[2] * quad[3])
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"p=(\d*),(\d*) v=(-?\d*),(-?\d*)").unwrap();

    // Real
    const WIDTH: usize = 101;
    const HEIGHT: usize = 103;

    let mut bots = Vec::new();

    for [px, py, vx, vy] in re
        .captures_iter(input)
        .map(|c| c.extract().1.map(|x| x.parse::<i32>().unwrap()))
    {
        bots.push(Bot {
            x: px,
            y: py,
            vx,
            vy,
        });
    }

    let path = Path::new("output");

    if path.exists() {
        println!("Path exists: {}", path.display());
    } else {
        println!("Path does not exist. Creating: {}", path.display());
        fs::create_dir_all(path).expect("Failed to create output path"); // Create the directory and its parents if needed
    }

    // make iteration to 10k to output answer
    for i in 0..1 {
        let mut img = GrayImage::new(WIDTH as u32, HEIGHT as u32);

        for b in &mut bots {
            let x = (WIDTH as i32 + b.x + b.vx) % WIDTH as i32;
            let y = (HEIGHT as i32 + b.y + b.vy) % HEIGHT as i32;
            img.put_pixel(x as u32, y as u32, Luma([255]));
            b.x = x;
            b.y = y;
            //println!("{:0>3}s : bot{:0>4} p:{},{}", i, j, b.x, b.y);
        }
        img.save(format!("output/{:0>8}.png", i + 1))
            .expect("Failed to save image");
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
