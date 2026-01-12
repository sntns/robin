use crate::error::RobinError;
use crate::sysfs::{read_mesh_bool, write_mesh_bool};

pub async fn get_aggregation(mesh_if: &str) -> Result<bool, RobinError> {
    read_mesh_bool(mesh_if, "aggregation")
}

pub async fn set_aggregation(mesh_if: &str, val: bool) -> Result<(), RobinError> {
    write_mesh_bool(mesh_if, "aggregation", val)
}
