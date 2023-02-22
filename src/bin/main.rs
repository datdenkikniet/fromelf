use std::io::Write;

fn usage(name: &str) {
    log::error!(
        "Usage: {} <input elf> [output binary]. The default value of [output binary] is output.bin",
        name
    );
}

fn main() {
    pretty_env_logger::init();

    let mut args = std::env::args();

    let exec_path = args.next().unwrap();

    if args.len() < 1 || args.len() > 3 {
        log::error!("Expected two arguments.");
        usage(&exec_path);
        std::process::exit(1);
    }

    let path = args.next().unwrap();
    let out_path = args.next().unwrap_or("output.bin".into());

    let bin_data = if let Ok(data) = fromelf::elf_file_to_bin(&path) {
        data
    } else {
        log::error!("Could not open {}", path);
        std::process::exit(1);
    };

    let mut write_file = if let Ok(file) = std::fs::File::create(&out_path) {
        file
    } else {
        log::error!("Could not write to {}", out_path);
        std::process::exit(1);
    };

    if write_file.write_all(&bin_data).is_err() {
        log::error!("Failed to write data to {}", out_path);
    }

    log::info!(
        "Succesfully wrote {} bytes of binary data to {}",
        bin_data.len(),
        out_path
    );
}
