use crate::info;
use clap::{App, Arg};
use std::path::PathBuf;

pub(crate) struct Settings {
    pub(crate) start_dir: PathBuf,
}

impl<'a> Settings {
    pub(crate) fn new() -> Self {
        let package_info = info::PackageInfo::new();
        let matches = App::new(package_info.description)
            .version(package_info.version)
            .author(package_info.author)
            .arg(
                Arg::with_name("start-dir")
                    .short("d")
                    .long("start-dir")
                    .value_name("DIR")
                    .help("The directory where the search is starting. Default is CWD")
                    .takes_value(true),
            )
            .get_matches();

        let start_dir = PathBuf::from(matches.value_of("start-dir").unwrap_or("."));
        if !start_dir.is_dir() {
            panic!("The provided argument is not a directory");
        }
        Self { start_dir }
    }
}
