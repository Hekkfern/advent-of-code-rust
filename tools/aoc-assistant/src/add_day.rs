use aoc_utils::path_utils;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub struct AddDayParameters {
    pub year: u32,
    pub day: u32,
    pub session_key: String,
    pub force: bool,
}

pub enum Error {
    NetworkError,
    ParseError,
    FolderAlreadyExists,
    IoError,
    MissingTemplate,
    InvalidSessionKey,
}

fn get_template_path() -> PathBuf {
    path_utils::get_workspace_root().join("tools/aoc-assistant/templates/new-puzzle/")
}

pub fn add_new_day(params: AddDayParameters) -> Result<(), Error> {
    // Check if the puzzle folder exists
    let puzzle_path = path_utils::get_workspace_root()
        .join(format!("puzzles/{}/{:0>2}", params.year, params.day));
    if puzzle_path.is_dir() && !params.force {
        return Err(Error::FolderAlreadyExists);
    }
    // Generate folder structure
    let templates_path = get_template_path();
    std::fs::create_dir_all(&puzzle_path).map_err(|_| Error::IoError)?;
    generate_cargo_file(&templates_path, &puzzle_path, params.year, params.day)?;
    generate_inputs_folder(&puzzle_path, params.year, params.day, &params.session_key)?;
    generate_solutions_folder(&puzzle_path)?;
    generate_src_folder(&templates_path, &puzzle_path, params.year, params.day)?;
    generate_tests_folder(&templates_path, &puzzle_path, params.year, params.day)?;

    Ok(())
}

fn generate_inputs_folder(
    puzzle_path: &Path,
    year: u32,
    day: u32,
    session_key: &str,
) -> Result<(), Error> {
    // Generate folder structure
    let inputs_path = puzzle_path.join("data/input");
    std::fs::create_dir_all(&inputs_path).map_err(|_| Error::IoError)?;
    std::fs::File::create(&inputs_path.join("input_part1_test1.txt"))
        .map_err(|_| Error::IoError)?;
    std::fs::File::create(&inputs_path.join("input_part2_test1.txt"))
        .map_err(|_| Error::IoError)?;
    // Download data from server
    let body = get_input_data_from_server(year, day, session_key)?;
    // Write the input data to a file
    std::fs::write(inputs_path.join("input.txt"), body).map_err(|_| Error::IoError)?;
    Ok(())
}

fn generate_solutions_folder(puzzle_path: &Path) -> Result<(), Error> {
    let solutions_path = puzzle_path.join("data/solutions");
    std::fs::create_dir_all(&solutions_path).map_err(|_| Error::IoError)?;
    let files = [
        "solution_part1.txt",
        "solution_part1_test1.txt",
        "solution_part2.txt",
        "solution_part2_test1.txt",
    ];
    for file in files.iter() {
        std::fs::File::create(solutions_path.join(file)).map_err(|_| Error::IoError)?;
    }
    Ok(())
}

fn generate_src_folder(
    templates_path: &Path,
    puzzle_path: &Path,
    year: u32,
    day: u32,
) -> Result<(), Error> {
    // Generate src folder
    let src_path = puzzle_path.join("src");
    std::fs::create_dir_all(&src_path).map_err(|_| Error::IoError)?;
    generate_src_files(&templates_path, &src_path, year, day)?;
    Ok(())
}

fn generate_tests_folder(
    templates_path: &Path,
    puzzle_path: &Path,
    year: u32,
    day: u32,
) -> Result<(), Error> {
    // Generate tests folder
    let tests_path = puzzle_path.join("tests");
    std::fs::create_dir_all(&tests_path).map_err(|_| Error::IoError)?;
    generate_tests_files(&templates_path, &tests_path, year, day)?;
    Ok(())
}

fn get_input_data_from_server(year: u32, day: u32, session_key: &str) -> Result<String, Error> {
    // Download data from server
    let url = format!("https://adventofcode.com/{year}/day/{day}/input");
    let client = reqwest::blocking::Client::new();
    let response = match client
        .get(&url)
        .header(reqwest::header::COOKIE, format!("session={session_key}"))
        .send()
    {
        Ok(resp) => resp,
        Err(_) => return Err(Error::NetworkError),
    };
    // Check if the response is successful
    if !response.status().is_success() {
        return Err(Error::InvalidSessionKey);
    }
    // Parse the response body
    let body = match response.text() {
        Ok(text) => text,
        Err(_) => return Err(Error::ParseError),
    };
    Ok(body)
}

fn generate_tests_files(
    templates_path: &Path,
    tests_path: &Path,
    year: u32,
    day: u32,
) -> Result<(), Error> {
    let template_values = HashMap::from([
        ("year", year.to_string()),
        ("long_day", format!("{day:0>2}")),
    ]);
    let template_content =
        match std::fs::read_to_string(templates_path.join("tests/solution_tests.rs.template")) {
            Ok(content) => content,
            Err(_) => return Err(Error::MissingTemplate),
        };
    let template = leon::Template::parse(&template_content).unwrap();
    let mut file = match std::fs::File::create(tests_path.join("solution_tests.rs")) {
        Ok(file) => file,
        Err(_) => return Err(Error::IoError),
    };
    template
        .render_into(&mut file, &template_values)
        .map_err(|_| Error::IoError)?;
    Ok(())
}

fn generate_src_files(
    templates_path: &Path,
    src_path: &Path,
    year: u32,
    day: u32,
) -> Result<(), Error> {
    // Generate "lib.rs" file
    std::fs::copy(
        templates_path.join("src/lib.rs.template"),
        src_path.join("lib.rs"),
    )
    .map_err(|_| Error::IoError)?;
    // Generate "bin" folder
    let bin_path = src_path.join("bin");
    std::fs::create_dir_all(&bin_path).map_err(|_| Error::IoError)?;
    // Generate "aoc_p1.rs" and "aoc_p2.rs" files
    let files = [
        (
            "aoc_XXXX_YY_p1.rs.template",
            format!("aoc_{year}_{day:0>2}_p1.rs"),
        ),
        (
            "aoc_XXXX_YY_p2.rs.template",
            format!("aoc_{year}_{day:0>2}_p2.rs"),
        ),
    ];
    let template_values = HashMap::from([
        ("year", year.to_string()),
        ("long_day", format!("{day:0>2}")),
    ]);
    for (template_filename, new_filename) in files.iter() {
        let template_content =
            match std::fs::read_to_string(templates_path.join("src/bin").join(template_filename)) {
                Ok(content) => content,
                Err(_) => return Err(Error::MissingTemplate),
            };
        let template = leon::Template::parse(&template_content).unwrap();
        let mut file = match std::fs::File::create(bin_path.join(new_filename)) {
            Ok(file) => file,
            Err(_) => return Err(Error::IoError),
        };
        template
            .render_into(&mut file, &template_values)
            .map_err(|_| Error::IoError)?;
    }

    Ok(())
}

fn generate_cargo_file(
    templates_path: &Path,
    puzzle_path: &Path,
    year: u32,
    day: u32,
) -> Result<(), Error> {
    let template_values = HashMap::from([
        ("year", year.to_string()),
        ("long_day", format!("{day:0>2}")),
    ]);
    let template_content = match std::fs::read_to_string(templates_path.join("Cargo.toml.template"))
    {
        Ok(content) => content,
        Err(_) => return Err(Error::MissingTemplate),
    };
    let template = leon::Template::parse(&template_content).unwrap();
    let mut file = match std::fs::File::create(puzzle_path.join("Cargo.toml")) {
        Ok(file) => file,
        Err(_) => return Err(Error::IoError),
    };
    template
        .render_into(&mut file, &template_values)
        .map_err(|_| Error::IoError)?;
    Ok(())
}
