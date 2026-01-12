use crate::error::RobinError;
use crate::sysfs::{read_mesh_bool, write_mesh_bool};

pub async fn get_bridge_loop_avoidance(mesh_if: &str) -> Result<bool, RobinError> {
    read_mesh_bool(mesh_if, "bridge_loop_avoidance")
}

pub async fn set_bridge_loop_avoidance(mesh_if: &str, val: bool) -> Result<(), RobinError> {
    write_mesh_bool(mesh_if, "bridge_loop_avoidance", val)
}
