use crate::forgefile::{Config, Task};

use log::{debug, error, info};

pub fn execute_task(config: &Config, forgefile_task: Task) -> Result<String, String> {
    debug!("{:}", forgefile_task);

    Ok(format!("Successful!"))
}
