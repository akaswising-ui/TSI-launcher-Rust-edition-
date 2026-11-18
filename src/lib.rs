mod app;
mod gpu;

pub use app::LauncherApp;
pub use gpu::{GpuRuntimeState, configure_native_options};

#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
fn android_main(app: winit::platform::android::activity::AndroidApp) {
    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Info),
    );

    let gpu_runtime = GpuRuntimeState::default();
    let mut options = eframe::NativeOptions {
        android_app: Some(app),
        ..Default::default()
    };
    configure_native_options(&mut options, gpu_runtime.clone());

    if let Err(error) = eframe::run_native(
        "TSI Orbit",
        options,
        Box::new(move |cc| Ok(Box::new(LauncherApp::new(cc, gpu_runtime.clone())))),
    ) {
        log::error!("TSI Orbit exited with an error: {error}");
    }
}
