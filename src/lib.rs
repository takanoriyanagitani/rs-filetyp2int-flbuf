use std::io;

use io::Write;

use flexbuffers::Builder;

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum FileType {
    /// Unspecified or unknown file type.
    Unk = 0,

    /// Regular file.
    Reg = 1,

    /// Symbolic link.
    Sym = 2,

    /// Character device.
    Chr = 3,

    /// Block device.
    Blk = 4,

    /// Directory.
    Dir = 5,

    /// Named pipe (FIFO).
    Pip = 6,

    /// Socket.
    Sck = 7,
}

impl FileType {
    pub fn as_str(&self) -> &'static str {
        match self {
            FileType::Unk => "UNK",
            FileType::Reg => "REG",
            FileType::Sym => "SYM",
            FileType::Chr => "CHR",
            FileType::Blk => "BLK",
            FileType::Dir => "DIR",
            FileType::Pip => "PIP",
            FileType::Sck => "SCK",
        }
    }
}

impl From<u8> for FileType {
    fn from(u: u8) -> Self {
        let masked: u8 = u & 0x07;
        match masked {
            1 => Self::Reg,
            2 => Self::Sym,
            3 => Self::Chr,
            4 => Self::Blk,
            5 => Self::Dir,
            6 => Self::Pip,
            7 => Self::Sck,
            _ => Self::Unk,
        }
    }
}

impl std::str::FromStr for FileType {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "REG" => Ok(FileType::Reg),
            "SYM" => Ok(FileType::Sym),
            "CHR" => Ok(FileType::Chr),
            "BLK" => Ok(FileType::Blk),
            "DIR" => Ok(FileType::Dir),
            "PIP" => Ok(FileType::Pip),
            "SCK" => Ok(FileType::Sck),
            "UNK" => Ok(FileType::Unk),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Invalid file type: {}", s),
            )),
        }
    }
}

impl core::fmt::Display for FileType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FileType {
    pub fn write_as_flex_buffer<W>(mut wtr: W, builder: &mut Builder) -> Result<(), io::Error>
    where
        W: Write,
    {
        builder.reset();
        let mut mp = builder.start_map();
        let typs = [
            FileType::Reg,
            FileType::Sym,
            FileType::Chr,
            FileType::Blk,
            FileType::Dir,
            FileType::Pip,
            FileType::Sck,
            FileType::Unk,
        ];
        for typ in typs {
            let key: &str = typ.as_str();
            let val: u8 = typ as u8;
            mp.push(key, val);
        }
        mp.end_map();
        let serialized: &[u8] = builder.view();
        wtr.write_all(serialized)?;
        Ok(())
    }
}

impl FileType {
    pub fn write_map2stdout(builder: &mut Builder) -> Result<(), io::Error> {
        let o = io::stdout();
        let mut ol = o.lock();
        Self::write_as_flex_buffer(&mut ol, builder)?;
        ol.flush()
    }
}

impl FileType {
    pub fn write_map2stdout_default() -> Result<(), io::Error> {
        let mut bldr = Builder::default();
        Self::write_map2stdout(&mut bldr)
    }
}
