use crate::{Capturer, Bgr8, Memory};

fn get_screen_pixel(x: i32, y: i32, capturer: &Capturer) -> [u8; 4] {
    let screenshot = capturer.get_stored_frame().unwrap();
    let (width, height) = capturer.geometry();
    let (width, height) = (width as i32, height as i32);

    if 0 > x || x >= width || 0 > y || y >= height {
        return [0, 0, 0, 0xff];
    };
    let index = y * (width+10) + x;
    match screenshot.get(index as usize) {
        None => [0, 0, 0, 0xff],
        Some(Bgr8 { r, g, b, .. }) => [*r, *g, *b, 0xff]
    }
}

fn on_nucleus(x: f64, y: f64, memory: &Memory) -> bool {
    let (cx, cy) = memory.atom_center;
    (cx - x).hypot(cy - y) < 20.0
}

fn on_ellipse(x: f64, y: f64, memory: &Memory) -> bool {
    let (cx, cy) = memory.atom_center;
    let (rad_min, rad_max) = (memory.radius * 2.0 - 1.0, memory.radius * 2.0 + 1.0);
    for &focus1 in &memory.orbit_foci {
        let focus2 = (2.0*cx - focus1.0, 2.0*cy - focus1.1);
        let dist_to_foc1 = (focus1.0 - x).hypot(focus1.1 - y);
        let dist_to_foc2 = (focus2.0 - x).hypot(focus2.1 - y);
        let sum_dists = dist_to_foc1 + dist_to_foc2;
        if rad_min < sum_dists && sum_dists < rad_max {
            return true;
        }
    };
    false
}

fn on_electron(x: f64, y: f64, elec_pos: &Vec<(f64, f64)>) -> bool {
    for (ex, ey) in elec_pos {
        if (ex - x).hypot(ey - y) < 3.5 {
            return true;
        }
    };
    false
}

fn color_at(x: i32, y: i32, memory: &Memory, elec_pos: &Vec<(f64, f64)>) -> [u8; 4] {
    let (x, y) = (x as f64, y as f64);
    if on_nucleus(x, y, memory) {
        [255, 0, 0, 255]
    } else if on_electron(x, y, elec_pos) {
        [0, 127, 255, 255]
    } else if on_ellipse(x, y, memory) {
        [127, 127, 127, 255]
    } else {
        [0, 0, 0, 0]  // Nothing matches with this position: Transparent
    }
}

fn compute_electrons_positions(memory: &Memory) -> Vec<(f64, f64)> {
    let mut result = Vec::new();

    let (cx, cy) = memory.atom_center;
    let foci_phases: Vec<((f64, f64), f64)> = memory.orbit_foci.iter()
        .zip(memory.electron_phases.iter())
        .map(|((x, y), phase)| ((*x, *y), *phase))
        .collect();
    for (focus, phase) in foci_phases {
        // We create the vector from center to focus of the ellipse, then normalise it
        // We also make a version of that normalized vector that is rotated 90° counterclockwise
        let (dx, dy) = (focus.0 - cx, focus.1 - cy);  // This is the vector from center to focus
        let inv_center_focus_dist = 1.0 / (dx).hypot(dy);
        let normalized = (dx * inv_center_focus_dist, dy * inv_center_focus_dist);
        let rotated = (-normalized.1, normalized.0);

        // We make the two vectors that correspond to the semi-length and semi-width of the ellipse
        let length_vec = (normalized.0 * memory.radius, normalized.1 * memory.radius);
        let half_width = (memory.radius.powi(2) - dx.powi(2) - dy.powi(2)).sqrt();
        let width_vec = (rotated.0 * half_width, rotated.1 * half_width);

        // We determine the position of the electrons
        let total_radians = memory.radians_per_frame * memory.frame_count as f64 + phase;
        let rotation = (total_radians.cos(), total_radians.sin());
        let electron_displacement = (  // Vector center -> electron
            width_vec.0 * rotation.0 + length_vec.0 * rotation.1,
            width_vec.1 * rotation.0 + length_vec.1 * rotation.1
        );
        let epos1 = (cx + electron_displacement.0, cy + electron_displacement.1);
        let epos2 = (cx - electron_displacement.0, cy - electron_displacement.1);

        result.push(epos1);
        result.push(epos2);
    };
    result
}

const SQUARE_SIDE: i32 = 8;
const SLICE_LENGTH: usize = 4 * SQUARE_SIDE as usize;
pub fn draw_frame(frame: &mut [u8], capturer: &Capturer, memory: &Memory) {
    // Window size
    let (width, height): (i32, i32) = memory.window_size.into();
    let width = width - (width & 0b_111);  // Enforce a width that is a multiple of 8 (by rounding down)
    let height = height - (height & 0b_111);  // Enforce a height that is a multiple of 8 (by rounding down)

    // Window position
    let (wx, wy): (i32, i32) = memory.window_pos.into();

    let elec_pos = compute_electrons_positions(memory);

    for y1 in (0..height).step_by(SQUARE_SIDE as usize) {
        for x1 in (0..width).step_by(SQUARE_SIDE as usize) {
            let color_square: Vec<[u8; 4]> = (0..SQUARE_SIDE)
                .flat_map(|y2| std::iter::repeat(y2).zip(0..SQUARE_SIDE))
                .map(|(y2, x2)| color_at(x1+x2, y1+y2, memory, &elec_pos))
                .collect();
            
            if color_square.iter().all(|pix| pix[3] == 0) {
                let empty_slice = [0_u8; SLICE_LENGTH];
                for y2 in 0..SQUARE_SIDE {
                    let index = ((y1 + y2) * width + x1) as usize * 4;
                    frame[index..index+SLICE_LENGTH].copy_from_slice(&empty_slice);
                };
            } else {
                for y2 in 0..SQUARE_SIDE {
                    let index = (SQUARE_SIDE * y2) as usize;
                    let color_slice: [u8; SLICE_LENGTH] = color_square[index..index+SQUARE_SIDE as usize].into_iter().enumerate()
                        .map(|(x2, pixel)|
                            if pixel[3] == 0 {
                                get_screen_pixel(wx + x1+x2 as i32, wy + y1+y2, capturer)
                            } else {
                                *pixel
                            }
                        )
                        .flatten()
                        .collect::<Vec<_>>()
                        .try_into().unwrap();
                    let index = ((y1 + y2) * width + x1) as usize * 4;
                    frame[index..index+SLICE_LENGTH].copy_from_slice(&color_slice);
                };
            };
        };
    };
}
