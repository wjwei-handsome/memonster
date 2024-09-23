use clap::Parser;
use regex::Regex;
use std::thread;
use std::time::Duration;

const MAXGB: usize = 500;
// Parse input with units
fn parse_memory_size(input: &str) -> Result<usize, String> {
    // A regular expression to match a number followed by an optional unit
    let re = Regex::new(r"(?i)^([\d\.]+)([kmgb]?b?)$").unwrap();

    // if not match, return error
    if let Some(caps) = re.captures(input) {
        let num_str = &caps[1]; // num
        let unit_str = &caps[2].to_lowercase(); // unit (lowercase)

        // parse number to f64
        let num = num_str
            .parse::<f64>()
            .map_err(|_| format!("Invalid value: {}", num_str))?;

        // match unit
        let multiplier = match unit_str.as_str() {
            "b" | "" => 1,                    // Byte
            "k" | "kb" => 1024,               // Kilobyte
            "m" | "mb" => 1024 * 1024,        // Megabyte
            "g" | "gb" => 1024 * 1024 * 1024, // Gigabyte
            _ => return Err(format!("Invalid unit: {}", unit_str)),
        };

        // calculate memory size
        let memory_size = (num * multiplier as f64) as usize;
        if memory_size == 0 {
            return Err(format!("wtf! It's zero? {}", num_str));
        } else if memory_size > MAXGB * 1024 * 1024 * 1024 {
            return Err(format!("Are you crazy for eat {} bytes?", memory_size));
        }
        Ok(memory_size)
    } else {
        Err(format!(
            "What shit are you typing: {}, unit must in [b,kb,mb,gb](Case insensitive)",
            input
        ))
    }
}

// eat memory
fn occupy_memory(size_in_bytes: usize) -> Vec<u8> {
    let mut memory = Vec::with_capacity(size_in_bytes);
    memory.resize(size_in_bytes, 0xff); // fill memory with 0xff
    memory
}

/// I AM A MEMORY MONSTER 🤖
#[derive(Parser, Debug)]
#[command(version, about, long_about = None, author)]
#[command(
help_template =
"{name} -- {about}\n\nVersion: {version}\n\nAuthors: {author}\
    \n\n{usage-heading} {usage}\n\n{all-args}"
) // change template more!
]
struct Args {
    /// Amount of memory to occupy, support units: B, KB, MB, GB
    #[arg(short, long, required = true)]
    memory: String,
    // Dry run for test
    #[arg[short, long, required = false]]
    dry: bool,
}

fn main() {
    // parse args
    let args = Args::parse();
    let mem_str = args.memory;
    let dryrun = args.dry;

    // get memory size
    match parse_memory_size(&mem_str) {
        Ok(memory_size) => {
            println!("Nom nom nom.... {} Bytes", memory_size);
            if !dryrun {
                // let _memory = occupy_memory(memory_size);
                // sleep 30s for monitoring
                // thread::sleep(Duration::from_secs(30));
            }
        }
        Err(e) => eprintln!("DAMN: {}", e),
    }
}
