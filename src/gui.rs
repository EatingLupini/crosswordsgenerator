use std::collections::HashMap;

use eframe::egui::{self, Frame, Label, RichText, Sense, UiBuilder, Widget};
use egui::Color32;
use egui_extras::{Size, StripBuilder};
use egui_modal::{Icon, Modal};

use crate::{generate, utils::{Board, Solution}};


pub struct BaseApp<'a> {
    width: usize,
    height: usize,
    words_len: HashMap<usize, Vec<&'a str>>,
    shuffle: bool,
    rep_words: bool,
    selected: bool,
    modal: Modal,
    result: Option<Solution>,
}


impl<'a> BaseApp<'a> {
    pub fn new(ctx: &egui::Context, words_len: HashMap<usize, Vec<&'a str>>) -> Self {
        let modal = Modal::new(ctx, "modal_result");
        Self {
            width: 5,
            height: 5,
            words_len,
            shuffle: false,
            rep_words: false,
            selected: false,
            modal,
            result: None,
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
                    //self.modal.body(ui, format!("This is a modal. {:?}", sol));
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
                                    ui.add(egui::DragValue::new(&mut self.width));
                                    ui.add(egui::DragValue::new(&mut self.height));
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
                                for _ in 0..self.height {
                                    ui.horizontal(|ui| {
                                        for _ in 0..self.width {
                                            let response = ui
                                            .scope_builder(
                                                UiBuilder::new()
                                                    .sense(Sense::click()),
                                                |ui| {
                                                    let response = ui.response();
                                                    let visuals = ui.style().interact(&response);
    
                                                    Frame::canvas(ui.style())
                                                        .fill(if self.selected {Color32::BLACK} else {Color32::WHITE})
                                                        .stroke(visuals.bg_stroke)
                                                        .inner_margin(ui.spacing().menu_margin)
                                                        .show(ui, |ui| {
                                                            ui.set_width(16.0);
                                                            ui.set_height(16.0);

                                                            ui.vertical_centered(|ui| {
                                                                Label::new(
                                                                    RichText::new(format!(""))
                                                                        .color(Color32::BLACK)
                                                                        .size(16.0)
                                                                ).ui(ui);
                                                            });
                                                        });
                                                },
                                            )
                                            .response;
    
                                            if response.clicked() {
                                                self.selected = !self.selected;
                                            }
                                        }
                                    });
                                }
                            });
                        });
                        
                    });
                    strip.cell(|ui| {
                        ui.separator();
                        ui.vertical_centered(|ui| {
                            ui.add_space(4.0);
                            let response = ui.add_sized((128.0, 48.0), egui::Button::new("Generate!"));
                            if response.clicked() {
                                let mut board = Board::new(self.width, self.height);
                                self.result = Some(generate(&mut board, self.words_len.clone(), self.shuffle, self.rep_words));
                            };
                        });
                    });
                }
            );
        });
    }
}

