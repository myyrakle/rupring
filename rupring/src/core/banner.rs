use crate::application_properties::ApplicationProperties;

const DEFAULT_BANNER_TEXT: &str = r#",------.                       ,--.                
|  .--. ',--.,--. ,---. ,--.--.`--',--,--,  ,---.  
|  '--'.'|  ||  || .-. ||  .--',--.|      \| .-. | 
|  |\  \ '  ''  '| '-' '|  |   |  ||  ||  |' '-' ' 
`--' '--' `----' |  |-' `--'   `--'`--''--'.`-  /  
                 `--'                      `---'   "#;

pub fn print_banner(application_properties: &ApplicationProperties) {
    if !application_properties.banner.enabled {
        return;
    }

    if let Some(location) = &application_properties.banner.location {
        let bytes = std::fs::read(location).expect("Failed to find banner file");

        let text = match application_properties
            .banner
            .charset
            .to_uppercase()
            .as_str()
        {
            "UTF-8" => String::from_utf8(bytes).unwrap_or_default(),
            "UTF-16" => {
                let utf16_bytes = bytes
                    .chunks(2)
                    .map(|b| u16::from_le_bytes([b[0], b[1]]))
                    .collect::<Vec<u16>>();
                String::from_utf16(&utf16_bytes).unwrap_or_default()
            }
            _ => String::from_utf8(bytes).unwrap_or_default(),
        };

        println!("{}", text);
    } else {
        println!("{}", DEFAULT_BANNER_TEXT);
    }

    print_app_info();
    println!("");
}

fn print_app_info() {
    let version = env!("CARGO_PKG_VERSION");
    let name = env!("CARGO_PKG_NAME");
    println!("{name} v{version}");
}
