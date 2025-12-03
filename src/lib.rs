use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

pub struct EnvFile {
    pub path: String,
    pub lines: HashMap<String, String>,
}

#[allow(dead_code)]
pub fn load(path: &str) -> EnvFile {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let lines = reader
        .lines()
        .map(|line| line.unwrap())
        .filter(|line| !line.starts_with('#'))
        .map(|line| {
            let mut parts = line.splitn(2, '=');
            let key = parts.next().unwrap().trim().to_string();
            let value = parts.next().unwrap().trim().to_string();
            (key, value)
        })
        .collect();

    EnvFile {
        path: path.to_string(),
        lines,
    }
}

impl EnvFile {
    pub fn get(&self, key: &str) -> Option<String> {
        self.lines.get(key).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{env, io::Write};

    #[test]
    fn test_load() {
        let path = env::current_dir().unwrap().join("test.env");
        let _ = std::fs::remove_file(&path);
        let mut file = File::create(&path).unwrap();
        file.write_all(
            b"TEST=123
TEST2=456",
        )
        .unwrap();

        let env_file = load(path.to_str().unwrap());
        assert_eq!(env_file.get("TEST"), Some("123".to_string()));
        assert_eq!(env_file.get("TEST2"), Some("456".to_string()));
    }
}
