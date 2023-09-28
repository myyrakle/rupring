use std::mem::MaybeUninit;

pub struct RupringApp {
    pub name: String,
    pub version: String,
}

impl RupringApp {
    pub fn run(&mut self) {
        println!("Hello, world!");
    }
}

static mut APP: MaybeUninit<RupringApp> = MaybeUninit::uninit();

pub fn init() {
    unsafe {
        APP = MaybeUninit::new(RupringApp {
            name: String::from("Rupring"),
            version: String::from("0.1.0"),
        });
    }
}

pub fn run_app() {
    init();

    unsafe {
        let app = APP.assume_init_mut();
        app.run();
    }
}
