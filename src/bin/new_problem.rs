use std::env;
use std::path::Path;
use std::{fs, io};

const TEMPLATE: &str = r#"// https://leetcode.com/problems/name_of_problem/
// Time: O(?) Space: O(?)

pub struct Solution;

impl Solution {
    // paste the signature from LeetCode here
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(1, 1);
    }
}
"#;

fn normalize_name(slug: &str) -> String {
    slug.trim()
        .to_lowercase()
        .replace('_', " ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join("-")
}

fn validate_difficulty(difficulty: &str) -> io::Result<()> {
    match difficulty {
        "easy" | "medium" | "hard" => Ok(()),
        _ => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("unknown difficulty: {difficulty} (allowed: easy, medium, hard)"),
        )),
    }
}

fn create_problem_file(dir: &str, module: &str, name: &str) -> io::Result<String> {
    let path = format!("{dir}/{module}.rs");

    if Path::new(&path).exists() {
        return Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            format!("{path} already exists"),
        ));
    }

    fs::create_dir_all(dir)?;
    fs::write(&path, TEMPLATE.replace("name_of_problem", name))?;
    Ok(path)
}

fn register_and_sort_module(dir: &str, module: &str) -> io::Result<()> {
    let mod_path = Path::new(dir).join("mod.rs");

    let existing = match fs::read_to_string(&mod_path) {
        Ok(text) => text,
        Err(e) if e.kind() == io::ErrorKind::NotFound => String::new(),
        Err(e) => return Err(e),
    };

    let mut content: Vec<String> = existing
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(String::from)
        .collect();

    content.push(format!("pub mod {module};"));
    content.sort();
    content.dedup();

    fs::write(&mod_path, content.join("\n") + "\n")
}

fn make_problem(num: u32, slug: &str, difficulty: &str) -> io::Result<()> {
    validate_difficulty(difficulty)?;

    let dir = format!("src/problems/{difficulty}");
    let name = normalize_name(slug);
    let module = format!("p{num:04}_{}", name.replace('-', "_"));

    let path = create_problem_file(&dir, &module, &name)?;
    register_and_sort_module(&dir, &module)?;

    println!("created {path}");
    Ok(())
}

fn run() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "usage: cargo np <number> <slug> <easy|medium|hard>",
        ));
    }

    let num: u32 = args[1].parse().map_err(|_| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("'{}' is not a valid problem number", args[1]),
        )
    })?;

    make_problem(num, &args[2], &args[3])
}

fn main() {
    if let Err(e) = run() {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}
