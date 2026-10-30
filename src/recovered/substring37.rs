//! Checked lowering of the UTF-16 substring searches, including PCMPESTRI carry.
use super::call_support::{MemoryFault, MemoryImage, MissingArguments, RecordCall, RelocateCall};
use super::search29::{self, SearchError};

fn words(memory: &MemoryImage, address: u64, bytes: usize) -> Result<[u16; 8], MemoryFault> {
    if bytes > 16 {
        return Err(MemoryFault {
            address,
            width: bytes,
        });
    }
    let mut raw = [0u8; 16];
    for (i, byte) in raw.iter_mut().take(bytes).enumerate() {
        *byte = memory.read_u8(address.wrapping_add(i as u64))?;
    }
    Ok(core::array::from_fn(|i| {
        u16::from_le_bytes([raw[i * 2], raw[i * 2 + 1]])
    }))
}
/// Mode 0x0d: unsigned words, positive equal-ordered results, least-set index.
/// Matches that reach the physical vector edge remain partial candidates;
/// explicit invalid haystack lanes inside the vector reject a candidate.
fn ordered(pattern: &[u16; 8], hay: &[u16; 8], pattern_len: usize, hay_len: usize) -> (u32, bool) {
    for start in 0..8 {
        let mut matches = true;
        for (offset, expected) in pattern.iter().enumerate().take(pattern_len.min(8)) {
            let at = start + offset;
            if at >= 8 {
                break;
            }
            if at >= hay_len || *expected != hay[at] {
                matches = false;
                break;
            }
        }
        if matches {
            return (start as u32, true);
        }
    }
    (8, false)
}
fn equal_bytes(memory: &MemoryImage, a: u64, b: u64, size: u64) -> Result<bool, MemoryFault> {
    for offset in 0..size {
        if memory.read_u8(a.wrapping_add(offset))? != memory.read_u8(b.wrapping_add(offset))? {
            return Ok(false);
        }
    }
    Ok(true)
}
fn search(
    arguments: &[u64],
    memory: &mut MemoryImage,
    resolve: &mut RelocateCall<'_>,
    call: &mut RecordCall<'_>,
) -> Result<Option<u64>, SearchError> {
    if arguments.len() < 4 {
        return Err(SearchError::Arguments(MissingArguments {
            required: 4,
            available: arguments.len(),
        }));
    }
    let (mut cursor, end, needle, count) = (arguments[0], arguments[1], arguments[2], arguments[3]);
    if count == 0 {
        return Ok(Some(cursor));
    }
    if count == 1 {
        let word = memory.read_u16(needle)?;
        return search29::fun_1400b1250(&[cursor, end, u64::from(word)], memory, resolve, call);
    }
    let available = end.wrapping_sub(cursor);
    let size = count.wrapping_mul(2);
    if size > available {
        return Ok(Some(end));
    }
    let feature = memory.read_u8(resolve("DAT_14017909c"))?;
    if feature & 4 == 0 || available < 16 {
        let stop = cursor
            .wrapping_add(available)
            .wrapping_sub(size)
            .wrapping_add(2);
        while cursor != stop {
            if memory.read_u16(cursor)? == memory.read_u16(needle)? {
                let mut index = 1u64;
                while memory.read_u16(cursor.wrapping_add(index.wrapping_mul(2)))?
                    == memory.read_u16(needle.wrapping_add(index.wrapping_mul(2)))?
                {
                    index = index.wrapping_add(1);
                    if index == count {
                        return Ok(Some(cursor));
                    }
                }
            }
            cursor = cursor.wrapping_add(2);
        }
    } else if size <= 16 {
        let pattern = words(memory, needle, size as usize)?;
        let pattern_len = count as u32 as usize;
        let last_block = end.wrapping_sub(16);
        loop {
            let block = words(memory, cursor, 16)?;
            let (index, carry) = ordered(&pattern, &block, pattern_len, 8);
            if carry {
                cursor = cursor.wrapping_add(u64::from(index) * 2);
                if index as i32 <= 8i32.wrapping_sub(count as u32 as i32) {
                    return Ok(Some(cursor));
                }
            } else {
                cursor = cursor.wrapping_add(16);
            }
            if cursor > last_block {
                break;
            }
        }
        let remaining = end.wrapping_sub(cursor);
        if remaining != 0 {
            let tail = words(memory, cursor, remaining as usize)?;
            let (index, carry) = ordered(&pattern, &tail, pattern_len, (remaining >> 1) as usize);
            if carry {
                return Ok(Some(cursor.wrapping_add(u64::from(index) * 2)));
            }
        }
    } else {
        let pattern = words(memory, needle, 16)?;
        let last = cursor.wrapping_sub(size).wrapping_add(available);
        loop {
            let block = words(memory, cursor, 16)?;
            let (index, carry) = ordered(&pattern, &block, 8, 8);
            if carry {
                let mut prefix_matches = index == 0;
                if index != 0 {
                    cursor = cursor.wrapping_add(u64::from(index) * 2);
                    if cursor > last {
                        return Ok(Some(end));
                    }
                    prefix_matches = words(memory, cursor, 16)? == pattern;
                }
                if prefix_matches
                    && equal_bytes(
                        memory,
                        cursor.wrapping_add(16),
                        needle.wrapping_add(16),
                        size.wrapping_sub(16),
                    )?
                {
                    return Ok(Some(cursor));
                }
                cursor = cursor.wrapping_add(2);
            } else {
                cursor = cursor.wrapping_add(16);
            }
            if cursor > last {
                break;
            }
        }
    }
    Ok(Some(end))
}
pub fn fun_1400b1350(
    arguments: &[u64],
    memory: &mut MemoryImage,
    resolve: &mut RelocateCall<'_>,
    call: &mut RecordCall<'_>,
) -> Result<Option<u64>, SearchError> {
    search(arguments, memory, resolve, call)
}
pub fn named_l156673(
    arguments: &[u64],
    memory: &mut MemoryImage,
    resolve: &mut RelocateCall<'_>,
    call: &mut RecordCall<'_>,
) -> Result<Option<u64>, SearchError> {
    search(arguments, memory, resolve, call)
}

#[cfg(test)]
mod tests {
    use super::ordered;
    #[test]
    fn equal_ordered_masks_match_real_pcmpe_stri() {
        for line in include_str!("../../tests/fixtures/pcmp37.csv").lines() {
            let p: Vec<usize> = line.split(',').map(|x| x.parse().unwrap()).collect();
            let seed = p[0];
            let a = core::array::from_fn(|i| {
                (if seed.is_multiple_of(4) {
                    1
                } else if seed % 4 == 1 {
                    i % 3
                } else {
                    (i * 37 + seed) % 5
                }) as u16
            });
            let b = core::array::from_fn(|i| {
                (if seed.is_multiple_of(4) {
                    1
                } else if seed % 4 == 1 {
                    (i + seed) % 3
                } else {
                    (i * 7 + seed) % 5
                }) as u16
            });
            assert_eq!(
                ordered(&a, &b, p[1], p[2]),
                (p[3] as u32, p[4] != 0),
                "{line}"
            );
        }
    }
}
