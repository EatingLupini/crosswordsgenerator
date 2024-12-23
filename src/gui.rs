use std::{collections::HashMap, ops::RangeInclusive};

use eframe::egui::{self, Frame, Label, RichText, Sense, UiBuilder, Widget};
use egui::Color32;
use egui_extras::{Size, StripBuilder};
use egui_modal::{Icon, Modal};

use crate::{generate, get_definitions, utils::{Board, Dir, Solution, WordPos}};


pub struct BaseApp<'a> {
    words_len: HashMap<usize, Vec<&'a str>>,
    words_def: &'a serde_json::Value,

    width: usize,
    height: usize,
    grid: Vec<Vec<char>>,
    shuffle: bool,
    rep_words: bool,
    modal: Modal,
    result: Option<Solution>,
    definitions: Vec<(WordPos, String)>,
}


impl<'a> BaseApp<'a> {
    pub fn new(ctx: &egui::Context, words_len: HashMap<usize, Vec<&'a str>>, words_def: &'a serde_json::Value) -> Self {
        let modal = Modal::new(ctx, "modal_result");
        Self {
            words_len,
            words_def,

            width: 5,
            height: 5,
            grid: vec![vec![' '; 5]; 5],
            shuffle: false,
            rep_words: false,
            modal,
            result: None,
            definitions: Vec::new(),
        }
    }
}


impl<'a> eframe::App for BaseApp<'a> {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let dark_mode = ui.visuals().dark_mode;
            let faded_color = ui.visuals().window_fill();
            let faded_color = |color: Color32| -> Color32 {
                use egui::Rgba;
                let t = if dark_mode { 0.95 } else { 0.8 };
                egui::lerp(Rgba::from(color)..=Rgba::from(faded_color), t).into()
            };

            self.modal.show(|ui| {
                let sol = self.result.as_ref().unwrap();
                
                //self.modal.title(ui, "Solution found!");
                ui.vertical_centered(|ui| {
                    if sol.found {
                        ui.heading("Solution found!");
                        self.modal.icon(ui, Icon::Success);
                    }
                    else {
                        ui.heading("Solution not found.");
                        self.modal.icon(ui, Icon::Warning);
                    }
                });
                ui.separator();

                self.modal.frame(ui, |ui| {
                    ui.label(format!("Visited Nodes: {}", sol.visited_nodes));
                    ui.label(format!("Time Elapsed: {} ms", sol.time_elapsed));
                });
                self.modal.buttons(ui, |ui| {
                    if self.modal.button(ui, "Close").clicked() {
                        self.result = None;
                    };
                }); 
            });

            if !self.result.is_none() {
                self.modal.open();
            }
            
            StripBuilder::new(ui)
                .size(Size::exact(64.0))
                .size(Size::exact(64.0))
                .size(Size::remainder())
                .size(Size::exact(64.0))
                .vertical(|mut strip| {
                    strip.cell(|ui| {
                        ui.painter().rect_filled(
                            ui.available_rect_before_wrap(),
                            0.0,
                            faded_color(Color32::BLUE),
                        );
                        ui.vertical_centered(|ui| {
                            ui.heading("Crosswords Generator");
                        });
                    });
                    strip.cell(|ui| {
                        ui.separator();
                        egui::Grid::new("GridSettings")
                            .num_columns(2)
                            .show(ui, |ui| {
                                ui.label("Size:");
                                ui.horizontal(|ui| {
                                    let resp_w = ui.add(
                                        egui::DragValue::new(&mut self.width)
                                            .range(RangeInclusive::new(2, 100)));
                                    let resp_h = ui.add(
                                        egui::DragValue::new(&mut self.height)
                                            .range(RangeInclusive::new(2, 100)));

                                    if resp_w.changed() {
                                        let diff: i32 = self.width as i32 - self.grid.first().unwrap().len() as i32;
                                        for v in self.grid.iter_mut() {
                                            for _ in 0..diff.abs() {
                                                if diff < 0 {
                                                    v.pop();
                                                }
                                                else {
                                                    v.push(' ');
                                                }
                                            }
                                        }
                                    }

                                    if resp_h.changed() {
                                        let diff: i32 = self.height as i32 - self.grid.len() as i32;
                                        for _ in 0..diff.abs() {
                                            if diff < 0 {
                                                self.grid.pop();
                                            }
                                            else {
                                                self.grid.push(vec![' '; self.width]);
                                            }
                                        }
                                    }
                                });
                                ui.end_row();

                                ui.label("Shuffle:");
                                ui.horizontal(|ui| {
                                    ui.add(egui::Checkbox::without_text(&mut self.shuffle));
                                });
                                ui.end_row();

                                ui.label("Repeat Words:");
                                ui.horizontal(|ui| {
                                    ui.add(egui::Checkbox::without_text(&mut self.rep_words));
                                });
                                ui.end_row();
                            });
                    });
                    strip.cell(|ui| {
                        ui.separator();
                        egui::ScrollArea::both().auto_shrink([false; 2]).show(ui, |ui| {
                            ui.vertical_centered(|ui| {
                                for j in 0..self.grid.len() {
                                    let v = &mut self.grid[j];
                                    ui.horizontal(|ui| {
                                        for i in 0..v.len() {
                                            let e = v[i];
                                            
                                            // Build definitions string
                                            let mut def_string: String = "".to_owned();
                                            if !self.definitions.is_empty() {
                                                for (wp, def) in &self.definitions {
                                                    if wp.x == i && wp.y == j {
                                                        if !def_string.is_empty() {
                                                            def_string.push_str("\n");
                                                        }
                                                        def_string.push_str(if wp.dir == Dir::HOR {"Hor: "} else {"Ver: "});
                                                        def_string.push_str(def);
                                                    }
                                                }
                                            }

                                            // Draw cell
                                            let response = ui
                                            .scope_builder(
                                                UiBuilder::new()
                                                    .sense(Sense::click()),
                                                |ui| {
                                                    let response = ui.response();
                                                    let visuals = ui.style().interact(&response);
                                                    
                                                    Frame::canvas(ui.style())
                                                        .fill(if e == '#' {Color32::BLACK} else {Color32::WHITE})
                                                        .stroke(visuals.bg_stroke)
                                                        .inner_margin(ui.spacing().menu_margin)
                                                        .show(ui, |ui| {
                                                            ui.set_width(16.0);
                                                            ui.set_height(16.0);
                                                            
                                                            ui.vertical_centered(|ui| {
                                                                let response = Label::new(
                                                                    RichText::new(if e != '#' {e} else {' '})
                                                                        .color(Color32::BLACK)
                                                                        .size(16.0)
                                                                ).ui(ui);
                                                                
                                                                // switch from black to white and viceversa
                                                                if response.clicked() {
                                                                    v[i] = if e == '#' {' '} else {'#'};
                                                                }
                                                                
                                                                // show definition
                                                                if e != '#' && !def_string.is_empty() {
                                                                    response.on_hover_text(&def_string);
                                                                }
                                                            });
                                                        });
                                                },
                                            )
                                            .response;
                                            
                                            // switch from black to white and viceversa
                                            if response.clicked() {
                                                v[i] = if e == '#' {' '} else {'#'};
                                            }

                                            // show definition
                                            if e != '#' && !def_string.is_empty()  {
                                                response.on_hover_text(&def_string);
                                            }
                                        }
                                    });
                                }
                            });
                        });
                        
                    });
                    strip.cell(|ui| {
                        let button_width = 128.0;
                        let padding = (ui.available_width() - button_width * 2.0) / 2.0;
                        ui.separator();
                        ui.vertical_centered(|ui| {
                            ui.add_space(4.0);
                            ui.horizontal(|ui| {
                                ui.add_space(padding);

                                // GENERATE
                                let response = ui.add_sized((128.0, 48.0), egui::Button::new("Generate!"));
                                if response.clicked() {
                                    // Clean grid
                                    for j in 0..self.grid.len() {
                                        let v = &mut self.grid[j];
                                        for i in 0..v.len() {
                                            if v[i] != '#' {
                                                v[i] = ' ';
                                            }
                                        }
                                    }
    
                                    // Create board
                                    let mut board = Board::new(self.width, self.height);
    
                                    // Add black cells
                                    for j in 0..self.grid.len() {
                                        let v = &self.grid[j];
                                        for i in 0..v.len() {
                                            board.set(i, j, v[i]);
                                        }
                                    }
    
                                    // Process
                                    self.result = Some(generate(&mut board, self.words_len.clone(), self.shuffle, self.rep_words));

                                    // Update definitions
                                    if self.result.as_ref().unwrap().found {
                                        self.definitions = get_definitions(&board, self.words_def);
                                    }
    
                                    // Update grid with board data
                                    for j in 0..self.grid.len() {
                                        let v = &mut self.grid[j];
                                        for i in 0..v.len() {
                                            v[i] = board.get(i, j);
                                        }
                                    }
                                };
    
                                // RESET
                                let response = ui.add_sized((128.0, 48.0), egui::Button::new("Reset"));
                                if response.clicked() {
                                    self.grid = vec![vec![' '; self.width]; self.height];
                                    self.definitions.clear();
                                }
                            });
                        });
                    });
                }
            );
        });
    }
}

