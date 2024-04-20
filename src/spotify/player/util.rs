use chrono::TimeDelta;
use std::time::Duration;

/// Converts a floating-point timestamp to a `chrono::TimeDelta`
pub fn f64_to_duration(timestamp: f64) -> Result<TimeDelta, chrono::OutOfRangeError> {
    let timestamp = f64::to_bits(timestamp);
    let duration = Duration::from_millis(timestamp);

    // Attempt to convert the `Duration` to a `chrono::TimeDelta`.
    // If conversion fails due to being out of range, return zero time delta.
    let time_delta = match TimeDelta::from_std(duration) {
        Ok(result) => result,
        Err(_e) => TimeDelta::zero(),
    };

    Ok(time_delta)
}
