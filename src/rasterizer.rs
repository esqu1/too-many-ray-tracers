use crate::color::Color;
use crate::ppm::PPM;
use crate::vector::Vec3f;
use std::collections::HashMap;

pub struct Rasterizer {
    fragments: HashMap<(usize, usize), Vec<Color>>,
}

impl Rasterizer {
    pub fn new() -> Self {
        Rasterizer {
            fragments: HashMap::new(),
        }
    }
    pub fn triangle(&mut self, p1: Vec3f, p2: Vec3f, p3: Vec3f, color: Color) {
        // icky, messy code. TODO: clean up
        let top_most: &Vec3f;
        let middle_pt: &Vec3f;
        let bottom_pt: &Vec3f;
        if p1.y < p2.y && p1.y < p3.y {
            top_most = &p1;
            if p2.y < p3.y {
                middle_pt = &p2;
                bottom_pt = &p3;
            } else {
                middle_pt = &p3;
                bottom_pt = &p2;
            }
        } else if p2.y < p1.y && p2.y < p3.y {
            top_most = &p2;
            if p1.y < p3.y {
                middle_pt = &p1;
                bottom_pt = &p3;
            } else {
                middle_pt = &p3;
                bottom_pt = &p1;
            }
        } else {
            top_most = &p3;
            if p1.y < p2.y {
                middle_pt = &p1;
                bottom_pt = &p2;
            } else {
                middle_pt = &p2;
                bottom_pt = &p1;
            }
        }

        let mut i = top_most.y as i32;
        while i < middle_pt.y as i32 {
            let starting_x = (i as f64 - bottom_pt.y) * (top_most.x - bottom_pt.x)
                / (top_most.y - bottom_pt.y)
                + bottom_pt.x;
            let ending_x = (i as f64 - middle_pt.y) * (top_most.x - middle_pt.x)
                / (top_most.y - middle_pt.y)
                + middle_pt.x;

            self.line(
                starting_x as usize,
                i as usize,
                ending_x as usize,
                i as usize,
                color.clone(),
            );

            i += 1;
        }
        while i < bottom_pt.y as i32 {
            let starting_x = (i as f64 - bottom_pt.y) * (top_most.x - bottom_pt.x)
                / (top_most.y - bottom_pt.y)
                + bottom_pt.x;
            let ending_x = (i as f64 - middle_pt.y) * (bottom_pt.x - middle_pt.x)
                / (bottom_pt.y - middle_pt.y)
                + middle_pt.x;

            self.line(
                starting_x as usize,
                i as usize,
                ending_x as usize,
                i as usize,
                color.clone(),
            );

            i += 1;
        }
    }

    pub fn line(&mut self, p1x: usize, p1y: usize, p2x: usize, p2y: usize, color: Color) {
        // Implementation of naive line drawing algorithm
        let slope = (p2y - p1y) as f64 / (p2x - p1x) as f64;
        let mut i: i32 = p1x as i32;
        let dx: i32 = if p2x > p1x { 1 } else { -1 };
        while i != p2x as i32 {
            let j = slope * (i - p1x as i32) as f64 + p1y as f64;
            let existing_fragments = self
                .fragments
                .entry((i as usize, j as usize))
                .or_insert(vec![]);
            existing_fragments.push(color.clone());
            i += dx;
        }
    }

    pub fn write_to_ppm(&self, ppm: &mut PPM) {
        for fragment in self.fragments.iter() {
            ppm.set_pixel(fragment.1[0].clone(), fragment.0 .1, fragment.0 .0);
        }
    }

    pub fn set_camera(&self, origin: Vec3f, lookat: Vec3f, up: Vec3f) {
        todo!();
    }
}

// Coordinate System
// .--------> (x)
// |
// |
// |
// |
// v (y)
