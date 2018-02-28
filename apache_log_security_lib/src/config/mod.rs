use input;
use reporting;
use glob::glob;


#[derive(Deserialize, Serialize)]
pub enum Service {
    Apache(input::apache::Apache),
}

#[derive(Deserialize, Serialize)]
pub enum Reporting {
    Std(reporting::std::Std),
    Email(reporting::email::Email),
}

#[derive(Deserialize, PartialEq, Serialize)]
pub enum XssLevel {
    Basic,
    Intelligent,
}

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub reporting: Vec<Reporting>,
    pub services: Vec<Service>,
    pub xss_level: XssLevel,
}

impl Config {
    pub fn new() -> Config {
        Config {
            reporting: vec![],
            services: vec![],
            xss_level: XssLevel::Basic,
        }
    }

    /// Is used to convert glob paths to concrete paths.
    ///
    /// If we have directory `dir` with files `a.log` and `b.log` this will turn `dir/*`
    /// to concrete paths `dir/a.log` and `dir/b.log` and return new config while consuming the old one.
    pub fn normalize_glob_path(mut self) -> Config {
        self.services = self.services
            .into_iter()
            .flat_map(|Service::Apache(apache)| {
                let services: Vec<_> = glob(&apache.path[..])
                    .expect("Failed to read input file path glob pattern")
                    .filter_map(|entry| match entry {
                        Ok(path_buf) => Some(Service::Apache(input::apache::Apache {
                            path: path_buf.to_str().unwrap().to_string(),
                        })),
                        Err(e) => {
                            eprintln!("{:?}", e);
                            None
                        }
                    })
                    .collect();
                services
            })
            .collect();

        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use config::Service;

    #[test]
    fn normalize_glob_path_00() {
        let mut config = Config::new();
        config.services = vec![
            Service::Apache(input::apache::Apache {
                path: "/*".to_string(),
            }),
        ];
        assert!(config.normalize_glob_path().services.len() > 1);
    }
}