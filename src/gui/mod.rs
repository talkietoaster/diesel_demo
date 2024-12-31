use eframe::egui;
use std::sync::{Arc, Mutex};
use tokio::sync::oneshot;
use diesel::prelude::*;
use diesel_demo::models::Instrument;

pub fn run_gui(connection: Arc<Mutex<MysqlConnection>>, shutdown_rx: oneshot::Receiver<()>) {
    eframe::run_native(
        "Instrument GUI",
        eframe::NativeOptions::default(),
        Box::new(|_cc| {
            Box::new(GuiApp::new(connection, shutdown_rx))
        }),
    )
        .expect("Failed to start GUI");
}

pub struct GuiApp {
    connection: Arc<Mutex<MysqlConnection>>,
    instruments: Vec<Instrument>,
    shutdown_rx: Option<oneshot::Receiver<()>>,
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
        // Check for shutdown signal
        if let Some(mut rx) = self.shutdown_rx.take() {
            if rx.try_recv().is_ok() {
                std::process::exit(0);
            }
            self.shutdown_rx = Some(rx);
        }



        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Instruments");

            egui::Grid::new("instrument_table")
                .striped(true)
                .show(ui, |ui| {
                    // Table header
                    ui.label("ID");
                    ui.label("Make");
                    ui.label("Model");
                    ui.end_row();

                    // Table rows
                    for instrument in &self.instruments {
                        ui.label(format!("{}", instrument.id));
                        ui.label(instrument.make.as_deref().unwrap_or("N/A"));
                        ui.label(instrument.model.as_deref().unwrap_or("N/A"));
                        ui.end_row();
                    }
                });

            if ui.button("Refresh").clicked() {
                self.instruments = Self::fetch_instruments(&self.connection);
            }
        });
    }
}
