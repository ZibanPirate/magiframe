use image::{DynamicImage, GenericImageView, Pixel};
use std::cmp::Reverse;
use std::collections::HashSet;

type Point = (u32, u32);
type Line = Vec<Point>;

pub fn extract_lines_from_image(image: &DynamicImage) -> Vec<Line> {
    let (width, height) = image.dimensions();
    let mut visited = HashSet::new();
    let mut lines = Vec::new();

    for y in 0..height {
        for x in 0..width {
            let point = (x, y);
            if visited.contains(&point) {
                continue;
            }

            let pixel = image.get_pixel(x, y);
            if pixel.to_luma()[0] == 0 {
                let mut line = Vec::new();
                dfs(point, image, &mut visited, &mut line);
                lines.push(line);
            }
        }
    }

    lines.sort_by_key(|line| Reverse(line.len()));
    lines
}

fn dfs(point: Point, image: &DynamicImage, visited: &mut HashSet<Point>, line: &mut Line) {
    let (x, y) = point;
    let (width, height) = image.dimensions();
    // let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    // 8 directions
    let directions = [
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
        (-1, 0),
        (-1, 1),
    ];

    visited.insert(point);
    line.push(point);

    for &(dx, dy) in &directions {
        let nx = x as i32 + dx;
        let ny = y as i32 + dy;

        if nx >= 0 && nx < width as i32 && ny >= 0 && ny < height as i32 {
            let npoint = (nx as u32, ny as u32);
            if !visited.contains(&npoint) && image.get_pixel(npoint.0, npoint.1).to_luma()[0] == 0 {
                dfs(npoint, image, visited, line);
            }
        }
    }
}
