use dirs::home_dir;
use std::{fs::read_to_string, path::PathBuf};

fn get_config_path() -> PathBuf {
    match home_dir() {
        Some(home_path) => {
            let mut path = PathBuf::new();
            path.push(home_path);
            path.push(".airs");
            return path;
        }
        None => panic!("Home directory not found"),
    };
}

pub fn read_config() -> String {
    let mut path = get_config_path();
    path.push("config.json");

    if path.exists() {
        return read_to_string(path).unwrap();
    }

    return "{}".to_string();
}

pub fn write_config(config: String) {
    let mut path = get_config_path();
    path.push("config.json");

    std::fs::write(path, config).unwrap();
}

pub fn read_profile_file(name: &String) -> String {
    let mut path = get_config_path();
    path.push(name);
    path.set_extension("json");

    if path.exists() {
        return read_to_string(path).unwrap();
    }

    panic!("Profile not found");
}

pub fn list_profiles() -> Vec<String> {
    let path = get_config_path();

    if path.exists() {
        let mut ret = vec![];
        for entry in path.read_dir().unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            let name = path.file_stem().unwrap().to_str().unwrap().to_string();
            if name == "config" {
                continue;
            }
            ret.push(name);
        }
        ret.sort();
        return ret;
    }
    vec![]
}
