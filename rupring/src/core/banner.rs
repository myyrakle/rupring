use crate::application_properties::ApplicationProperties;

const DEFAULT_BANNER_TEXT: &'static str = r#",------.                       ,--.                
|  .--. ',--.,--. ,---. ,--.--.`--',--,--,  ,---.  
|  '--'.'|  ||  || .-. ||  .--',--.|      \| .-. | 
|  |\  \ '  ''  '| '-' '|  |   |  ||  ||  |' '-' ' 
`--' '--' `----' |  |-' `--'   `--'`--''--'.`-  /  
                 `--'                      `---'   "#;

pub fn print_banner(application_properties: &ApplicationProperties) {
    if !application_properties.banner.enabled {
        return;
    }

    println!("{}", DEFAULT_BANNER_TEXT);
    print_app_info();
    println!("");
}

fn print_app_info() {
    let version = env!("CARGO_PKG_VERSION");
    let name = env!("CARGO_PKG_NAME");
    println!("{name} v{version}");
}
