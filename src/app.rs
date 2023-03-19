#![warn(unused_variables)]

use crate::models::lorenz::Lorenz;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    // label: String,

    // this how you opt-out of serialization of a member
    #[serde(skip)]
    x_0: f64,
    y_0: f64,
    z_0: f64,
    t: f64,
    dt: f64,
    sigma: f64,
    rho: f64,
    beta: f64,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            x_0: 1.0,
            y_0: 1.0,
            z_0: 1.0,
            t: 50.0,
            dt: 0.01,
            sigma: 10.0,
            rho: 28.0,
            beta: 8.0 / 3.0,
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default() // use this to return default app state (without solving the Lorenz system)
    }
}

impl eframe::App for TemplateApp {
    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {
            x_0,
            y_0,
            z_0,
            t,
            dt,
            sigma,
            rho,
            beta,
        } = self;

        // let mut ls_default = Lorenz::default();
        let mut ls_solution = Lorenz::new(*x_0, *y_0, *z_0, *t, *dt, *sigma, *rho, *beta);

        // solve the Lorenz system
        // ls_default.solve();
        ls_solution.solve();

        let n = ls_solution.x.len();

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
                ui.menu_button("Settings", |ui| {
                    if ui.button("DO NOT CLICK THIS").clicked() {
                        _frame.close();
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Simulation Settings");

            ui.add(egui::Slider::new(sigma, 0.0..=20.0).text("σ"));
            ui.horizontal(|ui| {
                ui.button("-")
                    .on_hover_text("Decrease σ by 0.1")
                    .on_hover_cursor(egui::CursorIcon::PointingHand)
                    .clicked()
                    .then(|| *sigma -= 0.1);
                ui.button("+")
                    .on_hover_text("Increase σ by 0.1")
                    .on_hover_cursor(egui::CursorIcon::PointingHand)
                    .clicked()
                    .then(|| *sigma += 0.1);
            });

            ui.add(egui::Slider::new(rho, 0.0..=50.0).text("ρ"));
            ui.horizontal(|ui| {
                ui.button("-")
                    .on_hover_text("Decrease ρ by 0.1")
                    .on_hover_cursor(egui::CursorIcon::PointingHand)
                    .clicked()
                    .then(|| *rho -= 0.1);
                ui.button("+")
                    .on_hover_text("Increase ρ by 0.1")
                    .on_hover_cursor(egui::CursorIcon::PointingHand)
                    .clicked()
                    .then(|| *rho += 0.1);
            });

            ui.add(egui::Slider::new(beta, 0.0..=10.0).text("β"));
            ui.horizontal(|ui| {
                ui.button("-")
                    .on_hover_text("Decrease β by 0.1")
                    .on_hover_cursor(egui::CursorIcon::PointingHand)
                    .clicked()
                    .then(|| *beta -= 0.001);
                ui.button("+")
                    .on_hover_text("Increase β by 0.1")
                    .on_hover_cursor(egui::CursorIcon::PointingHand)
                    .clicked()
                    .then(|| *beta += 0.001);
            });

            ui.add(egui::Slider::new(dt, 0.001..=0.1).text("dt"));
            ui.horizontal(|ui| {
                ui.button("-")
                    .on_hover_text("Decrease dt by 0.001")
                    .on_hover_cursor(egui::CursorIcon::PointingHand)
                    .clicked()
                    .then(|| *dt -= 0.001);
                ui.button("+")
                    .on_hover_text("Increase dt by 0.001")
                    .on_hover_cursor(egui::CursorIcon::PointingHand)
                    .clicked()
                    .then(|| *dt += 0.001);
            });

            ui.separator();

            ui.heading("Initial Conditions");
            ui.label(
                "Time t always starts at 0.0, below you can set the maximum time.
            Also, you can change the initial conditions for x, y and z.",
            );

            ui.add(egui::Slider::new(t, 0.0..=100.0).text("t"));
            ui.horizontal(|ui| {
                ui.button("-")
                    .on_hover_text("Decrease t by 0.1")
                    .on_hover_cursor(egui::CursorIcon::PointingHand)
                    .clicked()
                    .then(|| *t -= 0.1);
                ui.button("+")
                    .on_hover_text("Increase t by 0.1")
                    .on_hover_cursor(egui::CursorIcon::PointingHand)
                    .clicked()
                    .then(|| *t += 0.1);
            });

            ui.add(egui::Slider::new(x_0, -20.0..=20.0).text("x₀"));
            ui.horizontal(|ui| {
                ui.button("-")
                    .on_hover_text("Decrease x₀ by 0.1")
                    .on_hover_cursor(egui::CursorIcon::PointingHand)
                    .clicked()
                    .then(|| *x_0 -= 0.1);
                ui.button("+")
                    .on_hover_text("Increase x₀ by 0.1")
                    .on_hover_cursor(egui::CursorIcon::PointingHand)
                    .clicked()
                    .then(|| *x_0 += 0.1);
            });

            ui.add(egui::Slider::new(y_0, -20.0..=20.0).text("y₀"));
            ui.horizontal(|ui| {
                ui.button("-")
                    .on_hover_text("Decrease y₀ by 0.1")
                    .on_hover_cursor(egui::CursorIcon::PointingHand)
                    .clicked()
                    .then(|| *y_0 -= 0.1);
                ui.button("+")
                    .on_hover_text("Increase y₀ by 0.1")
                    .on_hover_cursor(egui::CursorIcon::PointingHand)
                    .clicked()
                    .then(|| *y_0 += 0.1);
            });

            ui.add(egui::Slider::new(z_0, -20.0..=20.0).text("z₀"));
            ui.horizontal(|ui| {
                ui.button("-")
                    .on_hover_text("Decrease z₀ by 0.1")
                    .on_hover_cursor(egui::CursorIcon::PointingHand)
                    .clicked()
                    .then(|| *z_0 -= 0.1);
                ui.button("+")
                    .on_hover_text("Increase z₀ by 0.1")
                    .on_hover_cursor(egui::CursorIcon::PointingHand)
                    .clicked()
                    .then(|| *z_0 += 0.1);
            });

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("powered by ");
                    ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                    ui.label(" and ");
                    ui.hyperlink_to(
                        "eframe",
                        "https://github.com/emilk/egui/tree/master/crates/eframe",
                    );
                    ui.label(".");
                });

                ui.separator();

                // reset to default values
                if ui.button("Reset").clicked() {
                    *x_0 = 1.0;
                    *y_0 = 1.0;
                    *z_0 = 1.0;
                    *sigma = 10.0;
                    *rho = 28.0;
                    *beta = 8.0 / 3.0;
                    *dt = 0.01;
                    *t = 50.0;
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Lorenz System Viewer");
            ui.horizontal(|ui| {
                ui.add(egui::github_link_file!(
                    "https://github.com/kovar/lsv/",
                    "Source code."
                ));
                egui::warn_if_debug_build(ui);
            });
            ui.separator();

            ui.label(
                "Below is a 2D plot of the Lorenz system in the x-y plane. 
            Double click to reset view.",
            );

            let xy: egui::plot::PlotPoints = (0..n)
                .map(|i| {
                    let x = ls_solution.x[i];
                    let y = ls_solution.y[i];
                    [x, y]
                })
                .collect();

            let xy_line = egui::plot::Line::new(xy);

            egui::plot::Plot::new("Lorenz System XY")
                // .width(420.0)
                // .height(240.0)
                .view_aspect(2.0)
                .show(ui, |plot_ui| plot_ui.line(xy_line));
        });

        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally choose either panels OR windows.");
            });
        }
    }
}
