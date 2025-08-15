use aoc_utils::path_utils;
use std::collections::HashMap;
use std::path::Path;

pub struct GetStatementParameters {
    pub year: u32,
    pub day: u32,
    pub session_key: String,
}

pub enum Error {
    NetworkError,
    ParseError,
    FolderNotFound,
    IoError,
    MissingTemplate,
}

pub fn get_puzzle_statement(params: GetStatementParameters) -> Result<(), Error> {
    // Check if the puzzle folder exists
    let puzzle_path = path_utils::get_workspace_root()
        .join(format!("puzzles/{}/{:0>2}", params.year, params.day));
    if !puzzle_path.is_dir() {
        return Err(Error::FolderNotFound);
    }
    // Download data from server
    let body = get_data_from_server(params.year, params.day, &params.session_key)?;
    // Extract the puzzle statement from the body
    let html_document = visdom::Vis::load(&body).unwrap();
    let articles = html_document.find("article.day-desc");
    let p1 = extract_puzzle_statement(&articles, 0);
    let p2 = extract_puzzle_statement(&articles, 1);
    // Write the body to a file
    let template_values = HashMap::from([
        ("year", params.year.to_string()),
        ("day", params.day.to_string()),
        ("content_part1", p1),
        ("content_part2", p2),
    ]);
    generate_readme_file(&puzzle_path, &template_values)?;
    Ok(())
}

fn get_data_from_server(year: u32, day: u32, session_key: &str) -> Result<String, Error> {
    // Download data from server
    let url = format!("https://adventofcode.com/{year}/day/{day}");
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
        return Err(Error::NetworkError);
    }
    // Parse the response body
    let body = match response.text() {
        Ok(text) => text,
        Err(_) => return Err(Error::ParseError),
    };
    Ok(body)
}

fn extract_puzzle_statement(articles: &visdom::types::Elements, index: usize) -> String {
    let p_vis = articles.eq(index);
    p_vis.find("h2").remove();
    htmd::convert(&p_vis.html()).unwrap()
}

fn generate_readme_file(
    puzzle_path: &Path,
    values: &HashMap<&'static str, String>,
) -> Result<(), Error> {
    let template_content = match std::fs::read_to_string(
        path_utils::get_workspace_root()
            .join("tools/aoc-assistant/templates/new-puzzle/README.md.template"),
    ) {
        Ok(content) => content,
        Err(_) => return Err(Error::MissingTemplate),
    };
    let template = leon::Template::parse(&template_content).unwrap();
    let mut file = match std::fs::File::create(puzzle_path.join("README.md")) {
        Ok(file) => file,
        Err(_) => return Err(Error::IoError),
    };
    template
        .render_into(&mut file, &values)
        .map_err(|_| Error::IoError)?;
    Ok(())
}
