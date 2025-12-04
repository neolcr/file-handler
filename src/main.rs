use std::io::Write;

fn ensure_flush() {
    match std::io::stdout().flush() {
        Ok(_) => {}
        Err(e) => println!("Flush failed with {}", e),
    };
}

fn list_directory() {
    let current_dir = std::fs::canonicalize(".").expect("Failed to get dir");

    print!("\n>>>> Current DIR: {:?} . Enter directory: ", current_dir);

    ensure_flush();

    let mut input = String::new();

    match std::io::stdin().read_line(&mut input) {
        Ok(_) => match std::fs::canonicalize(input.trim()) {
            Ok(path) => match path.to_str() {
                Some(abs_path) => {
                    dbg!("Will use path: {}", abs_path.trim());

                    match std::fs::read_dir(&abs_path) {
                        Ok(rd) => {
                            dbg!("Read input directory -> OK");
                            for entry in rd {
                                match entry {
                                    Ok(entry) => match entry.file_type() {
                                        Ok(file_type) => {
                                            if file_type.is_dir() {
                                                println!("d -> {:?}", entry.file_name());
                                            } else if file_type.is_file() {
                                                println!("f -> {:?}", entry.file_name());
                                            }
                                        }
                                        Err(e) => println!("File type error {}", e),
                                    },
                                    Err(e) => println!("Error dir entry: {}", e),
                                }
                            }
                        }
                        Err(e) => println!("Read dir error: {}", e),
                    }
                }
                None => println!("None"),
            },
            Err(e) => println!("Str {}", e),
        },
        Err(e) => println!("Stdin error: {}", e),
    }
}

fn enter_directory() {
    let current_dir = std::fs::canonicalize(".").expect("Failed to get dir");

    println!(
        "\n>>>> Current DIR: {:?} . Type directory to enter",
        current_dir
    );

    ensure_flush();

    let mut input = String::new();

    match std::io::stdin().read_line(&mut input) {
        Ok(_) => match std::fs::canonicalize(input.trim()) {
            Ok(path) => match path.to_str() {
                Some(abs_path) => {
                    println!("Will use path: {}", abs_path.trim());

                    match std::fs::read_dir(&abs_path) {
                        Ok(_) => match std::env::set_current_dir(abs_path.trim()) {
                            Ok(_) => {
                                let current_dir =
                                    std::fs::canonicalize(".").expect("Failed to get dir");
                                println!("\n#### Current DIR: {:?}", current_dir);
                            }
                            Err(e) => println!("Error {}", e),
                        },
                        Err(e) => println!("Read dir error: {}", e),
                    }
                }
                None => println!("None"),
            },
            Err(e) => println!("Str {}", e),
        },
        Err(e) => println!("Stdin error: {}", e),
    }
}

fn create_file() {
    let current_dir = std::fs::canonicalize(".").expect("Failed to get current directory");

    println!("Current directory: {:?}", current_dir);

    ensure_flush();

    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => {
            println!("Will create file: {}", input);
            match std::fs::File::create(input) {
                Ok(ok) => println!("File successfully created: {:?}", ok),
                Err(e) => println!("{}", e),
            };
        }
        Err(e) => println!("Error: {}", e),
    }
}

fn append_file() {
    let current_dir = std::fs::canonicalize(".").expect("Failed to get current directory");

    println!("Current directory: {:?}", current_dir);

    ensure_flush();

    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => {
            println!("Will append file: {}", input);
            match std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(input)
            {
                Ok(mut ok) => {
                    println!("File successfully created with mode option: {:?}", ok);
                    let mut input = String::new();
                    match std::io::stdin().read_line(&mut input) {
                        Ok(_) => match ok.write(input.as_bytes()) {
                            Ok(_) => println!("Content added to file"),
                            Err(e) => println!("Error {}", e),
                        },
                        Err(e) => println!("Error {}", e),
                    }
                }
                Err(e) => println!("{}", e),
            };
        }
        Err(e) => println!("Error: {}", e),
    }
}

fn read_file() {
    let current_dir = std::fs::canonicalize(".").expect("Failed to get current directory");

    println!("Current directory: {:?}", current_dir);

    ensure_flush();
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => match std::fs::read_to_string(input) {
            Ok(content) => println!("{}", content),
            Err(e) => println!("{}", e),
        },
        Err(e) => println!("Error {}", e),
    }
}

fn delete_file() {
    let current_dir = std::fs::canonicalize(".").expect("Failed to get current directory");
    println!("Current directory: {:?}", current_dir);
    ensure_flush();
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => match std::fs::remove_file(&input) {
            Ok(_) => println!("Removed file: {}", input),
            Err(e) => println!("Error {}", e),
        },
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("Recived args: {:?}", args);
    ensure_flush();
    let options = vec![
        "l -> list",
        "e -> enter directory",
        "c -> create file",
        "a -> append text to file",
        "r -> read file",
        "d -> delete file",
        "q -> quit",
    ];
    loop {
        println!("\n>>>> Main loop: Enter option:");
        for o in options.iter() {
            println!("{}", o);
        }
         

        let mut input = String::new();

        match std::io::stdin().read_line(&mut input) {
            Ok(_) => match input.trim() {
                "l" => list_directory(),
                "e" => enter_directory(),
                "c" => create_file(),
                "a" => append_file(),
                "r" => read_file(),
                "d" => delete_file(),
                "q" => {
                    println!("Salir");
                    break;
                }
                _ => println!("Otra opcion"),
            },
            Err(e) => println!("Stdin error: {}", e),
        }

        ensure_flush();
        input.clear();
    }
}
