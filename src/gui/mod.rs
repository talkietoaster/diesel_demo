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
                                // Draw row with a clickable response
                                let row_interaction = ui.interact(
                                    ui.available_rect_before_wrap(),
                                    egui::Id::new(index),
                                    egui::Sense::click(),
                                );

                                let bg_color = if self.selected_row == Some(index) {
                                    egui::Color32::LIGHT_BLUE
                                } else {
                                    egui::Color32::TRANSPARENT
                                };

                                // Draw the background for the row
                                let rect = row_interaction.rect;
                                ui.painter()
                                    .rect_filled(rect, egui::Rounding::none(), bg_color);

                                // Table columns
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

                                // Update selected row on click
                                if row_interaction.clicked() {
                                    self.selected_row = Some(index);
                                }
                            }
                        });
                });

            if ui.button("Refresh").clicked() {
                self.instruments = Self::fetch_instruments(&self.connection);
            }
        });
    }
}
