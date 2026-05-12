use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

#[derive(Parser, Debug)]
#[command(author, version, about = "A high-performance zero-copy CSV scanner")]
struct Args {
    #[arg(short, long)]
    file: String,

    #[arg(short, long, default_value_t = 2)]
    column: usize,

    #[arg(short, long)]
    query: String,
}

#[derive(Debug)]
struct Field {
    s: usize,
    e: usize,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let start_time = Instant::now();
    csv_parser(&args.file, args.column, &args.query)?;
    let duration = start_time.elapsed();

    let metadata = std::fs::metadata(&args.file)?;
    let file_size_mb = metadata.len() as f64 / 1_048_576.0;
    let mb_per_sec = file_size_mb / duration.as_secs_f64();

    println!("\n-----------------------------------------");
    println!("Processed {:.2} MB at {:.2} MB/s", file_size_mb, mb_per_sec);
    println!("Finished in - {:?}", duration);

    Ok(())
}

fn csv_parser(
    filename: &str,
    target_col: usize,
    query: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(filename)?;
    let mut reader = BufReader::new(file);

    let mut row_tray: Vec<Field> = Vec::with_capacity(20);
    let mut in_quotes = false;

    loop {
        let buff = reader.fill_buf()?;
        let length = buff.len();
        if length == 0 {
            break;
        }

        let mut start = 0;
        let mut i = 0;

        while i < length {
            match buff[i] {
                34 => {
                    if i + 1 < length && buff[i + 1] == 34 {
                        i += 1; // Skip the second quote
                    } else {
                        in_quotes = !in_quotes;
                    }
                }
                // Delimiter Logic
                44 if !in_quotes => {
                    row_tray.push(Field { s: start, e: i });
                    start = i + 1;
                }
                // Newline Logic
                10 if !in_quotes => {
                    row_tray.push(Field { s: start, e: i });

                    process_row(buff, &row_tray, target_col, query);

                    row_tray.clear();
                    start = i + 1;
                }
                _ => {}
            }
            i += 1;
        }

        let consumed = length;
        reader.consume(consumed);
    }
    Ok(())
}

fn process_row(buff: &[u8], row: &[Field], target_col: usize, search: &str) {
    if let Some(field) = row.get(target_col) {
        let field_bytes = &buff[field.s..field.e];

        if field_bytes == search.as_bytes() {
            if let Some(id_field) = row.get(0) {
                let id = std::str::from_utf8(&buff[id_field.s..id_field.e]).unwrap_or("Invalid");
                let found = std::str::from_utf8(field_bytes).unwrap_or("Invalid");
                println!("[Match Found] ID: {} | Value: {}", id, found);
            }
        }
    }
}
