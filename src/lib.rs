mod app;

pub use app::LauncherApp;

#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
fn android_main(app: winit::platform::android::activity::AndroidApp) {
    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Info),
    );

    let options = eframe::NativeOptions {
        android_app: Some(app),
        ..Default::default()
    };

    if let Err(error) = eframe::run_native(
        "TSI Orbit",
        options,
        Box::new(|cc| Ok(Box::new(LauncherApp::new(cc)))),
    ) {
        log::error!("TSI Orbit exited with an error: {error}");
    }
}
