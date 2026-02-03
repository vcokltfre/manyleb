use manyleb::{parse, format as format_schema};

fn format(file: &String) -> Result<(), String> {
    let input = std::fs::read_to_string(file).unwrap();
    let schema = parse(input.as_str())?;

    let formatted = format_schema(&schema);
    std::fs::write(file, formatted).unwrap();

    Ok(())
}

fn verify(file: &String) -> Result<(), String> {
    let input = std::fs::read_to_string(file).unwrap();
    let schema = parse(input.as_str())?;

    schema.verify()
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 1 {
        eprintln!("Usage: manyleb <command> [args...]");
        std::process::exit(1);
    }

    let command = &args[1];

    match command.as_str() {
        "format" | "fmt" => {
            if args.len() < 3 {
                eprintln!("Usage: manyleb format <input-file>");
                std::process::exit(1);
            }

            let input_file = &args[2];
            if let Err(err) = format(input_file) {
                eprintln!("Error formatting file {}: {}", input_file, err);
                std::process::exit(1);
            }
        },
        "verify" => {
            if args.len() < 3 {
                eprintln!("Usage: manyleb verify <input-file>");
                std::process::exit(1);
            }

            let input_file = &args[2];
            if let Err(err) = verify(input_file) {
                eprintln!("Verification failed for file {}: {}", input_file, err);
                std::process::exit(1);
            } else {
                println!("Verification succeeded for file {}", input_file);
            }
        }
        _ => {
            eprintln!("Unknown command: {}", command);
            std::process::exit(1);
        }
    }
}
