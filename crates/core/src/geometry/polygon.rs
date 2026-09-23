use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::component::lla::Lla;
type PolygonId = Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Polygon {
    pub id: PolygonId,
    pub vertices: Vec<Lla>,
}
