use wasm_bindgen::JsCast;

use crate::mass::Mass;

const PI2: f64 = std::f64::consts::PI * 2.0;

pub struct Renderer {
    ctx: web_sys::CanvasRenderingContext2d,
}

fn init_canvas_2d(canvas_id: String) -> web_sys::CanvasRenderingContext2d {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id(&canvas_id).unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();
    let ctx = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();
    ctx.set_image_smoothing_enabled(true);
    ctx.set_line_width(4.0);
    return ctx;
}

impl Renderer {
    pub fn new(canvas_id: String) -> Renderer {
        let ctx = init_canvas_2d(canvas_id);
        Renderer { ctx }
    }
    
    pub fn clear_canvas(
        &self,
        x: f64,
        y: f64,
        w: f64,
        h: f64,
        theme: u8,
        trail: u8,
    ) {
        match trail {
            // .trail-none
            0 => self.ctx.clear_rect(x, y, w, h),
            // .trail-fade
            1 => {
                let trails_color = match theme {
                    0 => "#29133705", // purple
                    1 => "#00000005", // black
                    2 => "#ffffff05", // white
                    _ => "",
                };
                self.ctx.set_fill_style(&trails_color.into());
                self.ctx.fill_rect(x, y, w, h);
            },
            // .trail-full
            2 => (),
            // not a recognised trail code
            _ => (),
        }
    }

    pub fn render_masses(
        &self,
        masses: &Vec<Mass>,
        masses_antigrav: &Vec<usize>,
        masses_elastic: &Vec<usize>,
        masses_gas: &Vec<usize>,
        matter_antigrav_radius: f64,
        matter_elastic_radius: f64,
        matter_gas_radius: f64,
        canvas_w: f64,
        guide: u8,
        theme: u8,
    ) {
        // Render anitigrav radiuses in magenta.
        self.ctx.set_fill_style(&"#87028780".into());
        self.ctx.begin_path();
        for id in masses_antigrav {
            let mass = &masses[*id];
            let radius = matter_antigrav_radius;
            self.ctx.move_to(mass.x, mass.y);
            let _ = self.ctx.arc(mass.x, mass.y, radius, 0.0, PI2);
            if mass.x < radius {
                self.ctx.move_to(mass.x + canvas_w, mass.y);
                let _ = self.ctx.arc(mass.x + canvas_w, mass.y, radius, 0.0, PI2);
            } else if mass.x > canvas_w - radius {
                self.ctx.move_to(mass.x - canvas_w, mass.y);
                let _ = self.ctx.arc(mass.x - canvas_w, mass.y, radius, 0.0, PI2);
            }
        }
        self.ctx.fill();

        // Render elastic radiuses in red.
        self.ctx.set_fill_style(&"#8a063580".into());
        self.ctx.begin_path();
        for id in masses_elastic {
            let mass = &masses[*id];
            let radius = matter_elastic_radius;
            self.ctx.move_to(mass.x, mass.y);
            let _ = self.ctx.arc(mass.x, mass.y, radius, 0.0, PI2);
            if mass.x < radius {
                self.ctx.move_to(mass.x + canvas_w, mass.y);
                let _ = self.ctx.arc(mass.x + canvas_w, mass.y, radius, 0.0, PI2);
            } else if mass.x > canvas_w - radius {
                self.ctx.move_to(mass.x - canvas_w, mass.y);
                let _ = self.ctx.arc(mass.x - canvas_w, mass.y, radius, 0.0, PI2);
            }
        }
        self.ctx.fill();

        // Render gas radiuses in cyan.
        self.ctx.set_fill_style(&"#015e9080".into());
        self.ctx.begin_path();
        for id in masses_gas {
            let mass = &masses[*id];
            let radius = matter_gas_radius;
            self.ctx.move_to(mass.x, mass.y);
            let _ = self.ctx.arc(mass.x, mass.y, radius, 0.0, PI2);
            if mass.x < radius {
                self.ctx.move_to(mass.x + canvas_w, mass.y);
                let _ = self.ctx.arc(mass.x + canvas_w, mass.y, radius, 0.0, PI2);
            } else if mass.x > canvas_w - radius {
                self.ctx.move_to(mass.x - canvas_w, mass.y);
                let _ = self.ctx.arc(mass.x - canvas_w, mass.y, radius, 0.0, PI2);
            }
        }
        self.ctx.fill();

        // Render all the centers, unless '.guide-none'.
        if guide != 2 {
            let center_color = match theme {
                0 => "#fff", // purple
                1 => "#fff", // black
                2 => "#000", // white
                _ => "",
            };
            self.ctx.set_fill_style(&center_color.into());

            self.ctx.begin_path();
            for mass in masses {
                self.ctx.move_to(mass.x, mass.y);
                let _ = self.ctx.arc(mass.x, mass.y, 4.0, 0.0, PI2);
                if mass.x < 4.0 {
                    self.ctx.move_to(mass.x + canvas_w, mass.y);
                    let _ = self.ctx.arc(mass.x + canvas_w, mass.y, 4.0, 0.0, PI2);
                } else if mass.x > canvas_w - 4.0 {
                    self.ctx.move_to(mass.x - canvas_w, mass.y);
                    let _ = self.ctx.arc(mass.x - canvas_w, mass.y, 4.0, 0.0, PI2);
                }
            }
        }

        self.ctx.fill();
    }

    pub fn render_grabbed_masses(
        &self,
        masses: &Vec<Mass>,
        grab_masses: &Vec<usize>,
        matter_antigrav_radius: f64,
        matter_elastic_radius: f64,
        matter_gas_radius: f64,
        canvas_w: f64,
        theme: u8,
    ) {
        let grab_color = match theme {
            0 => "#fff3", // purple
            1 => "#fff3", // black
            2 => "#0003", // white
            _ => "",
        };
        self.ctx.set_fill_style(&grab_color.into());

        self.ctx.begin_path();
        for id in grab_masses {
            let mass = &masses[*id];
            let radius = match mass.matter {
                0 => matter_antigrav_radius,
                1 => matter_elastic_radius,
                2 => matter_gas_radius,
                _ => matter_elastic_radius,
            };
            self.ctx.move_to(mass.x, mass.y);
            let _ = self.ctx.arc(mass.x, mass.y, radius, 0.0, PI2);
            if mass.x < radius {
                self.ctx.move_to(mass.x + canvas_w, mass.y);
                let _ = self.ctx.arc(mass.x + canvas_w, mass.y, radius, 0.0, PI2);
            } else if mass.x > canvas_w - radius {
                self.ctx.move_to(mass.x - canvas_w, mass.y);
                let _ = self.ctx.arc(mass.x - canvas_w, mass.y, radius, 0.0, PI2);
            }
        }
        self.ctx.fill();
    }

    pub fn render_hover(
        &self,
        masses: &Vec<Mass>,
        matter_antigrav_radius: f64,
        matter_elastic_radius: f64,
        matter_gas_radius: f64,
        canvas_w: f64,
        hover_x: f64,
        hover_y: f64,
        theme: u8,
    ) {
        if hover_x < 0.0 || hover_y < 0.0 { return } // mouse is outside canvas

        let hover_color = match theme {
            0 => "#fff1", // purple
            1 => "#fff1", // black
            2 => "#0001", // white
            _ => "",
        };

        for mass in masses {
            let radius = match mass.matter {
                0 => matter_antigrav_radius,
                1 => matter_elastic_radius,
                2 => matter_gas_radius,
                _ => matter_elastic_radius,
            };

            // Detect if the Mass is overlapping the cylidrical clumpiverse.
            let mass_overlaps_left = mass.x < radius;
            let mass_overlaps_right = mass.x > canvas_w - radius;
    
            // Get the distance between hover point and Mass center, in px.
            let dx_pxl = mass.x - hover_x; // +ve or -ve
            let dy_pxl = mass.y - hover_y; // +ve or -ve
            let d_pxl = (dx_pxl * dx_pxl + dy_pxl * dy_pxl).sqrt(); // only +ve

            // Highlight the circle.
            if d_pxl <= radius {
                self.ctx.set_fill_style(&hover_color.into());
                self.ctx.begin_path();
                let _ = self.ctx.arc(mass.x, mass.y, radius, 0.0, PI2);
                if mass_overlaps_left {
                    let _ = self.ctx.arc(mass.x + canvas_w, mass.y, radius, 0.0, PI2);
                } else if mass_overlaps_right {
                    let _ = self.ctx.arc(mass.x - canvas_w, mass.y, radius, 0.0, PI2);
                }
                self.ctx.fill();
            }

            // For masses near the left edge, a lesser-part of their circle
            // could be hovered on the right-edge.
            if mass_overlaps_left {
                // Get the distance between hover point and Mass center, if the
                // Mass was shifted rightwards by a canvas-width.
                let dx_pxl = mass.x + canvas_w - hover_x;
                let d_pxl = (dx_pxl * dx_pxl + dy_pxl * dy_pxl).sqrt();
                // Highlight the circle.
                if d_pxl <= radius {
                    self.ctx.set_fill_style(&hover_color.into());
                    self.ctx.begin_path();
                    let _ = self.ctx.arc(mass.x, mass.y, radius, 0.0, PI2);
                    let _ = self.ctx.arc(mass.x + canvas_w, mass.y, radius, 0.0, PI2);
                    self.ctx.fill();
                }
            }

            // For masses near the right edge, a lesser-part of their circle
            // could be hovered on the left-edge.
            if mass_overlaps_right {
                // Get the distance between hover point and Mass center, if the
                // Mass was shifted leftwards by a canvas-width.
                let dx_pxl = mass.x - canvas_w - hover_x;
                let d_pxl = (dx_pxl * dx_pxl + dy_pxl * dy_pxl).sqrt();
                // Highlight the circle.
                if d_pxl <= radius {
                    self.ctx.set_fill_style(&hover_color.into());
                    self.ctx.begin_path();
                    let _ = self.ctx.arc(mass.x, mass.y, radius, 0.0, PI2);
                    let _ = self.ctx.arc(mass.x - canvas_w, mass.y, radius, 0.0, PI2);
                    self.ctx.fill();
                }
            }
        }
    }

    pub fn render_focused_mass(
        &self,
        mass: &Mass,
        matter_antigrav_radius: f64,
        matter_elastic_radius: f64,
        matter_gas_radius: f64,
        canvas_w: f64,
        theme: u8,
    ) {
        let focus_color = match theme {
            0 => "#fff8", // purple
            1 => "#fff8", // black
            2 => "#0008", // white
            _ => "",
        };
        self.ctx.set_stroke_style(&focus_color.into());

        let radius = match mass.matter {
            0 => matter_antigrav_radius,
            1 => matter_elastic_radius,
            2 => matter_gas_radius,
            _ => matter_elastic_radius,
        };

        // Detect if the Mass is overlapping the cylidrical clumpiverse.
        let mass_overlaps_left = mass.x < radius;
        let mass_overlaps_right = mass.x > canvas_w - radius;

        // Highlight the circle.
        self.ctx.move_to(mass.x, mass.y);
        self.ctx.begin_path();
        let _ = self.ctx.arc(mass.x, mass.y, radius, 0.0, PI2);

        // For masses near the left or right edge, a lesser-part of their circle
        // could appear on the other edge.
        if mass_overlaps_left {
            self.ctx.stroke();
            self.ctx.move_to(mass.x + canvas_w, mass.y);
            self.ctx.begin_path();
            let _ = self.ctx.arc(mass.x + canvas_w, mass.y, radius, 0.0, PI2);
        }
        if mass_overlaps_right {
            self.ctx.stroke();
            self.ctx.move_to(mass.x - canvas_w, mass.y);
            self.ctx.begin_path();
            let _ = self.ctx.arc(mass.x - canvas_w, mass.y, radius, 0.0, PI2);
        }

        self.ctx.stroke();
    }

    // @TODO don’t render every connection twice
    pub fn render_connections(
        &self,
        masses: &Vec<Mass>,
        _grab_masses: &Vec<usize>,
        canvas_w: f64,
        theme: u8,
    ) {
        let quarter_canvas_w = canvas_w * 0.25;
        let threequarter_canvas_w = canvas_w * 0.75;

        match theme {
            0 => self.ctx.set_stroke_style(&"#fff6".into()), // purple
            1 => self.ctx.set_stroke_style(&"#fff6".into()), // black
            2 => self.ctx.set_stroke_style(&"#0003".into()), // white
            _ => (),
        }

        for mass in masses { // @TODO make this rendering understand grab_x and grab_y
            self.ctx.begin_path();
            for id in &mass.connections {
                let m = &masses[*id];
                self.ctx.move_to(mass.x, mass.y);

                // If this Mass is in the left quadrant, and the other Mass is
                // in the right quadrant:
                if mass.x < quarter_canvas_w && m.x > threequarter_canvas_w {
                    // Draw a line off both sides.
                    self.ctx.line_to(m.x - canvas_w, m.y);
                    self.ctx.move_to(m.x, m.y);
                    self.ctx.line_to(mass.x + canvas_w, mass.y);
                // Otherwise, if this Mass is in the right quadrant, and the
                // other Mass is in the left quadrant:
                } else if mass.x > threequarter_canvas_w && m.x < quarter_canvas_w {
                    // Draw a line off both sides.
                    self.ctx.line_to(m.x + canvas_w, m.y);
                    self.ctx.move_to(m.x, m.y);
                    self.ctx.line_to(mass.x - canvas_w, mass.y);
                    // Otherwise, cylindrical clumpiverse rules don’t apply.
                } else {
                    self.ctx.line_to(m.x, m.y);
                }

                // If this Mass is being dragged from any point not on a line
                // down its exact center:
                if mass.grab_x != 1.0 {
                    // If both Masses are in the right quadrant:
                    if mass.x > threequarter_canvas_w && m.x > threequarter_canvas_w {
                        // Draw an extra line off the left side.
                        self.ctx.move_to(mass.x - canvas_w, mass.y);
                        self.ctx.line_to(m.x - canvas_w, m.y);
                    // Otherwise if they are both in the left quadrant:
                    } else if mass.x < quarter_canvas_w && m.x < quarter_canvas_w {
                        // Draw an extra line off the right side.
                        self.ctx.move_to(mass.x + canvas_w, mass.y);
                        self.ctx.line_to(m.x + canvas_w, m.y);
                    }
                }
                // Without the block above, when you drag a Mass off one
                // edge, connection-line is missing on the other edge.
                // @TODO figure out why — it’s not a very neat solution
            }
            self.ctx.stroke();
        }

        // self.ctx.begin_path();
        // self.ctx.set_stroke_style(&"#0f06".into());
        // for grab_id in grab_masses {
        //     let mass = &masses[*grab_id];
        //     for id in &mass.connections {
        //         let m = &masses[*id];
        //         self.ctx.move_to(mass.x, mass.y + 10.0);
        //         self.ctx.line_to(m.x, m.y + 10.0);
        //     }
        // }
        // self.ctx.stroke();

    }
}
