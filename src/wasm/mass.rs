#[derive(Clone)]
pub struct Mass {
    pub radius: f64,
    pub matter: u8,
    pub connections: Vec<usize>,
    pub grab_x: f64, // offset when being grabbed
    pub grab_y: f64, // offset when being grabbed
    pub id: usize,
    pub start: f64,
    pub vx: f64, // velocity in the x direction
    pub vy: f64, // velocity in the y direction
    pub x: f64,
    pub y: f64,
}

impl Mass {
    pub fn step_sim(
        &mut self,
        dnow: f64,
        canvas_w: f64,
        masses_clone: &Vec<Mass>,
        matter_antigrav_radius: f64,
        matter_elastic_radius: f64,
        matter_gas_radius: f64,
    ) {
        let quarter_canvas_w = canvas_w * 0.25;
        let threequarter_canvas_w = canvas_w * 0.75;

        let radius = match self.matter {
            0 => matter_antigrav_radius,
            1 => matter_elastic_radius,
            2 => matter_gas_radius,
            _ => matter_elastic_radius,
        };

        // Apply gravity to `vy`.
        self.vy -= 0.001;

        // Every Mass can push or pull any of the other Masses.
        'outer: for m in masses_clone {
            if self.id == m.id { continue } // ignore itself

            // Get the x-distance between the two Mass centers, in pixels.
            let dx_pxl; // +ve or -ve

            // If this Mass is in the left quadrant, and the other Mass is in
            // the right quadrant:
            if self.x < quarter_canvas_w && m.x > threequarter_canvas_w {
                // Virtually bring the other Mass off the left edge.
                dx_pxl = m.x - canvas_w - self.x;
            // Otherwise, if this Mass is in the right quadrant, and the other
            // Mass is in the left quadrant:
            } else if self.x > threequarter_canvas_w && m.x < quarter_canvas_w {
                // Virtually bring the other Mass off the right edge.
                dx_pxl = m.x + canvas_w - self.x;
            // Otherwise, cylindrical clumpiverse rules don’t apply.
            } else {
                dx_pxl = m.x - self.x;
            }

            // Get the y-distance between the two Mass centers, in pixels.
            let dy_pxl = m.y - self.y; // +ve or -ve
            // Get the direct distance between the two Mass centers, in pixels.
            let d_pxl = (dx_pxl * dx_pxl + dy_pxl * dy_pxl).sqrt(); // only +ve

            // Push very hard upwards on any Masses less than 2 pixels away.
            // @TODO just push one of them, not both
            if d_pxl < 2.0 {
                self.vy -= 0.2 / radius;
                continue;
            }

            // Push hard on all Masses less than a quarter of a radius away.
            if d_pxl < radius / 4.0 {
                self.vx -= dx_pxl * 0.1 / radius;
                self.vy -= dy_pxl * 0.1 / radius;
                continue;
            }

            // Push on all Masses between a quarter and one radius away.
            if d_pxl < radius {
                self.vx -= dx_pxl * 0.002 / radius;
                self.vy -= dy_pxl * 0.002 / radius;
                continue;
            }

            // Pull on all connected Masses more than a radius away.
            for c_id in &self.connections {
                if m.id != *c_id { continue } // not connected
                self.vx += dx_pxl * 0.002 / radius;
                self.vy += dy_pxl * 0.002 / radius;
                continue 'outer;
            }

            // Ignore (cull) disconnected Masses further than one diameter away.
            if d_pxl > radius * 2.0 { continue }

            // Push on disconnected Masses between a radius and a diameter away.
            self.vx -= dx_pxl * 0.001 / radius;
            self.vy -= dy_pxl * 0.001 / radius;

        }

        // Lose some energy (dampen).
        self.vx *= 0.98;
        self.vy *= 0.98;

        // Apply velocity (which may have been modifed above) to the position.
        let mut x = self.x + dnow * self.vx; // `x` if there were no constraints
        let mut y = self.y + dnow * self.vy; // `y` if there were no constraints

        // Prevent the Mass falling through the floor.
        let low_bound = radius; // minimum `y` allowed
        if y < low_bound { // true if sim would move `y` below the minimum
            y = low_bound; // clamp `y`
            self.vy = 0.0 - self.vy * 0.45; // bounce half reverse velocity
            self.vx *= 0.97; // also, apply friction to vx
        }

        // Apply cylindrical clumpiverse.
        if x < 0.0 {
            x += canvas_w
        }
        x = x % canvas_w;

        // Update the position.
        self.x = x;
        self.y = y;
    }
}
