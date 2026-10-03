use std::io;
use std::process::ExitCode;

use rs_filetyp2int_flbuf::FileType;

fn sub() -> Result<(), io::Error> {
    FileType::write_map2stdout_default()
}

fn main() -> ExitCode {
    sub().map(|_| ExitCode::SUCCESS).unwrap_or_else(|e| {
        eprintln!("{e}");
        ExitCode::FAILURE
    })
}
