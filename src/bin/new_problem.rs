use std::io::Write;
use std::fs::OpenOptions;
use std::{fs, io};
use std::path::Path;

const TEMPLATE: &str = r#"
// https://leetcode.com/problems/name_of_problem/
// Time: O(?) Space: O(?)

pub struct Solution;

impl Solution {
    // paste the signature from LeetCode here
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(1, 1);
    }
}
"#;

fn normalize_name(slug: &str) -> String {
    let name = slug.trim()
        .to_lowercase()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join("-")
        .replace('_', "-");
    name
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

fn register_module(dir: &str, module: &str) -> io::Result<()> {
    let mod_path = Path::new(dir).join("mod.rs");
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(mod_path)?;
    writeln!(file, "pub mod {module};")
}

fn make_problem(num: u32, slug: &str, difficulty: &str) -> io::Result<()> {
    validate_difficulty(difficulty)?;

    let dir = format!("src/problems/{difficulty}");
    let name = normalize_name(slug);
    let module = format!("p{num:04}_{}", name.replace('-', "_"));

    let path = create_problem_file(&dir, &module, &name)?;
    register_module(&dir, &module)?;

    println!("created {path}");
    Ok(())
}


fn main() -> io::Result<(), std::io::Error> {
    make_problem(1, "NUMber of_ELEMENTS", "hard")
}
