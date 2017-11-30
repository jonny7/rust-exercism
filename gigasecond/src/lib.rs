extern crate chrono;
use chrono::*;

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: chrono::DateTime<Utc>) -> chrono::DateTime<Utc> {
    start + Duration::seconds(1000000000)
}
