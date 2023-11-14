use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

const MOST_COMMON_200_PATH: &str = "assets/words/most_common_200.txt";

pub fn load_most_common_200(handle: tauri::AppHandle) -> io::Result<[String; 200]> {
    match handle
        .path_resolver()
        .resolve_resource(MOST_COMMON_200_PATH)
    {
        Some(resource_path) => {
            let file = File::open(&resource_path)?;
            let reader = BufReader::new(file);
            let mut words = Vec::with_capacity(200);
            for word in reader.lines() {
                let word = word?;
                let word = word.trim().to_string();
                words.push(word);
            }
            if words.len() != 200 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    ["File at ", MOST_COMMON_200_PATH, "doesn't have 200 words"].concat(),
                ));
            }

            Ok(words.try_into().unwrap())
        }
        None => Err(io::Error::new(
            io::ErrorKind::NotFound,
            ["Can't resolve ", MOST_COMMON_200_PATH].concat(),
        )),
    }
}
