extern crate crossbeam;

use color::Color;
use scene::Scene;
use camera::Camera;
use ray::Ray;

use std::ops::Range;
use std::thread;
use std::sync::mpsc;

pub struct Renderer<'a> {
    scene: &'a Scene<'a>,
    camera: &'a Camera,
    num_threads: u32,
}

unsafe impl<'a> Sync for Renderer<'a> {}
unsafe impl<'a> Send for Renderer<'a> {}

impl<'a> Renderer<'a> {
    pub fn new(scene: &'a Scene<'a>, camera: &'a Camera, num_threads: u32) -> Renderer<'a> {
        assert!(camera.height % num_threads == 0,
                "camera.height should be devisble by num_threads");

        Renderer {
            scene: scene,
            camera: camera,
            num_threads: num_threads,
        }
    }

    pub fn render(&self, max_depth: u32) -> Vec<Color> {
        if self.num_threads == 1 {
            let range: Range<usize> = (0 as usize)..(self.camera.height as usize);
            self.render_segment(0, &range, max_depth)
        } else {
            let thread_segments = self.segments();
            let mut i = 0;

            let rxs = thread_segments.iter()
                .map(|&(start, end, offset)| {
                    let (tx, rx) = mpsc::channel();
                    let range = start..end;

                    crossbeam::scope(|scope| {
                        scope.spawn(move || {
                                        tx.send(self.render_segment(offset, &range, max_depth));
                                    });
                    });
                    rx
                })
                .collect::<Vec<_>>();


            rxs.iter().flat_map(|rx| rx.recv().unwrap()).collect()
        }
    }

    fn segments(&self) -> Vec<(usize, usize, usize)> {
        let segment_size = self.camera.height / self.num_threads;
        let mut ranges = Vec::with_capacity(self.num_threads as usize);

        for i in 0..self.num_threads {
            let start = (0 * segment_size) as usize;
            let end = ((0 + 1) * segment_size) as usize;
            ranges.push((start, end, (segment_size * i) as usize));
        }

        ranges
    }

    fn render_segment(&self,
                      segment_offset: usize,
                      segment_range: &Range<usize>,
                      max_depth: u32)
                      -> Vec<Color> {
        let width = self.camera.width as usize;
        let height = &segment_range.end - &segment_range.start;
        let mut colors = vec![Color::black(); width  * height];

        for y in segment_range.clone() {
            for x in 0..width {
                let index = y * width + x;
                let global_y = segment_offset + y;
                let ray = self.camera.create_ray(x as u32, global_y as u32);
                colors[index as usize] = self.trace(ray, max_depth);
            }
        }

        colors
    }

    fn trace(&self, ray: Ray, depth: u32) -> Color {
        if depth == 0 {
            return Color::black();
        }

        let mut result = self.scene.clear_color;
        let possible_hit = self.scene.intersect(ray);

        if let Some(hit) = possible_hit {
            result = hit.shape.material().ambient_color;
        }

        result
    }
}
