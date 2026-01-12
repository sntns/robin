use crate::error::RobinError;
use std::fs;

fn mesh_sysfs_path(mesh_if: &str, name: &str) -> String {
    format!("/sys/class/net/{}/mesh/{}", mesh_if, name)
}

pub fn read_mesh_bool(mesh_if: &str, name: &str) -> Result<bool, RobinError> {
    let path = mesh_sysfs_path(mesh_if, name);
    let val =
        fs::read_to_string(&path).map_err(|e| RobinError::Io(format!("{}: {:?}", path, e)))?;

    Ok(val.trim() == "1")
}

pub fn write_mesh_bool(mesh_if: &str, name: &str, value: bool) -> Result<(), RobinError> {
    let path = mesh_sysfs_path(mesh_if, name);
    fs::write(&path, if value { "1" } else { "0" })
        .map_err(|e| RobinError::Io(format!("{}: {:?}", path, e)))?;
    Ok(())
}
