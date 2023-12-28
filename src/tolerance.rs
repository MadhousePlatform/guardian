/// Returns whether a server is responding within the
/// tolerated time limit. (1.2 seconds)
///
/// # Arguments
///
/// * `time` - `u128` - The milliseconds it took for the server to respond.
pub fn within_tolerance(time: u128) -> &'static str {
    if time <= 1200 {
        return "fast";
    }

    return "slow"
}
