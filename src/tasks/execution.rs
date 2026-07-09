use crate::forgefile::Task;

use log::{debug, error, info};

pub fn execute_task(forgefile_task: Task) -> Result<String, String> {
    debug!("{:}", forgefile_task);

    Ok(format!("Successful!"))
}
