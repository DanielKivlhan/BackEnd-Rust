use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PhotodiodeData {
    pub intensity: u16,
}
