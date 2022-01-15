use std::{env, fs};

fn process(current_dir: &str) -> Result<(), String> {
    for entry in fs::read_dir(current_dir).map_err(|err| format!("{}", err))? {
        let entry = entry.map_err(|err| format!("{}", err))?;
        let path = entry.path();
        let metadata = fs::metadata(&path).map_err(|err| format!("{}", err))?;
        println!(
            "filename: {:?}, filesize: {:?} bytes",
            path.file_name()
                .ok_or("No filename")
                .map_err(|err| err.to_string())?,
            metadata.len()
        );
    }

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Args: {args:?}");
    let mut dir = ".";
    if args.len() >= 2 {
        dir = &args[1];
    }
    if let Err(err) = process(dir) {
        eprintln!("{}", err)
    }
}
