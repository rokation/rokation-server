use serde::{Deserialize, Serialize};

use crate::component::lla::Lla;

#[derive(Debug, Serialize, Deserialize)]
pub struct Line {
    pub vertices: Vec<Lla>,
}
