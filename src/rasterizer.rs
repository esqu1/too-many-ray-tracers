use crate::color::Color;
use crate::ppm::PPM;
use crate::vector::Vec3f;
use std::collections::HashMap;

// Coordinate System
// .--------> (x)
// |
// |
// |
// |
// v (y)

pub fn interpolate(starting: f64, ending: f64, t: f64) -> f64 {
    ending * t + (1.0 - t) * starting
}

pub struct Rasterizer {
    fragments: HashMap<(usize, usize), Vec<(Color, f64)>>,
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

            let t1 = (i as f64 - top_most.y) / (bottom_pt.y - top_most.y);
            let t2 = (i as f64 - top_most.y) / (middle_pt.y - top_most.y);

            self.line(
                Vec3f::new(
                    starting_x,
                    i as f64,
                    interpolate(top_most.z, bottom_pt.z, t1),
                ),
                Vec3f::new(ending_x, i as f64, interpolate(top_most.z, middle_pt.z, t2)),
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

            let t1 = (i as f64 - top_most.y) / (bottom_pt.y - top_most.y);
            let t2 = (i as f64 - middle_pt.y) / (bottom_pt.y - middle_pt.y);

            self.line(
                Vec3f::new(
                    starting_x,
                    i as f64,
                    interpolate(top_most.z, bottom_pt.z, t1),
                ),
                Vec3f::new(
                    ending_x,
                    i as f64,
                    interpolate(middle_pt.z, bottom_pt.z, t2),
                ),
                color.clone(),
            );

            i += 1;
        }
    }

    pub fn line(&mut self, p1: Vec3f, p2: Vec3f, color: Color) {
        // Implementation of naive line drawing algorithm
        let slope = (p2.y - p1.y) as f64 / (p2.x - p1.x) as f64;
        let mut i: i32 = p1.x as i32;
        let dx: i32 = if p2.x > p1.x { 1 } else { -1 };
        while i != p2.x as i32 {
            let j = slope * (i - p1.x as i32) as f64 + p1.y as f64;
            let existing_fragments = self
                .fragments
                .entry((i as usize, j as usize))
                .or_insert(vec![]);
            existing_fragments.push((
                color.clone(),
                interpolate(p1.z, p2.z, (i as f64 - p1.x) / (p2.x - p1.x)),
            ));
            i += dx;
        }
    }

    pub fn write_to_ppm(&self, ppm: &mut PPM) {
        for fragment in self.fragments.iter() {
            let min_fragment = fragment
                .1
                .iter()
                .min_by(|f1, f2| f2.1.partial_cmp(&f1.1).unwrap());
            ppm.set_pixel(
                min_fragment.unwrap().0.clone(),
                fragment.0 .1,
                fragment.0 .0,
            );
        }
    }

    #[allow(dead_code)]
    pub fn set_camera(&self, _origin: Vec3f, _lookat: Vec3f, _up: Vec3f) {
        todo!();
    }
}
