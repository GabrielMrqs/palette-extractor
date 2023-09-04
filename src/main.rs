use std::env;

use anyhow::Error;
use image::{GenericImageView, Rgba};
use rand::seq::IteratorRandom;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let k = args[2].parse::<usize>()?;
    let image = image::open(path)?;
    let mut rng = rand::thread_rng();
    let points: Vec<Point> = image.pixels().map(|x| Point::new(x.2)).collect();
    let clusters: Vec<Cluster> = (0..k)
        .filter_map(|_| points.iter().choose(&mut rng))
        .map(|point| Cluster::new(*point))
        .collect();

    let clusters = fill_clusters(points, clusters);
    for mut cluster in clusters {
        let len = cluster.points.len() as u32;
        let [mut r, mut g, mut b, mut a] = [0, 0, 0, 0];
        for point in cluster.points {
            let [p_r, p_g, p_b, p_a] = point.rgba.0;
            r += p_r as u32;
            g += p_g as u32;
            b += p_b as u32;
            a += p_a as u32;
        }
        let [r, g, b, a] = [
            (r / len) as u8,
            (g / len) as u8,
            (b / len) as u8,
            (a / len) as u8,
        ];
        cluster.center = Point::new(Rgba([r, g, b, a]));
        let hex = rgb_to_hex(r, g, b);
        println!("{hex}");
    }
    Ok(())
}

fn rgb_to_hex(r: u8, g: u8, b: u8) -> String {
    format!("#{:02X}{:02X}{:02X}", r, g, b)
}

fn fill_clusters(points: Vec<Point>, mut clusters: Vec<Cluster>) -> Vec<Cluster> {
    points.iter().for_each(|point| {
        let mut min_distance = f64::MAX;
        let mut idx = 0;

        for (i, cluster) in clusters.iter().enumerate() {
            let distance = euclidean(point, &cluster.center);
            if distance < min_distance {
                min_distance = distance;
                idx = i;
            }
        }

        clusters[idx].points.push(*point);
    });

    clusters
}

fn euclidean(a: &Point, b: &Point) -> f64 {
    let [a_r, a_g, a_b, a_a] = a.rgba.0;
    let [b_r, b_g, b_b, b_a] = b.rgba.0;
    let distance = (a_r as i32 - b_r as i32).pow(2)
        + (a_g as i32 - b_g as i32).pow(2)
        + (a_b as i32 - b_b as i32).pow(2)
        + (a_a as i32 - b_a as i32).pow(2);
    (distance as f64).sqrt()
}

#[derive(Debug, Clone, Copy)]
struct Point {
    rgba: Rgba<u8>,
}

impl Point {
    fn new(rgba: Rgba<u8>) -> Self {
        Self { rgba }
    }
}

#[derive(Debug, Clone)]
struct Cluster {
    center: Point,
    points: Vec<Point>,
}

impl Cluster {
    fn new(center: Point) -> Self {
        Self {
            center,
            points: vec![center],
        }
    }
}
