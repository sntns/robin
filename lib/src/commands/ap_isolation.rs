use crate::error::RobinError;
use crate::sysfs::{read_mesh_bool, write_mesh_bool};

pub async fn get_ap_isolation(mesh_if: &str) -> Result<bool, RobinError> {
    read_mesh_bool(mesh_if, "ap_isolation")
}

pub async fn set_ap_isolation(mesh_if: &str, val: bool) -> Result<(), RobinError> {
    write_mesh_bool(mesh_if, "ap_isolation", val)
}
