pub struct Logger {
    loggers: Vec<Box<dyn simplelog::SharedLogger>>,
}

impl Logger {
    pub fn new() -> Self {
        Self {
            loggers: vec![simplelog::TermLogger::new(
                simplelog::LevelFilter::Info,
                simplelog::Config::default(),
                simplelog::TerminalMode::Mixed,
                simplelog::ColorChoice::Auto,
            )],
        }
    }

    pub fn push(&mut self, logger: Box<dyn simplelog::SharedLogger>) {
        self.loggers.push(logger);
    }

    pub fn create_logfile(&mut self, path: &std::path::Path) -> Result<(), String> {
        use std::fs::File;

        if self.check_if_path_exist(path) {
            let result = File::create(path);

            match result {
                Ok(file) => {
                    self.push(simplelog::WriteLogger::new(
                        simplelog::LevelFilter::Trace,
                        simplelog::Config::default(),
                        file,
                    ));

                    Ok(())
                }
                Err(err) => Err(err.to_string()),
            }
        } else {
            Err(String::from("Failed to create directory!"))
        }
    }

    fn check_if_path_exist(&self, path: &std::path::Path) -> bool {
        use std::fs;
        use std::path::Path;

        let dirs: Vec<String> = path
            .to_str()
            .unwrap()
            .split("/")
            .into_iter()
            .map(|dir| String::from(dir))
            .collect();

        let dirs: Vec<String> = dirs
            .iter()
            .filter(|dir| (*dir).as_str() != ".")
            .map(|dir| dir.clone())
            .collect();

        let dir_path = {
            let tail = dirs.len() - 1;

            format!("{}", dirs[..tail].join("/"))
        };

        match fs::create_dir_all(Path::new(dir_path.as_str())) {
            Ok(()) => true,
            Err(_) => false,
        }
    }

    pub fn init(self) -> Result<(), String> {
        match simplelog::CombinedLogger::init(self.loggers) {
            Ok(()) => Ok(()),
            Err(err) => Err(err.to_string()),
        }
    }
}
