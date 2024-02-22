use std::collections::HashMap;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use crate::config::Config;

#[derive(Debug, Serialize, Deserialize)]
struct Data {
    pub projector: HashMap<PathBuf, HashMap<String, String>>,
}

pub struct Projector {
    config: Config,
    data: Data,
}

fn default_data() -> Data {
    return Data {
        projector: HashMap::new(),
    };
}

impl Projector {
    pub fn get_all_value(&self) -> HashMap<&String, &String> {
        let mut curr = Some(self.config.pwd.as_path());
        let mut paths = vec![];
        while let Some(p) = curr {
            paths.push(p);
            curr = p.parent();
        }

        let mut out = HashMap::new();
        for path in paths.into_iter().rev() {
            if let Some(map) = self.data.projector.get(path) {
                out.extend(map.iter());
            }
        }

        return out;
    }

    pub fn get_value(&self, key: &str) -> Option<&String> {
        let mut curr = Some(self.config.pwd.as_path());
        let mut out = None;
        while let Some(p) = curr {
            if let Some(dir) = self.data.projector.get(p) {
                if let Some(value) = dir.get(key) {
                    out = Some(value);
                    break;
                }
            }
            curr = p.parent()
        }

        return out;
    }

    pub fn set_value(&mut self, key: String, value: String) {
        self.data.projector
            .entry(self.config.pwd.clone())
            .or_default()
            .insert(key, value);
    }

    pub fn remove_value(&mut self, key: &str) {
        self.data.projector
            .get_mut(&self.config.pwd)
            .map(|x| {
                x.remove(key);
            });
    }

    pub fn from_config(config: Config) -> Self {
        if std::fs::metadata(&config.config).is_ok() {
            let contents = std::fs::read_to_string(&config.config);
            let contents = contents.unwrap_or(
                String::from("{\"projector\":{}")
            );
            let data = serde_json::from_str(&contents);
            let data = data.unwrap_or(default_data());

            return Projector {
                config,
                data,
            };
        }

        return Projector {
            config,
            data: default_data(),
        };
    }
}


#[cfg(test)]
mod test {
    use std::path::PathBuf;
    use collection_macros::hashmap;
    use crate::config::Config;
    use crate::config::Operation::Print;
    use crate::projector::{Data, Projector};

    fn get_data() -> Data {
        return Data {
            projector: hashmap! {
                PathBuf::from("/") => hashmap! {
                    String::from("foo") => String::from("bar1"),
                    String::from("fem") => String::from("is_great"),
                },
                PathBuf::from("/foo") => hashmap! {
                    String::from("foo") => String::from("bar2")
                },
                PathBuf::from("/foo/bar") => hashmap! {
                    String::from("foo") => String::from("bar3")
                }
            }
        };
    }

    fn get_projector(pwd: PathBuf, data: Data) -> Projector {
        return Projector {
            config: Config {
                pwd,
                config: PathBuf::from(""),
                operation: Print(None),
            },
            data,
        };
    }

    #[test]
    fn get_value() {
        let data = get_data();
        let proj = get_projector(PathBuf::from("/foo/bar"), data);

        assert_eq!(proj.get_value("foo"), Some(&String::from("bar3")));
        assert_eq!(proj.get_value("fem"), Some(&String::from("is_great")));
    }

    #[test]
    fn set_value() {
        let data = get_data();
        let mut proj = get_projector(PathBuf::from("/foo/bar"), data);

        proj.set_value(String::from("foo"), String::from("baz"));
        proj.set_value(String::from("fem"), String::from("is_super_great"));
        assert_eq!(proj.get_value("foo"), Some(&String::from("baz")));
        assert_eq!(proj.get_value("fem"), Some(&String::from("is_super_great")));
    }

    #[test]
    fn remove_value() {
        let data = get_data();
        let mut proj = get_projector(PathBuf::from("/foo/bar"), data);

        proj.remove_value("foo");
        proj.remove_value("fem");
        assert_eq!(proj.get_value("foo"), Some(&String::from("bar2")));
        assert_eq!(proj.get_value("fem"), Some(&String::from("is_great")));
    }
}