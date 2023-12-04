use serde::{Deserialize, Serialize};
use serde_json::Error;

use crate::state::State;

#[derive(Serialize, Deserialize)]
struct TrawFile {
    version: FileVersion,
    data: String,
}

const CURRENT_VERSION: FileVersion = FileVersion::V1;

impl TrawFile {
    pub fn new(data: String) -> Self {
        TrawFile {
            version: CURRENT_VERSION,
            data,
        }
    }
}

#[derive(Serialize, Deserialize)]
enum FileVersion {
    V1,
}

pub fn save(state: &State) -> Result<(), Error> {
    let data = serde_json::to_string(state)?;
    let traw_file = TrawFile::new(data);
    let _ = std::fs::write("unnamed.traw", serde_json::to_string(&traw_file)?);
    Ok(())
}

pub fn load(path: String) -> std::io::Result<State> {
    let traw_file: TrawFile = serde_json::from_slice(std::fs::read(path)?.as_slice())?;
    match traw_file.version {
        FileVersion::V1 => Ok(serde_json::from_str(&traw_file.data)?),
    }
}
