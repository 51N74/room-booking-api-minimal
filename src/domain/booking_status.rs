use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum BookingStatusUpdate {
    Pending,
    Confirmed,
    Cancelled,
    Completed,
}


impl fmt::Display for BookingStatusUpdate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BookingStatusUpdate::Pending => write!(f, "Pending"),
            BookingStatusUpdate::Confirmed => write!(f, "Confirmed"),
            BookingStatusUpdate::Cancelled => write!(f, "Cancelled"),
            BookingStatusUpdate::Completed => write!(f, "Completed"),
        }
    }
}

