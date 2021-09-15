use glob::{glob, Paths, PatternError};
use std::num::ParseIntError;

pub fn get_files(path: &str) -> Result<Paths, PatternError> {
    glob(&format!("{}/*.sql", path))
}

fn last_migration_prefix(paths: Paths) -> Option<String> {
    let mut p: Vec<String> = paths
        .filter(|p| p.is_ok())
        .map(|p| {
            p.unwrap()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string()
        })
        .collect::<Vec<String>>();
    p.sort();

    match p.last().cloned() {
        Some(last) => last
            .split("-")
            .collect::<Vec<&str>>()
            .iter()
            .map(|l| l.to_string())
            .collect::<Vec<String>>()
            .first()
            .cloned(),
        None => None,
    }
}

fn next_migration_prefix(last_prefix: String) -> Result<String, ParseIntError> {
    let last_prefix_int = last_prefix.parse::<i32>()?;
    Ok(format!("{:0>3}", last_prefix_int + 1).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_last_migration_prefix() {
        let paths = glob("test/sql_files/*.sql").expect("could not get test sql files");
        let last_migration_prefix = last_migration_prefix(paths);
        assert!(last_migration_prefix.unwrap() == "01");
    }

    #[test]
    fn test_next_migration_prefix() {
        let paths = glob("test/sql_files/*.sql").expect("could not get test sql files");
        let last_migration_prefix = last_migration_prefix(paths).unwrap();
        let next_migration_prefix = next_migration_prefix(last_migration_prefix).unwrap();

        println!("{:?}", next_migration_prefix);
        assert!(next_migration_prefix == "002");
    }
}
