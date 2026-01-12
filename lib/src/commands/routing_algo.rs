use crate::error::RobinError;
use crate::sysfs::{read_routing_algo, write_routing_algo};

pub async fn get_routing_algo() -> Result<String, RobinError> {
    read_routing_algo().await
}

pub async fn set_routing_algo(val: &str) -> Result<(), RobinError> {
    write_routing_algo(val).await
}
