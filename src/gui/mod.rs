use std::sync::{Arc, Mutex};
use tokio::sync::oneshot;
use diesel::prelude::*;
use eframe::egui;
use diesel_demo::models::Instrument;

pub fn run_gui(connection: Arc<Mutex<MysqlConnection>>, shutdown_rx: oneshot::Receiver<()>) {
    eframe::run_native(
        "Music Store GUI",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Box::new(GuiApp::new(connection, shutdown_rx))),
    )
        .expect("Failed to start GUI");
}

pub struct GuiApp {
    connection: Arc<Mutex<MysqlConnection>>,
    instruments: Vec<Instrument>,
    shutdown_rx: Option<oneshot::Receiver<()>>,
    selected_row: Option<usize>, // To track the selected row
}

impl GuiApp {
    pub fn new(
        connection: Arc<Mutex<MysqlConnection>>,
        shutdown_rx: oneshot::Receiver<()>,
    ) -> Self {
        let instruments = Self::fetch_instruments(&connection);
        Self {
            connection,
            instruments,
            shutdown_rx: Some(shutdown_rx),
            selected_row: None,
        }
    }

    fn fetch_instruments(connection: &Arc<Mutex<MysqlConnection>>) -> Vec<Instrument> {
        let mut conn = connection.lock().unwrap();
        diesel_demo::schema::instrument::dsl::instrument
            .load::<Instrument>(&mut *conn)
            .unwrap_or_else(|_| vec![])
    }
}

impl eframe::App for GuiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Some(mut rx) = self.shutdown_rx.take() {
            if rx.try_recv().is_ok() {
                std::process::exit(0); // Close GUI
            }
            self.shutdown_rx = Some(rx);
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Music Store");
            ui.add_space(10.0);

            // Add Refresh and Clear Table buttons
            ui.horizontal(|ui| {
                if ui.button("Refresh").clicked() {
                    self.instruments = Self::fetch_instruments(&self.connection);
                }

                if ui.button("Clear Table").clicked() {
                    self.instruments.clear();
                    self.selected_row = None; // Clear any selected row as well
                }
            });

            ui.add_space(10.0);

            if let Some(selected_index) = self.selected_row {
                let selected_instrument = &self.instruments[selected_index];
                ui.label(format!("Selected Instrument ID: {}", selected_instrument.id));
                ui.label(format!(
                    "Selected Instrument Make: {}",
                    selected_instrument.make.as_deref().unwrap_or("N/A")
                ));
            } else {
                ui.label("No Instrument Selected");
            }

            ui.add_space(10.0);

            egui::ScrollArea::vertical()
                .max_height(400.0)
                .show(ui, |ui| {
                    egui::Grid::new("instrument_table")
                        .striped(true)
                        .min_col_width(100.0)
                        .show(ui, |ui| {
                            ui.label("ID");
                            ui.label("Make");
                            ui.label("Model");
                            ui.label("Type");
                            ui.label("Country");
                            ui.label("Serial #");
                            ui.label("Updated");
                            ui.label("Line");
                            ui.end_row();

                            for (index, instrument) in self.instruments.iter().enumerate() {
                                let row_start = ui.cursor().min;
                                let row_width = ui.max_rect().width();
                                let row_height = ui.spacing().interact_size.y;
                                let row_rect = egui::Rect::from_min_size(
                                    row_start,
                                    egui::Vec2::new(row_width, row_height),
                                );

                                let is_selected = self.selected_row == Some(index);
                                let bg_color = if is_selected {
                                    egui::Color32::LIGHT_BLUE
                                } else {
                                    egui::Color32::TRANSPARENT
                                };

                                ui.painter()
                                    .rect_filled(row_rect, egui::Rounding::none(), bg_color);

                                ui.label(format!("{}", instrument.id));
                                ui.label(instrument.make.as_deref().unwrap_or("N/A"));
                                ui.label(instrument.model.as_deref().unwrap_or("N/A"));
                                ui.label(instrument.type_.as_deref().unwrap_or("N/A"));
                                ui.label(instrument.country_of_manufacture.as_deref().unwrap_or("N/A"));
                                ui.label(instrument.serial_number.as_deref().unwrap_or("N/A"));
                                ui.label(
                                    instrument
                                        .updated_at
                                        .map(|date| date.format("%Y-%m-%d").to_string())
                                        .unwrap_or_else(|| "N/A".to_string()),
                                );
                                ui.label(instrument.line.as_deref().unwrap_or("N/A"));
                                ui.end_row();

                                if ui.interact(row_rect, egui::Id::new(index), egui::Sense::click()).clicked() {
                                    self.selected_row = Some(index);
                                }
                            }
                        });
                });
        });
    }
}
