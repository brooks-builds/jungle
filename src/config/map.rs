use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub enum MapFeature {
    Pit1,
    Pit3,
    Rope,
}
