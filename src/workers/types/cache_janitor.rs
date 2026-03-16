use crate::exceptions::SCloudException;
use crate::workers::SCloudWorker;
use std::sync::Arc;

pub async fn run_dns_cache_janitor(
    worker: Arc<SCloudWorker>,
) -> Result<(), SCloudException> {
    Ok(())
}
