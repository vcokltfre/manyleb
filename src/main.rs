use manyleb::{format as format_schema, parse};


fn format(file: &String) -> Result<(), String> {
    let input = std::fs::read_to_string(file).unwrap();
    let schema = parse(input.as_str(), false)?;

    let formatted = format_schema(&schema);
    std::fs::write(file, formatted).unwrap();

    Ok(())
}

fn verify(file: &String) -> Result<(), String> {
    let input = std::fs::read_to_string(file).unwrap();
    let schema = parse(input.as_str(), true)?;

    schema.verify()
}

fn docs(input_file: &String, output_file: &String) -> Result<(), String> {
    let input = std::fs::read_to_string(input_file).unwrap();
    let schema = parse(input.as_str(), true)?;

    let docs = manyleb::generate_docs(&schema);
    std::fs::write(output_file, docs).unwrap();

    Ok(())
}

fn summary(input_file: &String, output_file: &String) -> Result<(), String> {
    let input = std::fs::read_to_string(input_file).unwrap();
    let schema = parse(input.as_str(), true)?;

    let summary = manyleb::generate_summary(&schema);
    std::fs::write(output_file, summary).unwrap();

    Ok(())
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
        }
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
        "docs" => {
            if args.len() < 4 {
                eprintln!("Usage: manyleb docs <input-file> <output-file>");
                std::process::exit(1);
            }

            let input_file = &args[2];
            let output_file = &args[3];

            if let Err(err) = docs(input_file, output_file) {
                eprintln!("Error generating docs from file {}: {}", input_file, err);
                std::process::exit(1);
            }
        }
        "summary" => {
            if args.len() < 4 {
                eprintln!("Usage: manyleb summary <input-file> <output-file>");
                std::process::exit(1);
            }

            let input_file = &args[2];
            let output_file = &args[3];

            if let Err(err) = summary(input_file, output_file) {
                eprintln!("Error generating summary from file {}: {}", input_file, err);
                std::process::exit(1);
            }
        }
        _ => {
            eprintln!("Unknown command: {}", command);
            std::process::exit(1);
        }
    }
}
