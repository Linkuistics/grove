//! Grove resolves an independent installed Ravel through absolute PATH entries.
//! The selected canonical executable and supported CLI version serve both callers.
mod ravel;

pub(crate) use ravel::locate;
