fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_title("TSI Orbit")
            .with_inner_size([1280.0, 800.0])
            .with_min_inner_size([780.0, 520.0]),
        ..Default::default()
    };

    eframe::run_native(
        "TSI Orbit",
        options,
        Box::new(|cc| Ok(Box::new(tsi_orbit::LauncherApp::new(cc)))),
    )
}
