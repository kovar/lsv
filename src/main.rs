#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

pub mod models;

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    use models::lorenz::Lorenz;

    let l: Lorenz = Lorenz::new(0.0, 1.0, 0.0, 10.0, 28.0, 8.0 / 3.0);

    // print!("{:?}", l);

    // let mut rng = rand::thread_rng();
    // const N: usize = 500;
    // let dt = 0.1;
    // let w = wiener_process::wiener_fn(&mut rng, N, dt);

    // // write w to a file
    // let file = std::fs::File::create("wiener.txt").unwrap();
    // const BUF_CAPACITY: usize = N * std::mem::size_of::<f64>();
    // println!("Buffer capacity: {}", BUF_CAPACITY);
    // let mut buffer = BufWriter::with_capacity(BUF_CAPACITY, file);
    // for i in 0..w.len() {
    //     writeln!(&mut buffer, "{}", w[i]).unwrap();
    // }

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "eframe template",
        native_options,
        Box::new(|cc| Box::new(lsv::TemplateApp::new(cc))),
    )
}

// when compiling to web using trunk.
#[cfg(target_arch = "wasm32")]
fn main() {
    // Make sure panics are logged using `console.error`.
    console_error_panic_hook::set_once();

    // Redirect tracing to console.log and friends:
    tracing_wasm::set_as_global_default();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::start_web(
            "the_canvas_id", // hardcode it
            web_options,
            Box::new(|cc| Box::new(eframe_template::TemplateApp::new(cc))),
        )
        .await
        .expect("failed to start eframe");
    });
}
