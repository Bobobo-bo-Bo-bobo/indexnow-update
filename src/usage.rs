use crate::constants;

pub fn show_usage() {
    show_version();
    println!("Usage: {} [-D|--debug] -c <config>|--config=<config> [-h|--help] [-n|--dry-run] [-p|--purge] [-v|--version] [-q|--quiet] /path/to/html

    -D          Enable debug output
    --debug

    -c <config>         Path to configuration file
    --config=<config>   This option is mandatory

    -h                  This text
    --help

    -n                  Process data but print what whould be submitted instead
    --dry-run           submitting it

    -p                  Purge files found in the database but no
    --purge             longer present in the filesystem

    -v                  Show version information
    --version

    -q                  Quiet operation
    --quiet             Only errors will be logged
", constants::NAME)
}

pub fn show_version() {
    println!(
        "{} version {}
Copyright (C) 2021-2022 by Andreas Maus <maus@ypbind.de>
This program comes with ABSOLUTELY NO WARRANTY.

{} is distributed under the Terms of the GNU General
Public License Version 3. (http://www.gnu.org/copyleft/gpl.html)
",
        constants::NAME,
        constants::VERSION,
        constants::NAME
    );
}
