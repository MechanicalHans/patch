use crate::prelude::*;

type ApplyResult = std::result::Result<(), ApplyError>;

pub fn main(patch: Box<Path>, target: Box<Path>) -> ApplyResult {
    apply(
        File::options()
            .read(true)
            .open(patch)
            .map_err(ApplyError::Patch)?,
        File::options()
            .write(true)
            .open(target)
            .map_err(ApplyError::Target)?,
    )
}

fn apply(mut patch: impl Read, mut target: impl Write + Seek) -> ApplyResult {
    let mut buf = [0; 8];
    // Initially we use the first five bytes of the buffer to store the header.
    patch.read_exact(&mut buf).map_err(ApplyError::Patch)?;
    if &buf[0..5] != b"PATCH" {
        return Err(ApplyError::Patch(io::Error::new(
            io::ErrorKind::InvalidData,
            "invalid header",
        )));
    }
    // Afterwards we divide the buffer into the following regions:
    //   0-2: length
    //   2-4: RLE length
    //   4-5: RLE byte
    //   5-8: offset
    // which allows us to batch multiple reads together.
    while &buf[5..8] != b"EOF" {
        let offset = u64::from_be_bytes([0, 0, 0, 0, 0, buf[5], buf[6], buf[7]]);
        target
            .seek(io::SeekFrom::Start(offset))
            .map_err(ApplyError::Target)?;
        patch
            .read_exact(&mut buf[0..2])
            .map_err(ApplyError::Patch)?;
        let length = u64::from_be_bytes([0, 0, 0, 0, 0, 0, buf[0], buf[1]]);
        if length > 0 {
            memcpy(&mut patch, &mut target, length).map_err(ApplyError::Transfer)?;
            patch
                .read_exact(&mut buf[5..8])
                .map_err(ApplyError::Patch)?;
        } else {
            patch
                .read_exact(&mut buf[2..8])
                .map_err(ApplyError::Patch)?;
            let length = u64::from_be_bytes([0, 0, 0, 0, 0, 0, buf[2], buf[3]]);
            let byte = buf[4];
            memset(&mut target, byte, length).map_err(ApplyError::Target)?;
        }
    }
    target.flush().map_err(ApplyError::Target)?;
    Ok(())
}

fn memcpy(reader: &mut impl Read, writer: &mut impl Write, length: u64) -> io::Result<()> {
    let read = io::copy(&mut reader.take(length), writer)?;
    if read != length {
        return Err(io::Error::from(io::ErrorKind::UnexpectedEof));
    }
    Ok(())
}

fn memset(writer: &mut impl Write, byte: u8, length: u64) -> io::Result<()> {
    memcpy(&mut io::repeat(byte), writer, length)
}

#[derive(Debug)]
pub enum ApplyError {
    Target(io::Error),
    Patch(io::Error),
    Transfer(io::Error),
}

impl fmt::Display for ApplyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Patch(error) => write!(f, "patch IO: {error}"),
            Self::Target(error) => write!(f, "target IO: {error}"),
            Self::Transfer(error) => write!(f, "transfer IO: {error}"),
        }
    }
}

impl Error for ApplyError {}
