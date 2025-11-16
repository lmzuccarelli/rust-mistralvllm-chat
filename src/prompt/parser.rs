use custom_logger as log;
use walkdir::WalkDir;

#[allow(unused)]
pub struct PromptParser {}

impl PromptParser {
    pub fn parse(input: String) -> Result<String, Box<dyn std::error::Error + Sync + Send>> {
        let result: String;
        match input.clone() {
            x if x.contains(&"read") => {
                result = build_from_input(input);
            }
            _ => {
                log::warn!("unknow input command");
                result = "unkown command".to_string();
            }
        }
        Ok(result)
    }
}

fn build_from_input(input: String) -> String {
    if input.contains("open") && !input.contains("close")
        || !input.contains("open") && input.contains("close")
    {
        log::warn!("[parse] please ensure you have 'open' and 'close' key words");
        "none".to_string()
    } else {
        let start = input.split("open").nth(1);
        let result = match start {
            Some(value) => {
                let end = value.split("close").nth(0);
                match end {
                    Some(end_value) => {
                        log::trace!("[parse] sub_string {}", end_value);
                        let vec_file: Vec<&str> = end_value.split(" ").collect();
                        let mut file_result = String::new();
                        let mut count = 0;
                        for entry in WalkDir::new(format!("{}/{}", vec_file[1], vec_file[2]))
                            .into_iter()
                            .filter_map(|e| e.ok())
                        {
                            if entry.path().is_file() {
                                let filename = entry.path().file_name().unwrap().to_string_lossy();
                                if filename.contains(vec_file[2]) {
                                    println!();
                                    file_result = entry.path().to_string_lossy().to_string();
                                    log::info!("[parse] found {}", file_result);
                                    count += 1;
                                }
                            }
                        }
                        if count == 0 || count > 1 {
                            log::warn!(
                                "[parse] be more specific with dir and search filename i.e use -> 'open dir/subdir fine-grained-name close'"
                            );
                            "none".to_string()
                        } else {
                            file_result
                        }
                    }
                    None => {
                        log::warn!("[parse] please ensure you have 'open' and 'close' key words");
                        "none".to_string()
                    }
                }
            }
            None => input,
        };
        result
    }
}
