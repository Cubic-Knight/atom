use winit::dpi::{PhysicalPosition, PhysicalSize};

#[allow(unused)]
pub struct Memory {
    pub window_size: PhysicalSize<i32>,
    pub window_pos: PhysicalPosition<i32>,
    pub is_cur_inside: bool,
    pub cur_pos: PhysicalPosition<f64>,
    pub is_grabbed: bool,
    pub grab_pos: PhysicalPosition<f64>,
    pub atom_center: (f64, f64),
    pub radius: f64,
    pub orbit_foci: Vec<(f64, f64)>,
    pub electron_phases: Vec<f64>,
    pub radians_per_frame: f64,
    pub frame_count: u32
}

impl Default for Memory {
    fn default() -> Self {
        Memory {
            window_size: PhysicalSize::new(320, 320),
            window_pos: PhysicalPosition::new(100, 100),
            is_cur_inside: false,
            cur_pos: PhysicalPosition::default(),
            is_grabbed: false,
            grab_pos: PhysicalPosition::default(),
            atom_center: (160.0, 160.0),
            radius: 100.0,
            orbit_foci: vec![(160.0, 250.0), (238.0, 205.0), (238.0, 115.0)],
            electron_phases: vec![0.0, 0.3, 0.6],
            radians_per_frame: 0.1,
            frame_count: 0
        }
    }
}