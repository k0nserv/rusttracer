use renderer::SuperSampling;

pub struct Config {
    pub width: u32,
    pub height: u32,
    pub max_depth: u32,
    pub super_sampling: SuperSampling,
}

impl Config {
    pub fn new(width: u32, height: u32, max_depth: u32, super_sampling: SuperSampling) -> Self {
        Config {
            width: width,
            height: height,
            max_depth: max_depth,
            super_sampling: super_sampling
        }
    }
}
