
#[derive(serde::Deserialize, serde::Serialize)]
struct Ingredient {
    name: String,
    amount_str: String,
    unit: String,
    selected_amount_str: String,
    modified_amount_str: String,
}

impl Default for Ingredient {
    fn default() -> Self {
        Self {
            name: String::new(),
            amount_str: String::new(),
            unit: String::new(),
            selected_amount_str: String::new(),
            modified_amount_str: String::new(),
        }
    }
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    ingredients: Vec<Ingredient>,
    selected_ingredient_idx: i32,
    selected_ingredient_ratio: f32,
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
            selected_ingredient_ratio: 0.0,
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
            selected_ingredient_ratio,
            modify_ingredient_amount,
        } = self;

        let spacing = 10.0;

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

            ui.heading("Recipe Calculator");

            ui.add_space(10.0);

            ui.vertical(|ui| {
                ui.label("Modify ingredient");

                ui.horizontal(|ui| {
                    ui.radio_value(&mut *modify_ingredient_amount, false, "Off");
                    ui.radio_value(&mut *modify_ingredient_amount, true, "On")
                });
            });

            ui.add_space(spacing);

            ui.horizontal(|ui| {
                ui.style_mut().override_text_style = (egui::TextStyle::Monospace).into();
                ui.add_space(2.0);
                ui.label("Ingredient");
                ui.add_space(87.0);
                ui.label("Amount");
                ui.add_space(12.0);
                ui.label("Unit");
                ui.add_space(27.0);
                ui.add_visible(*modify_ingredient_amount, egui::Label::new("Original"));
            });

            let mut remove_ingredient_idx = (false, 0);

            let ingredient_name_width = 125.0;
            let ingredient_amount_width = 45.0;
            let ingredient_unit_width = 25.0;

            for (ingredient_index, ingredient) in ingredients.iter_mut().enumerate() {
                ui.horizontal(|ui| {
                    ui.add_enabled(!*modify_ingredient_amount, egui::TextEdit::singleline(&mut ingredient.name).desired_width(ingredient_name_width));

                    let radio_button = egui::RadioButton::new(*selected_ingredient_idx == ingredient_index as i32, "");
                    if (ui.add_visible(*modify_ingredient_amount, radio_button)).clicked() {
                        *selected_ingredient_idx = ingredient_index as i32;
                    }

                    let ingredient_selected = *selected_ingredient_idx == ingredient_index as i32;
                    let ingredient_enabled = !*modify_ingredient_amount || ingredient_selected;

                    if ingredient_selected && *modify_ingredient_amount {
                        ui.add_enabled(ingredient_enabled, egui::TextEdit::singleline(&mut ingredient.selected_amount_str).desired_width(ingredient_amount_width));

                        let normal_amount_result = ingredient.amount_str.parse::<f32>();
                        let selected_amount_result = ingredient.selected_amount_str.parse::<f32>();
                        match (normal_amount_result, selected_amount_result) {
                            (Ok(normal_amount), Ok(selected_amount)) => *selected_ingredient_ratio = selected_amount / normal_amount,
                            _ => (),
                        };
                    }
                    else if *modify_ingredient_amount {
                        if let Ok(ingredient_amount) = ingredient.amount_str.parse::<f32>() {
                            let modified_amount = *selected_ingredient_ratio * ingredient_amount;
                            ingredient.modified_amount_str = modified_amount.to_string();
                        }
                        ui.add_enabled(false, egui::TextEdit::singleline(&mut ingredient.modified_amount_str).desired_width(ingredient_amount_width));
                    }
                    else {
                        ui.add_enabled(ingredient_enabled, egui::TextEdit::singleline(&mut ingredient.amount_str).desired_width(ingredient_amount_width));

                        if ingredient.selected_amount_str.is_empty() {
                            ingredient.selected_amount_str = ingredient.amount_str.clone();
                        }
                    }

                    ui.add_enabled(!*modify_ingredient_amount, egui::TextEdit::singleline(&mut ingredient.unit).desired_width(ingredient_unit_width));

                    if ui.add_enabled(!*modify_ingredient_amount, egui::Button::new("X").small()).clicked() {
                        remove_ingredient_idx = (true, ingredient_index);
                    }

                    ui.horizontal(|ui| {
                        ui.set_visible(*modify_ingredient_amount);
                        let mut original_amount = ingredient.amount_str.clone();
                        original_amount.push_str(" ");
                        original_amount.push_str(&ingredient.unit);
                        ui.add_enabled(false, egui::TextEdit::singleline(&mut original_amount).desired_width(ingredient_amount_width));
                    });

                    // ui.label(selected_ingredient_ratio.to_string());
                    // ui.label(ingredient.amount_str.to_string());
                    // ui.label(ingredient.selected_amount_str.to_string());
                    // ui.label(ingredient.modified_amount_str.to_string());
                });
            }

            if remove_ingredient_idx.0 {
                ingredients.remove(remove_ingredient_idx.1);
            }

            ui.add_space(spacing);

            if ui.add_enabled(!*modify_ingredient_amount, egui::Button::new("Add ingredient")).clicked() {
                ingredients.push(Ingredient::default());
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
