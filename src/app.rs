

#[derive(serde::Deserialize, serde::Serialize)]
struct Ingredient {
    name: String,
    amount_str: String,
    amount: u32,
    selected: bool,
    selected_amount: u32,
    selected_amount_str: String,
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    ingredients: Vec<Ingredient>,
    selected_ingredient_idx: i32,
    modify_ingredient_amount: bool,

    // // this how you opt-out of serialization of a member
    // #[serde(skip)]
    // value: f32,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            ingredients: Vec::with_capacity(32),
            selected_ingredient_idx: 0,
            modify_ingredient_amount: false,
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

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { 
            ingredients,
            selected_ingredient_idx,
            modify_ingredient_amount,
        } = self;

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
                })
            });
        });

        // egui::SidePanel::left("side_panel").show(ctx, |ui| {
        //     ui.heading("Side Panel");

        //     ui.horizontal(|ui| {
        //         ui.label("Write something: ");
        //         ui.text_edit_singleline(label);
        //     });

        //     ui.add(egui::Slider::new(value, 0.0..=10.0).text("value"));
        //     if ui.button("Increment").clicked() {
        //         *value += 1.0;
        //     }

        //     ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
        //         ui.horizontal(|ui| {
        //             ui.spacing_mut().item_spacing.x = 0.0;
        //             ui.label("powered by ");
        //             ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        //             ui.label(" and ");
        //             ui.hyperlink_to(
        //                 "eframe",
        //                 "https://github.com/emilk/egui/tree/master/crates/eframe",
        //             );
        //             ui.label(".");
        //         });
        //     });
        // });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            // ui.heading("eframe template");
            // ui.hyperlink("https://github.com/emilk/eframe_template");
            // ui.add(egui::github_link_file!(
            //     "https://github.com/emilk/eframe_template/blob/master/",
            //     "Source code."
            // ));
            ui.heading("Recipe Calculator");

            ui.add_space(10.0);

            ui.vertical(|ui| {
                ui.label("Modify ingredient");

                ui.horizontal(|ui| {
                    ui.radio_value(&mut *modify_ingredient_amount, false, "Off");
                    ui.radio_value(&mut *modify_ingredient_amount, true, "On")
                });
            });

            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.label("Ingredient");
                ui.label("Amount");
            });

            for (ingredient_index, ingredient) in ingredients.iter_mut().enumerate() {
                ui.horizontal(|ui| {

                    let radio_button = egui::RadioButton::new(*selected_ingredient_idx == ingredient_index as i32, "");
                    if (ui.add_enabled(*modify_ingredient_amount, radio_button)).clicked() {
                        *selected_ingredient_idx = ingredient_index as i32;
                    }

                    let ingredient_selected = *selected_ingredient_idx == ingredient_index as i32;

                    let ingredient_enabled = !*modify_ingredient_amount || ingredient_selected;

                    ui.add_enabled(ingredient_enabled, egui::TextEdit::singleline(&mut ingredient.name));
                    ui.add_enabled(ingredient_enabled, egui::TextEdit::singleline(&mut ingredient.amount_str));

                    if let Ok(parsed) = ingredient.amount_str.parse::<u32>() {
                        ingredient.amount = parsed;
                    }
                });
            }

            ui.add_space(10.0);

            if ui.button("Add ingredient").clicked() {
                let new_ingredient = Ingredient {
                    name: "New ingredient".to_string(),
                    amount_str: String::new(),
                    amount: 0,
                    selected: false,
                    selected_amount: 0,
                    selected_amount_str: String::new(),
                };
                ingredients.push(new_ingredient);
            }

            egui::warn_if_debug_build(ui);
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
