use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Resource, Debug)]
pub struct CameraSettings {}
