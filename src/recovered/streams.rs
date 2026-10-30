//! Safe state model for two UTF-16 stream-buffer virtual methods. Indirect get,
//! put and beginning pointers become optional indices into a shared slice.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InvalidStreamState;
impl std::fmt::Display for InvalidStreamState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("stream pointer index is outside its buffer")
    }
}
impl std::error::Error for InvalidStreamState {}

#[derive(Debug)]
pub struct Utf16Stream<'a> {
    pub buffer: &'a mut [u16],
    pub get: Option<usize>,
    pub put: Option<usize>,
    pub begin: usize,
    pub available: i32,
    pub high_water: usize,
    pub flags: u8,
}

impl Utf16Stream<'_> {
    fn validate(&self) -> Result<(), InvalidStreamState> {
        if self.begin > self.buffer.len()
            || self.high_water > self.buffer.len()
            || self.get.is_some_and(|index| index > self.buffer.len())
            || self.put.is_some_and(|index| index > self.buffer.len())
        {
            Err(InvalidStreamState)
        } else {
            Ok(())
        }
    }
}

/// `FUN_140014350`: peek without advancing, extending readable data to the high
/// water mark when output is enabled. EOF is 0xffff, as in the original.
pub fn fun_140014350(stream: &mut Utf16Stream<'_>) -> Result<u16, InvalidStreamState> {
    stream.validate()?;
    if let Some(get) = stream.get {
        if stream.available > 0 {
            return stream.buffer.get(get).copied().ok_or(InvalidStreamState);
        }
        if let Some(put) = stream.put {
            if stream.flags & 4 == 0 {
                let high = stream.high_water.max(put);
                if get < high {
                    let value = *stream.buffer.get(get).ok_or(InvalidStreamState)?;
                    stream.high_water = high;
                    stream.available = (high - get) as i32;
                    return Ok(value);
                }
            }
        }
    }
    Ok(0xffff)
}

/// `FUN_1400143c0`: put back a unit or unget when `value == -1`. A mismatching
/// unit is rejected if flag two prohibits writing. Failure returns -1.
pub fn fun_1400143c0(stream: &mut Utf16Stream<'_>, value: i16) -> Result<i16, InvalidStreamState> {
    stream.validate()?;
    if let Some(get) = stream.get {
        if get > stream.begin {
            let previous = stream.buffer[get - 1];
            if value == -1 || value == previous as i16 || stream.flags & 2 == 0 {
                stream.available = stream.available.wrapping_add(1);
                stream.get = Some(get - 1);
                if value != -1 {
                    stream.buffer[get - 1] = value as u16;
                }
                return Ok(if value == -1 { 0 } else { value });
            }
        }
    }
    Ok(-1)
}
