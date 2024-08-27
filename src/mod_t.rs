#![allow(unused)]
// spec document: https://fabricmc.net/wiki/documentation:fabric_mod_json_spec

use serde_json::Value;
use std::{
    collections::HashMap,
    fmt,
    fs::ReadDir,
    io::{Read, Seek},
    path::PathBuf,
};
use zip::ZipArchive;

#[derive(Debug)]
pub struct FabricMod {
    pub filename: String,
    pub id: String,
    pub version: String,
    pub depends: HashMap<String, String>,
    pub recommends: HashMap<String, String>,
    pub suggests: HashMap<String, String>,
    pub conflicts: HashMap<String, String>,
    pub breaks: HashMap<String, String>,
}

pub struct FabricModLite {
    pub filename: String,
    pub id: String,
}

impl fmt::Debug for FabricModLite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl FabricMod {
    pub fn from_json(filename: String, json: Value) -> Self {
        let id = json["id"].as_str().unwrap().to_string();
        let version = json["version"].as_str().unwrap().to_string();
        let depends = match json.get("depends") {
            Some(depends) => depends
                .as_object()
                .unwrap()
                .iter()
                .map(|(k, v)| (k.to_string(), v.as_str().unwrap_or_default().to_string()))
                .collect(),
            None => HashMap::new(),
        };
        let recommends = match json.get("recommends") {
            Some(recommends) => recommends
                .as_object()
                .unwrap()
                .iter()
                .map(|(k, v)| (k.to_string(), v.as_str().unwrap_or_default().to_string()))
                .collect(),
            None => HashMap::new(),
        };
        let suggests = match json.get("suggests") {
            Some(suggests) => suggests
                .as_object()
                .unwrap()
                .iter()
                .map(|(k, v)| (k.to_string(), v.as_str().unwrap_or_default().to_string()))
                .collect(),
            None => HashMap::new(),
        };
        let conflicts = match json.get("conflicts") {
            Some(conflicts) => conflicts
                .as_object()
                .unwrap()
                .iter()
                .map(|(k, v)| (k.to_string(), v.as_str().unwrap_or_default().to_string()))
                .collect(),
            None => HashMap::new(),
        };
        let breaks = match json.get("breaks") {
            Some(breaks) => breaks
                .as_object()
                .unwrap()
                .iter()
                .map(|(k, v)| (k.to_string(), v.as_str().unwrap_or_default().to_string()))
                .collect(),
            None => HashMap::new(),
        };
        Self {
            filename,
            id,
            version,
            depends,
            recommends,
            suggests,
            conflicts,
            breaks,
        }
    }
    pub fn from_jar(filename: String, jar: impl Read + Seek) -> Self {
        let mut zip = ZipArchive::new(jar).unwrap();
        let mut metadata_reader = zip.by_name("fabric.mod.json").unwrap();
        let mut metadata: String = String::new();
        metadata_reader.read_to_string(&mut metadata).unwrap();
        let json = serde_json::from_str(&metadata.replace("\n", "")).unwrap();
        Self::from_json(filename, json)
    }
    pub fn from_jars(jars_list: Vec<PathBuf>) -> Vec<Self> {
        let mut mods = vec![];
        for path in jars_list {
            let file = std::fs::File::open(&path).unwrap();
            mods.push(Self::from_jar(
                path.file_name().unwrap().to_str().unwrap().to_string(),
                file,
            ));
        }
        mods
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn test_fabric_mod_from_json() {
        let base_dir: PathBuf = [env!("CARGO_MANIFEST_DIR"), "resources", "test"]
            .iter()
            .collect();
        let test_filepath = base_dir.join("fabric.mod.json");
        let json: Value = serde_json::from_reader(fs::File::open(test_filepath).unwrap()).unwrap();
        let fabric_mod = FabricMod::from_json("unknown.jar".to_string(), json);
        assert_eq!(fabric_mod.id, "appleskin");
        assert_eq!(fabric_mod.version, "2.5.1+mc1.20");
        assert_eq!(fabric_mod.depends.len(), 2);
        assert_eq!(fabric_mod.recommends.len(), 0);
        assert_eq!(fabric_mod.suggests.len(), 0);
        assert_eq!(fabric_mod.conflicts.len(), 0);
        assert_eq!(fabric_mod.breaks.len(), 1);
    }

    #[test]
    fn test_fabric_mod_from_jar() {
        let base_dir: PathBuf = [env!("CARGO_MANIFEST_DIR"), "resources", "test"]
            .iter()
            .collect();
        let test_filepath = base_dir.join("alloy-forgery-2.1.2+1.20.jar");
        let fabric_mod = FabricMod::from_jar(
            "alloy-forgery-2.1.2+1.20.jar".to_string(),
            fs::File::open(test_filepath).unwrap(),
        );
        assert_eq!(fabric_mod.filename, "alloy-forgery-2.1.2+1.20.jar");
        assert_eq!(fabric_mod.id, "alloy_forgery");
        assert_eq!(fabric_mod.version, "2.1.2+1.20");
        assert_eq!(fabric_mod.depends.len(), 4);
        assert_eq!(fabric_mod.recommends.len(), 1);
        assert_eq!(fabric_mod.suggests.len(), 2);
        assert_eq!(fabric_mod.conflicts.len(), 0);
        assert_eq!(fabric_mod.breaks.len(), 0);
    }
}
