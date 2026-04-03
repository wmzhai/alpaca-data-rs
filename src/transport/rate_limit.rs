use std::sync::Arc;

use tokio::sync::{OwnedSemaphorePermit, Semaphore};

use crate::Error;

#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub(crate) struct RateLimiter {
    semaphore: Option<Arc<Semaphore>>,
}

#[allow(dead_code)]
impl RateLimiter {
    pub(crate) fn new(max_in_flight: Option<usize>) -> Self {
        Self {
            semaphore: max_in_flight.map(|value| Arc::new(Semaphore::new(value))),
        }
    }

    pub(crate) async fn acquire(&self) -> Result<Option<OwnedSemaphorePermit>, Error> {
        match &self.semaphore {
            Some(semaphore) => semaphore
                .clone()
                .acquire_owned()
                .await
                .map(Some)
                .map_err(|error| Error::Transport(error.to_string())),
            None => Ok(None),
        }
    }
}
