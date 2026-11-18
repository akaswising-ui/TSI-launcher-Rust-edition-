fn main() -> eframe::Result {
    let gpu_runtime = tsi_orbit::GpuRuntimeState::default();
    let mut options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_title("TSI Orbit")
            .with_inner_size([1280.0, 800.0])
            .with_min_inner_size([780.0, 520.0]),
        ..Default::default()
    };
    tsi_orbit::configure_native_options(&mut options, gpu_runtime.clone());

    eframe::run_native(
        "TSI Orbit",
        options,
        Box::new(move |cc| {
            Ok(Box::new(tsi_orbit::LauncherApp::new(cc, gpu_runtime.clone())))
        }),
    )
}
