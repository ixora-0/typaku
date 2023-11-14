use std::{
    fs::{self, File, OpenOptions},
    io::{self, Read, Write},
    path::PathBuf,
};

use log::{error, info, trace};

use crate::data_structs::AppData;

const APPDATA_FOLDER_NAME: &str = "AdversaType";
const DATA_FILE_PATH: &str = "data.toml"; // would be joined with user's data directory

pub fn get_app_data_dir() -> Option<PathBuf> {
    if let Some(user_data_dir) = tauri::api::path::data_dir() {
        let app_data_dir = user_data_dir.join(APPDATA_FOLDER_NAME);
        return Some(app_data_dir);
    }
    None
}

pub fn load_data() -> io::Result<AppData> {
    fn err_and_log(err: io::Error) -> io::Result<AppData> {
        error!("Error while loading data: {}", err);
        Err(err)
    }
    let app_data_dir = match get_app_data_dir() {
        Some(app_data_dir) => app_data_dir,
        None => {
            return err_and_log(io::Error::new(
                io::ErrorKind::NotFound,
                "User's data directory not found. Using default data",
            ))
        }
    };
    let app_data_dir_repr = app_data_dir.as_os_str().to_string_lossy();

    let data_file_path = app_data_dir.join(DATA_FILE_PATH);
    let data_file_path_repr = data_file_path.as_os_str().to_string_lossy();

    let write_default_data = |mut file: File| {
        trace!("Creating new default data file at {}", data_file_path_repr);
        let toml_str = toml::to_string(&AppData::default())
            .map_err(|toml_err| io::Error::new(io::ErrorKind::InvalidData, toml_err))?;
        file.write_all(toml_str.as_bytes())?;
        info!("Created new default data file at {}", data_file_path_repr);
        Ok(())
    };
    let read_data = || {
        trace!("Reading existing data file: {}", data_file_path_repr);
        let mut file = File::open(&data_file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let read_app_data: AppData = toml::from_str(&contents)
            .map_err(|toml_err| io::Error::new(io::ErrorKind::InvalidData, toml_err))?;
        info!("Done reading existing data file: {}", data_file_path_repr);
        Ok(read_app_data)
    };

    // try to create directory
    match fs::create_dir(&app_data_dir) {
        Ok(()) => info!("Created directory {}", app_data_dir_repr),
        Err(err) => match err.kind() {
            io::ErrorKind::AlreadyExists => {
                trace!("Found user's data directory {}", app_data_dir_repr)
            }
            _ => return err_and_log(err),
        },
    }

    // try to create new file
    match OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&data_file_path)
    {
        Ok(file) => match write_default_data(file) {
            Ok(()) => Ok(AppData::default()),
            Err(err) => err_and_log(err),
        },
        Err(err) => match err.kind() {
            io::ErrorKind::AlreadyExists => match read_data() {
                Ok(read_app_data) => Ok(read_app_data),
                Err(err) => err_and_log(err),
            },
            _ => err_and_log(err),
        },
    }
}
