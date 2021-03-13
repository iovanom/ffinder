pub(crate) struct PackageInfo {
    pub(crate) description: &'static str,
    pub(crate) version: &'static str,
    pub(crate) author: &'static str,
}

impl PackageInfo {
    pub(crate) fn new() -> Self {
        let description = option_env!("CARGO_PKG_DESCRIPTION").unwrap_or("unknown");
        let version = option_env!("CARGO_PKG_VERSION").unwrap_or("unknown");
        let author = option_env!("CARGO_PKG_AUTHORS").unwrap_or("unknown");
        Self {
            description,
            version,
            author,
        }
    }
}
