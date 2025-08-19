use eframe::egui;
use std::error::Error;
use std::fs::File;
use std::path::PathBuf;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 700.0]),
        ..Default::default()
    };
    
    eframe::run_native(
        "CSV Viewer",
        options,
        Box::new(|_cc| Box::new(CsvViewerApp::default())),
    )
}

#[derive(Default)]
struct CsvViewerApp {
    csv_data: Vec<Vec<String>>,
    headers: Vec<String>,
    file_path: Option<PathBuf>,
    error_message: Option<String>,
}

impl CsvViewerApp {
    fn load_csv(&mut self, path: PathBuf) -> Result<(), Box<dyn Error>> {
        let file = File::open(&path)?;
        let mut reader = csv::Reader::from_reader(file);
        
        // Read headers
        let headers = reader.headers()?.clone();
        self.headers = headers.iter().map(|s| s.to_string()).collect();
        
        // Read data rows
        let mut data = Vec::new();
        for result in reader.records() {
            let record = result?;
            let row: Vec<String> = record.iter().map(|s| s.to_string()).collect();
            data.push(row);
        }
        
        self.csv_data = data;
        self.file_path = Some(path);
        self.error_message = None;
        Ok(())
    }
}

impl eframe::App for CsvViewerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("CSV Viewer");
            
            ui.horizontal(|ui| {
                if ui.button("📁 Open CSV File").clicked() {
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("CSV files", &["csv"])
                        .pick_file() 
                    {
                        if let Err(e) = self.load_csv(path) {
                            self.error_message = Some(format!("Error loading CSV: {}", e));
                        }
                    }
                }
                
                if let Some(ref path) = self.file_path {
                    ui.label(format!("File: {}", path.display()));
                }
            });
            
            ui.separator();
            
            // Show error message if any
            if let Some(ref error) = self.error_message {
                ui.colored_label(egui::Color32::RED, error);
                ui.separator();
            }
            
            // Display CSV data if loaded
            if !self.csv_data.is_empty() && !self.headers.is_empty() {
                ui.label(format!("Rows: {} | Columns: {}", 
                    self.csv_data.len(), 
                    self.headers.len()
                ));
                
                ui.separator();
                
                egui::ScrollArea::both()
                    .id_source("csv_scroll")
                    .show(ui, |ui| {
                        egui::Grid::new("csv_grid")
                            .striped(true)
                            .min_col_width(100.0)
                            .max_col_width(300.0)
                            .show(ui, |ui| {
                                // Header row
                                for header in &self.headers {
                                    ui.strong(header);
                                }
                                ui.end_row();
                                
                                // Data rows
                                for (row_idx, row) in self.csv_data.iter().enumerate() {
                                    // Add row number
                                    ui.weak(format!("{}", row_idx + 1));
                                    
                                    for (col_idx, cell) in row.iter().enumerate() {
                                        if col_idx < self.headers.len() {
                                            // Truncate very long cell content for display
                                            let display_text = if cell.len() > 50 {
                                                format!("{}...", &cell[..47])
                                            } else {
                                                cell.clone()
                                            };
                                            
                                            ui.label(display_text)
                                                .on_hover_text(cell); // Show full content on hover
                                        }
                                    }
                                    ui.end_row();
                                }
                            });
                    });
            } else if self.file_path.is_none() {
                ui.vertical_centered(|ui| {
                    ui.add_space(100.0);
                    ui.heading("Welcome to CSV Viewer");
                    ui.label("Click 'Open CSV File' to get started");
                    ui.add_space(20.0);
                    ui.label("Features:");
                    ui.label("• Browse and select CSV files from any directory");
                    ui.label("• View data in a clean, scrollable table");
                    ui.label("• Hover over truncated cells to see full content");
                    ui.label("• Automatic row and column counting");
                });
            }
        });
    }
}


