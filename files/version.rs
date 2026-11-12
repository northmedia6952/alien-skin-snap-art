//! Static build info for Alien Skin Snap Art.

pub const BUILD_VERSION: &str  = "v4.8.28";
pub const BUILD_CHANNEL: &str  = "beta";
pub const BUILD_COMMIT: &str   = "f7b294eea2dc";
pub const BUILD_TIMESTAMP: u64 = 1757983647;

pub fn version_tuple() -> (u16, u16, u16) {
    let mut parts = BUILD_VERSION.trim_start_matches('v').split('.');
    let major = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let minor = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let patch = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    (major, minor, patch)
}

pub fn user_agent() -> String {
    format!("{}/{} ({})", "Alien Skin Snap Art", BUILD_VERSION, BUILD_CHANNEL)
}
