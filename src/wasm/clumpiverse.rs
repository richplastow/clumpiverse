use wasm_bindgen::prelude::*;
// use web_sys::console;

use crate::mass::Mass;
use crate::renderer::Renderer;

#[wasm_bindgen]
pub struct Clumpiverse {
    canvas_h: f64,
    canvas_w: f64,
    focused_mass: usize,
    grabbing: bool,
    grab_ids: Vec<usize>,
    guide: u8,
    hover_x: f64,
    hover_y: f64,
    masses: Vec<Mass>,
    masses_antigrav: Vec<usize>,
    masses_elastic: Vec<usize>,
    masses_gas: Vec<usize>,
    matter_antigrav_size: u8,
    matter_elastic_size: u8,
    matter_gas_size: u8,
    next_id: usize,
    now: f64,
    renderer: Renderer,
    theme: u8,
    tpgy: u8,
    trail: u8,
}

#[wasm_bindgen]
impl Clumpiverse {




    // INITIALISE

    pub fn new(
        canvas_w: f64,
        canvas_h: f64,
        canvas_id: String,
        guide_f64: f64,
        matter_antigrav_size_f64: f64,
        matter_elastic_size_f64: f64,
        matter_gas_size_f64: f64,
        theme_f64: f64,
        tpgy_f64: f64,
        trail_f64: f64,
    ) -> Clumpiverse {
        Clumpiverse {
            canvas_h,
            canvas_w,
            focused_mass: usize::MAX,
            grabbing: false,
            grab_ids: vec![],
            guide: guide_f64 as u8,
            hover_x: -1.0, // mouse outside the canvas
            hover_y: -1.0, // mouse outside the canvas
            masses: vec![],
            masses_antigrav: vec![],
            masses_elastic: vec![],
            masses_gas: vec![],
            matter_antigrav_size: matter_antigrav_size_f64 as u8,
            matter_elastic_size: matter_elastic_size_f64 as u8,
            matter_gas_size: matter_gas_size_f64 as u8,
            next_id: 0,
            now: 0.0,
            renderer: Renderer::new(canvas_id),
            theme: theme_f64 as u8,
            tpgy: tpgy_f64 as u8,
            trail: trail_f64 as u8,
        }
    }




    // CANVAS INPUT

    pub fn grab_masses(
        &mut self,
        grab_x: f64,
        grab_y: f64,
    ) -> String {
        for mass in &mut self.masses {
            // Get the distance between hover point and Mass center, in px.
            let dx_pxl = mass.x - grab_x; // +ve or -ve
            let dy_pxl = mass.y - grab_y; // +ve or -ve
            let d_pxl = (dx_pxl * dx_pxl + dy_pxl * dy_pxl).sqrt(); // only +ve
            if d_pxl <= mass.radius {
                self.grabbing = true;
                self.grab_ids.push(mass.id);
                mass.grab_x = dx_pxl;
                mass.grab_y = dy_pxl;
                continue;
            }

            // For masses near the left edge, a lesser-part of their circle
            // could be hovered on the right-edge.
            if mass.x < mass.radius {
                // Get the distance between hover point and Mass center, if the
                // Mass was shifted rightwards by a canvas-width.
                let dx_pxl = mass.x + self.canvas_w - grab_x;
                let d_pxl = (dx_pxl * dx_pxl + dy_pxl * dy_pxl).sqrt();
                if d_pxl <= mass.radius {
                    self.grabbing = true;
                    self.grab_ids.push(mass.id);
                    mass.grab_x = dx_pxl;
                    mass.grab_y = dy_pxl;
                    continue;
                }
            }

            // For masses near the right edge, a lesser-part of their circle
            // could be hovered on the left-edge.
            if mass.x > self.canvas_w - mass.radius {
                // Get the distance between hover point and Mass center, if the
                // Mass was shifted leftwards by a canvas-width.
                let dx_pxl = mass.x - self.canvas_w - grab_x;
                let d_pxl = (dx_pxl * dx_pxl + dy_pxl * dy_pxl).sqrt();
                if d_pxl <= mass.radius {
                    self.grabbing = true;
                    self.grab_ids.push(mass.id);
                    mass.grab_x = dx_pxl;
                    mass.grab_y = dy_pxl;
                    continue;
                }
            }
        }

        // If any Masses ware grabbed, focus on the most recently created.
        let len = self.grab_ids.len();
        if len != 0 { self.focused_mass = self.grab_ids[len - 1] }

        // 
        format!("{:?}", self.grabbing)
    }

    pub fn ungrab_masses(
        &mut self,
    ) {
        // Reset all grabbed Masses’ `grab_x` and `grab_y` to `0.0`.
        for id in &self.grab_ids {
            let mass = &mut self.masses[*id];
            mass.grab_x = 0.0;
            mass.grab_y = 0.0;
        }
        self.grabbing = false;
        self.grab_ids = vec![];
    }

    pub fn add_mass(
        &mut self,
        mass_x: f64,
        mass_y: f64,
        matter_f64: f64,
        radius: f64,
    ) -> usize {
        // Deal with .tpgy-none right away.
        if self.tpgy == 0 {
            self.focused_mass = usize::MAX; // focus on nothing
            return usize::MAX // signifies nothing was added
        }

        let matter = matter_f64 as u8;
        let id = self.next_id;
        let mut connections: Vec<usize> = vec![];

        match self.tpgy {

            // .tpgy-none
            0 => (), // dealt with above

            // .tpgy-mono
            1 => {
                self.focused_mass = usize::MAX; // focus on nothing
            },

            // .tpgy-pair
            2 => {
                // If nothing is currently focused:
                if self.focused_mass == usize::MAX {
                    // Focus on this Mass, the first of a pair. It should not be
                    // connected to anything yet.
                    self.focused_mass = id;
                // Otherwise, there is a Mass with focus:
                } else {
                    // Connect to the focused Mass, and then focus on nothing.
                    connections.push(self.focused_mass);
                    self.masses[self.focused_mass].connections.push(id);
                    self.focused_mass = usize::MAX;
                }
            },

            // .tpgy-hair
            3 => {
                // If there is a Mass with focus:
                if self.focused_mass != usize::MAX {
                    // Connect to the focused Mass.
                    connections.push(self.focused_mass);
                    self.masses[self.focused_mass].connections.push(id);
                }
                // Focus on this Mass, whether it’s the first of a new hair, or
                // just the latest in a long strand.
                self.focused_mass = id;
            },

            // .tpgy-strut
            4 => {
                // If there is a Mass with focus:
                if self.focused_mass != usize::MAX {
                    let m0_id = self.focused_mass;
                    let m0 = &self.masses[m0_id];
                    let len = m0.connections.len();
                    // If that focused Mass has any connections:
                    if len != 0 {
                        // Get the id of its most recently-connected Mass.
                        let m1_id = m0.connections[len - 1];
                        // Connect to its most recently-connected Mass.
                        connections.push(m1_id);
                        self.masses[m1_id].connections.push(id);
                    }
                    // Connect to the focused Mass.
                    connections.push(m0_id);
                    self.masses[m0_id].connections.push(id);
                }
                // Focus on this Mass, whether it’s the first of a new strut, or
                // just the latest in a long existing strut.
                self.focused_mass = id;
            },

            // .tpgy-girder
            5 => {
                // If there is a Mass with focus:
                if self.focused_mass != usize::MAX {
                    let m0_id = self.focused_mass;
                    let m0 = &self.masses[m0_id];
                    let len0 = m0.connections.len();
                    // If that focused Mass has any connections:
                    if len0 != 0 {
                        // Get the id of its most recently-connected Mass.
                        let m1_id = m0.connections[len0 - 1];

                        // Odd ids also join to an older Mass.
                        if id % 2 == 1 {
                            let m1 = &self.masses[m1_id];
                            let len1 = m1.connections.len();
                            // If that Mass has more than one connection:
                            if len1 > 1 {
                                // Get the id of its second most recently connected Mass.
                                let m2_id = m1.connections[len1 - 2];
                                // Connect to its secondmost recently connected Mass.
                                connections.push(m2_id);
                                self.masses[m2_id].connections.push(id);
                            }
                        }

                        // Connect to its most recently-connected Mass.
                        connections.push(m1_id);
                        self.masses[m1_id].connections.push(id);
                    }
                    // Connect to the focused Mass.
                    connections.push(m0_id);
                    self.masses[m0_id].connections.push(id);
                }
                // Focus on this Mass, whether it’s the first of a new girder,
                // or just the latest in a long existing girder.
                self.focused_mass = id;
            },

            // .tpgy-pyramid
            6 => {
                // If nothing is currently focused:
                if self.focused_mass == usize::MAX {
                    // Focus on this Mass, the apex of the pyramid. It will
                    // remain focused as the pyramid is constructed. It should
                    // not be connected to anything yet.
                    self.focused_mass = id;
                // Otherwise, there is a Mass with focus:
                } else {
                    // All ids join to the most recently created Mass.
                    connections.push(id - 1);
                    self.masses[id - 1].connections.push(id);
                    // Get the id-delta between the focused mass and new Mass.
                    let d_id = id - self.focused_mass; // eg 8
                    // If this is the first Mass, or the Mass will be the first
                    // in a new row, after a corner:
                    if d_id == 0 || is_tricorn(d_id - 1) {
                        // Do nothing — the connection to `id - 1` is enough.
                    // Otherwise, if the id is for a Mass on a corner:
                    } else if is_tricorn(d_id) { // 8 is not a triangle corner
                        let tri_test_1 = (d_id + 1) * 8 + 1;
                        let sqrt_1 = (tri_test_1 as f64).sqrt();
                        let rel_id = sqrt_1 as usize - 3;
                        // The id is for the last Mass in a row, and needs
                        // one extra connection.
                        connections.push(id - rel_id);
                        self.masses[id - rel_id].connections.push(id);
                    // Otherwise, the Mass is part way along a row, and
                    // needs two extra connections.
                    } else {
                        // Get the id (relative to focused) of the most
                        // recent triangle corner.
                        let prev_jut = prev_tricorn(d_id) + 1; // 8 -> 6
                        // Use part of the ‘triangle number test’ equation.
                        let curr_tri_test = d_id * 8 + 1; // 8 -> 65
                        let prev_jut_tri_test = prev_jut * 8 + 1; // 6 -> 49
                        let rel_id = (curr_tri_test - prev_jut_tri_test) / 4; // (65-49)/4 = 4
                        // console::log_1(&format!("id {} prev_jut {} rel_id {} id - rel_id {}", id, prev_jut, rel_id, id - rel_id).into());
                        connections.push(id - rel_id); // connect to id - 4
                        self.masses[id - rel_id].connections.push(id);
                        connections.push(id - rel_id - 1); // connect to id - 5
                        self.masses[id - rel_id - 1].connections.push(id);
                    }
                }
            },

            // .tpgy-plank
            7 => {
                // If nothing is currently focused:
                if self.focused_mass == usize::MAX {
                    // Focus on this Mass, the foundation of the plank. It will
                    // remain focused as the plank is constructed. It should not
                    // be connected to anything yet.
                    self.focused_mass = id;
                // Otherwise, there is a Mass with focus:
                } else {
                    // All ids join to the most recently created Mass.
                    connections.push(id - 1);
                    self.masses[id - 1].connections.push(id);
                    // Get the id-delta between the focused mass and new Mass.
                    let d_id = id - self.focused_mass; // eg 8
                    // Get the square root of the id-delta, and round it down.
                    // Then, get the square of that rounded-down number.
                    let sqrt = (d_id as f64).sqrt(); // 8 -> 8.0 -> 2.828
                    let floor_sqrt = sqrt.floor() as usize; // 2.828 -> 2
                    let square_jutter = floor_sqrt.pow(2); // 2 -> 4
                    let rectangle_jutter = square_jutter + floor_sqrt; // 4 + 2 = 6
                    // If the id is not for a Mass which will ‘jut out’:
                    if d_id != square_jutter      // 8 != 4 is true
                    && d_id != rectangle_jutter { // 8 != 6 is true
                        let rel_id = if d_id < rectangle_jutter { // 8 < 6 is false
                            floor_sqrt * 4 - 3
                        } else {
                            floor_sqrt * 4 - 1 // 2 * 4 - 1 = 7
                        };
                        if rel_id != 0 {
                            connections.push(id - rel_id); // 8 - 7 = 1
                            self.masses[id - rel_id].connections.push(id);
                        }
                    }
                }
            },

            // .tpgy-brick
            8 => {
                // If nothing is currently focused:
                if self.focused_mass == usize::MAX {
                    // Focus on this Mass, the foundation of the brick. It will
                    // remain focused as the brick is constructed. It should not
                    // be connected to anything yet.
                    self.focused_mass = id;
                // Otherwise, there is a Mass with focus:
                } else {
                    // All ids join to the most recently created Mass.
                    connections.push(id - 1);
                    self.masses[id - 1].connections.push(id);
                    // Get the id-delta between the focused mass and new Mass.
                    let d_id = id - self.focused_mass; // eg 8
                    // id-delta `2` is a special case.
                    if d_id == 2 {
                        connections.push(self.focused_mass);
                        self.masses[self.focused_mass].connections.push(id);
                    } else {
                        // Get the square root of the id-delta, and round it down.
                        // Then, get the square of that rounded-down number.
                        let sqrt = (d_id as f64).sqrt(); // 8 -> 8.0 -> 2.828
                        let floor_sqrt = sqrt.floor() as usize; // 2.828 -> 2
                        let square_jutter = floor_sqrt.pow(2); // 2 -> 4
                        let rectangle_jutter = square_jutter + floor_sqrt; // 4 + 2 = 6
                        // If the id is for a Mass which will ‘jut out’:
                        let is_jutter = d_id == square_jutter || d_id == rectangle_jutter;
                        if is_jutter {
                            // We just need one extra connection — a diagonal.
                            // Jutter 6 and below connect back to the focused Mass.
                            if d_id < 7 {
                                connections.push(0);
                                self.masses[0].connections.push(id);
                            // Jutter 9 and above use a formula for their extra
                            // diagonal connection.
                            } else {
                                let rel_id = if d_id == square_jutter {
                                    floor_sqrt * 4 - 4
                                } else {
                                    floor_sqrt * 4 - 2
                                };
                                connections.push(id - rel_id);
                                self.masses[id - rel_id].connections.push(id);
                            }

                        // Otherwise, the id is for a Mass which will not jut out:
                        } else {
                            // We need one or more extra connections. First, add the
                            // extra horizontal or vertical (same as tpgy-plank).
                            let rel_id = if d_id < rectangle_jutter { // 8 < 6 is false
                                floor_sqrt * 4 - 3
                            } else {
                                floor_sqrt * 4 - 1 // 2 * 4 - 1 = 7
                            };
                            if rel_id != 0 {
                                connections.push(id - rel_id); // 8 - 7 = 1
                                self.masses[id - rel_id].connections.push(id);
                            }
                            // Get the id-delta between the focused mass and the
                            // most recently created Mass.
                            let prev_d_id = id - 1 - self.focused_mass; // eg 7
                            // Get the square root of the prev-id-delta. Round down.
                            // Then, get the square of that rounded-down number.
                            let prev_sqrt = (prev_d_id as f64).sqrt();
                            let prev_floor_sqrt = prev_sqrt.floor() as usize;
                            let prev_square_jutter = prev_floor_sqrt.pow(2);
                            let prev_rectangle_jutter = prev_square_jutter + prev_floor_sqrt;
                            // If the most recently created Mass jutted out:
                            let prev_is_jutter = prev_d_id == prev_square_jutter || prev_d_id == prev_rectangle_jutter;
                            if prev_is_jutter {
                                // We need one or two extra connections. We always
                                // need a diagonal connecting back two Masses.
                                connections.push(id - 2);
                                self.masses[id - 2].connections.push(id);
                                // For id-deltas past 6, we also need a connection
                                // back to the Mass one-step newer than the extra
                                // horizontal or vertical connection made above.
                                if d_id > 6 {
                                    connections.push(id - rel_id + 1);
                                    self.masses[id - rel_id + 1].connections.push(id);
                                }
                            // Otherwise the most recently created Mass did not jut out:
                            } else {
                                // Get the id-delta between the focused mass and the
                                // Mass which will be created next.
                                let next_d_id = id + 1 - self.focused_mass; // eg 9
                                let next_sqrt = (next_d_id as f64).sqrt();
                                let next_floor_sqrt = next_sqrt.floor() as usize;
                                let next_square_jutter = next_floor_sqrt.pow(2);
                                let next_rectangle_jutter = next_square_jutter + next_floor_sqrt;
                                // If the Mass to be created next will jut out:
                                let next_is_jutter = next_d_id == next_square_jutter || next_d_id == next_rectangle_jutter;
                                if next_is_jutter {
                                    // For id-deltas past 6, we need one extra diagonal
                                    // connection. To the Mass one-step older than 
                                    // the extra horizontal or vertical connection made 
                                    // above.
                                    if d_id > 6 {
                                        connections.push(id - rel_id - 1);
                                        self.masses[id - rel_id - 1].connections.push(id);
                                    }
                                    // Otherwise the most recently created Mass did not jut out:
                                } else {
                                    // For id-deltas past 6, we need two extra diagonal
                                    // connections. One to the Mass one-step older than 
                                    // the extra horizontal or vertical connection made 
                                    // above. And another to the Mass one-step newer
                                    // than it.
                                    if d_id > 6 {
                                        connections.push(id - rel_id - 1);
                                        self.masses[id - rel_id - 1].connections.push(id);
                                        connections.push(id - rel_id + 1);
                                        self.masses[id - rel_id + 1].connections.push(id);
                                    }
                                }
                            }
                        }
                    }
                }
            },

            // not a recognised tpgy code
            _ => (),
        }

        self.masses.push(
            Mass {
                id,
                matter,
                grab_x: 0.0,
                grab_y: 0.0,
                radius,
                connections,
                start: self.now,
                vx: 0.0,
                vy: 0.0,
                x: mass_x,
                y: mass_y,
            }
        );
        match matter {
            0 => self.masses_antigrav.push(id),
            1 => self.masses_elastic.push(id),
            2 => self.masses_gas.push(id),
            _ => (),
        }
        self.next_id = id + 1;
        id
    }

    pub fn update_hover(
        &mut self,
        hover_x: f64,
        hover_y: f64,
    ) {
        self.hover_x = hover_x;
        self.hover_y = hover_y;
    }




    // PANEL INPUT

    pub fn set_tpgy(&mut self, tpgy_f64: f64) {
        self.tpgy = tpgy_f64 as u8;
    }
    pub fn set_guide(&mut self, guide_f64: f64) {
        self.guide = guide_f64 as u8;
    }
    pub fn set_size(&mut self, matter_f64: f64, size_f64: f64) {
        let matter = matter_f64 as u8;
        let size = size_f64 as u8;
        match matter {
            0 => self.matter_antigrav_size = size,
            1 => self.matter_elastic_size = size,
            2 => self.matter_gas_size = size,
            _ => (),
        }
    }
    pub fn set_theme(&mut self, theme_f64: f64) {
        self.theme = theme_f64 as u8;
    }
    pub fn set_trail(&mut self, trail_f64: f64) {
        self.trail = trail_f64 as u8;
    }




    // RENDER

    pub fn set_now(&mut self, now: f64) {
        let dnow = now - self.now;
        let matter_antigrav_radius = size_to_radius(self.matter_antigrav_size);
        let matter_elastic_radius  = size_to_radius(self.matter_elastic_size);
        let matter_gas_radius      = size_to_radius(self.matter_gas_size);
        let mut masses_clone: Vec<Mass> = vec![];
        for i in 0..self.masses.len() {
            masses_clone.push(self.masses[i].clone());
        }
        for i in 0..self.masses.len() {
            if self.grabbing && self.grab_ids.contains(&i) {
                self.masses[i].x = self.hover_x + &self.masses[i].grab_x;
                self.masses[i].y = self.hover_y + &self.masses[i].grab_y;
            } else {
                self.masses[i].step_sim(
                    dnow,
                    self.canvas_w,
                    &masses_clone,
                    matter_antigrav_radius,
                    matter_elastic_radius,
                    matter_gas_radius,
                );
            }
        }
        self.now = now;
    }

    pub fn render(&mut self) {
        let matter_antigrav_radius = size_to_radius(self.matter_antigrav_size);
        let matter_elastic_radius  = size_to_radius(self.matter_elastic_size);
        let matter_gas_radius      = size_to_radius(self.matter_gas_size);

        self.renderer.clear_canvas(
            0.0,
            0.0,
            self.canvas_w,
            self.canvas_h,
            self.theme,
            self.trail,
        );
        self.renderer.render_masses(
            &self.masses,
            &self.masses_antigrav,
            &self.masses_elastic,
            &self.masses_gas,
            matter_antigrav_radius,
            matter_elastic_radius,
            matter_gas_radius,
            self.canvas_w,
            self.guide,
            self.theme,
        );
        if self.grabbing {
            self.renderer.render_grabbed_masses(
                &self.masses,
                &self.grab_ids,
                matter_antigrav_radius,
                matter_elastic_radius,
                matter_gas_radius,
                self.canvas_w,
                self.theme,
            )
        } else {
            self.renderer.render_hover(
                &self.masses,
                matter_antigrav_radius,
                matter_elastic_radius,
                matter_gas_radius,
                self.canvas_w,
                self.hover_x,
                self.hover_y,
                self.theme,
            );
        }
        if self.focused_mass != usize::MAX {
            self.renderer.render_focused_mass(
                &self.masses[self.focused_mass],
                matter_antigrav_radius,
                matter_elastic_radius,
                matter_gas_radius,
                self.canvas_w,
                self.theme,
            )
        }
        if self.guide == 0 { // .guide-all
            self.renderer.render_connections(
                &self.masses,
                &self.grab_ids,
                self.canvas_w,
                self.theme,
            );
        }
    }

    pub fn get_hovered(
        &self,
    ) -> String {
        let hover_x = self.hover_x;
        let hover_y = self.hover_y;
        let mut hovered: Vec<usize> = vec![];

        for mass in &self.masses {
            // Get the distance between hover point and Mass center, in px.
            let dx_pxl = mass.x - hover_x; // +ve or -ve
            let dy_pxl = mass.y - hover_y; // +ve or -ve
            let d_pxl = (dx_pxl * dx_pxl + dy_pxl * dy_pxl).sqrt(); // only +ve
            if d_pxl <= mass.radius {
                hovered.push(mass.id);
                continue
            }

            // For masses near the left edge, a lesser-part of their circle
            // could be hovered on the right-edge.
            if mass.x < mass.radius {
                // Get the distance between hover point and Mass center, if the
                // Mass was shifted rightwards by a canvas-width.
                let dx_pxl = mass.x + self.canvas_w - hover_x;
                let d_pxl = (dx_pxl * dx_pxl + dy_pxl * dy_pxl).sqrt();
                if d_pxl <= mass.radius {
                    hovered.push(mass.id);
                    continue
                }
            }

            // For masses near the right edge, a lesser-part of their circle
            // could be hovered on the left-edge.
            if mass.x > self.canvas_w - mass.radius {
                // Get the distance between hover point and Mass center, if the
                // Mass was shifted leftwards by a canvas-width.
                let dx_pxl = mass.x - self.canvas_w - hover_x;
                let d_pxl = (dx_pxl * dx_pxl + dy_pxl * dy_pxl).sqrt();
                if d_pxl <= mass.radius {
                    hovered.push(mass.id);
                    continue
                }
            }
        }
        format!("{:?}", hovered)
    }

}

fn is_tricorn(zero_indexed_id: usize) -> bool {
    let one_indexed_id = zero_indexed_id + 1;
    let square = (one_indexed_id * 8 + 1) as f64;
    let root = square.sqrt();
    root % 1.0 == 0.0
}

fn prev_tricorn(zero_indexed_id: usize) -> usize {
    for i in (0..zero_indexed_id).rev() {
        let result = is_tricorn(i);
        // console::log_1(&format!("result {} i {}", result, i).into());
        if result { return i }
    }
    0
}

fn size_to_radius(size: u8) -> f64 {
    return match size {
        0 => 80.0 * 0.3, // xs
        1 => 80.0 * 0.5, // s
        2 => 80.0 * 0.7, // m
        3 => 80.0 * 1.0, // l
        4 => 80.0 * 1.5, // xl
        _ => 80.0 * 1.0, // l
    }
}
