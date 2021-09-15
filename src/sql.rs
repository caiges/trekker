use glob::{glob, Paths, PatternError};

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_last_migration_prefix() {
        let paths = glob("test/sql_files/*.sql").expect("could not get test sql files");
        let last_migration_prefix = last_migration_prefix(paths);
        println!("{:?}", last_migration_prefix);
        assert!(last_migration_prefix.unwrap() == "01");
    }
}
