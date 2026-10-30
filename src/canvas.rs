//! Owned raster document for the native editor. Pixel words are 0x00RRGGBB.
use crate::bitmap;
pub type Selection = ((i32, i32), (i32, i32));

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tool {
    Pencil,
    Eraser,
    Line,
    Rectangle,
    Ellipse,
    Fill,
    Picker,
    Brush,
    Select,
    Text,
}
#[derive(Clone)]
struct Snapshot {
    width: u32,
    height: u32,
    pixels: Vec<u32>,
}
pub struct Canvas {
    width: u32,
    height: u32,
    pixels: Vec<u32>,
    undo: Vec<Snapshot>,
    redo: Vec<Snapshot>,
}
impl Canvas {
    pub fn new(width: u32, height: u32) -> Result<Self, String> {
        let size = width
            .checked_mul(height)
            .filter(|n| *n <= 16_000_000 && *n > 0)
            .ok_or("canvas dimensions exceed the 16 million pixel limit")?;
        Ok(Self {
            width,
            height,
            pixels: vec![0xffffff; size as usize],
            undo: Vec::new(),
            redo: Vec::new(),
        })
    }
    pub fn pixels(&self) -> &[u32] {
        &self.pixels
    }
    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn height(&self) -> u32 {
        self.height
    }
    pub fn restore_preview(&mut self) {
        if let Some(previous) = self.undo.last() {
            self.width = previous.width;
            self.height = previous.height;
            self.pixels.clone_from(&previous.pixels);
        }
    }
    pub fn checkpoint(&mut self) {
        // Bound history by both operation count and approximate byte budget.
        let limit = (64_000_000 / (self.pixels.len() * 4)).clamp(1, 20);
        while self.undo.len() >= limit
            || (!self.undo.is_empty()
                && self.undo.iter().map(|s| s.pixels.len() * 4).sum::<usize>()
                    + self.pixels.len() * 4
                    > 64_000_000)
        {
            self.undo.remove(0);
        }
        self.undo.push(self.snapshot());
        self.redo.clear();
    }
    pub fn undo(&mut self) -> bool {
        if let Some(previous) = self.undo.pop() {
            self.redo.push(self.snapshot());
            self.restore(previous);
            true
        } else {
            false
        }
    }
    pub fn redo(&mut self) -> bool {
        if let Some(next) = self.redo.pop() {
            self.undo.push(self.snapshot());
            self.restore(next);
            true
        } else {
            false
        }
    }
    pub fn clear(&mut self) {
        self.checkpoint();
        self.pixels.fill(0xffffff);
    }
    fn snapshot(&self) -> Snapshot {
        Snapshot {
            width: self.width,
            height: self.height,
            pixels: self.pixels.clone(),
        }
    }
    fn restore(&mut self, s: Snapshot) {
        self.width = s.width;
        self.height = s.height;
        self.pixels = s.pixels;
    }
    pub fn from_pixels(width: u32, height: u32, pixels: Vec<u32>) -> Result<Self, String> {
        let mut canvas = Self::new(width, height)?;
        if pixels.len() != canvas.pixels.len() {
            return Err("pixel count does not match image dimensions".into());
        }
        canvas.pixels = pixels.into_iter().map(|p| p & 0xffffff).collect();
        Ok(canvas)
    }
    pub fn replace_pixels(&mut self, pixels: Vec<u32>) -> Result<(), String> {
        if pixels.len() != self.pixels.len() {
            return Err("pixel count does not match image dimensions".into());
        }
        self.checkpoint();
        self.pixels = pixels.into_iter().map(|p| p & 0xffffff).collect();
        Ok(())
    }
    pub fn region(&self, a: (i32, i32), b: (i32, i32)) -> Result<Self, String> {
        let x = a.0.min(b.0).clamp(0, self.width as i32 - 1);
        let y = a.1.min(b.1).clamp(0, self.height as i32 - 1);
        let right =
            a.0.max(b.0)
                .saturating_add(1)
                .clamp(x + 1, self.width as i32);
        let bottom =
            a.1.max(b.1)
                .saturating_add(1)
                .clamp(y + 1, self.height as i32);
        let mut image = Self::new((right - x) as u32, (bottom - y) as u32)?;
        for yy in 0..image.height {
            for xx in 0..image.width {
                image.pixels[(yy * image.width + xx) as usize] =
                    self.pixels[((y as u32 + yy) * self.width + x as u32 + xx) as usize];
            }
        }
        Ok(image)
    }
    pub fn erase_region(&mut self, a: (i32, i32), b: (i32, i32)) {
        self.checkpoint();
        let (left, right) = (a.0.min(b.0).max(0), a.0.max(b.0).min(self.width as i32 - 1));
        let (top, bottom) = (
            a.1.min(b.1).max(0),
            a.1.max(b.1).min(self.height as i32 - 1),
        );
        for y in top..=bottom {
            for x in left..=right {
                self.pixels[y as usize * self.width as usize + x as usize] = 0xffffff;
            }
        }
    }
    pub fn paste(&mut self, image: &Self, x: i32, y: i32) {
        self.checkpoint();
        for yy in 0..image.height as i32 {
            for xx in 0..image.width as i32 {
                let (dx, dy) = (x.saturating_add(xx), y.saturating_add(yy));
                if self.get(dx, dy).is_some() {
                    self.pixels[dy as usize * self.width as usize + dx as usize] =
                        image.pixels[yy as usize * image.width as usize + xx as usize];
                }
            }
        }
    }
    pub fn crop(&mut self, a: (i32, i32), b: (i32, i32)) -> Result<(), String> {
        let image = self.region(a, b)?;
        self.checkpoint();
        self.width = image.width;
        self.height = image.height;
        self.pixels = image.pixels;
        Ok(())
    }
    pub fn resize(&mut self, width: u32, height: u32, scale: bool) -> Result<(), String> {
        let mut image = Self::new(width, height)?;
        for y in 0..height {
            for x in 0..width {
                let (sx, sy) = if scale {
                    (
                        x as u64 * self.width as u64 / width as u64,
                        y as u64 * self.height as u64 / height as u64,
                    )
                } else {
                    (x as u64, y as u64)
                };
                if sx < self.width as u64 && sy < self.height as u64 {
                    image.pixels[(y * width + x) as usize] =
                        self.pixels[sy as usize * self.width as usize + sx as usize];
                }
            }
        }
        self.checkpoint();
        self.width = width;
        self.height = height;
        self.pixels = image.pixels;
        Ok(())
    }
    pub fn rotate_right(&mut self) {
        self.rotate(1);
    }
    pub fn rotate(&mut self, turns: u8) {
        let turns = turns % 4;
        if turns == 0 {
            return;
        }
        let (width, height) = if turns % 2 == 1 {
            (self.height, self.width)
        } else {
            (self.width, self.height)
        };
        let mut pixels = vec![0; self.pixels.len()];
        for y in 0..self.height {
            for x in 0..self.width {
                let (dx, dy) = match turns {
                    1 => (self.height - 1 - y, x),
                    2 => (self.width - 1 - x, self.height - 1 - y),
                    _ => (y, self.width - 1 - x),
                };
                pixels[(dy * width + dx) as usize] = self.pixels[(y * self.width + x) as usize];
            }
        }
        self.checkpoint();
        self.width = width;
        self.height = height;
        self.pixels = pixels;
    }
    pub fn paste_expanding(&mut self, image: &Self, x: i32, y: i32) -> Result<(), String> {
        let width = self.width.max(x.max(0) as u32 + image.width);
        let height = self.height.max(y.max(0) as u32 + image.height);
        let mut result = Self::new(width, height)?;
        for yy in 0..self.height {
            let from = (yy * self.width) as usize;
            let to = (yy * width) as usize;
            result.pixels[to..to + self.width as usize]
                .copy_from_slice(&self.pixels[from..from + self.width as usize]);
        }
        for yy in 0..image.height as i32 {
            for xx in 0..image.width as i32 {
                let (dx, dy) = (x.saturating_add(xx), y.saturating_add(yy));
                if result.get(dx, dy).is_some() {
                    result.pixels[dy as usize * width as usize + dx as usize] =
                        image.pixels[yy as usize * image.width as usize + xx as usize];
                }
            }
        }
        self.checkpoint();
        self.width = width;
        self.height = height;
        self.pixels = result.pixels;
        Ok(())
    }
    pub fn move_selection_preview(
        &mut self,
        a: (i32, i32),
        b: (i32, i32),
        dx: i32,
        dy: i32,
    ) -> Result<Selection, String> {
        self.restore_preview();
        let image = self.region(a, b)?;
        let left = a.0.min(b.0).clamp(0, self.width as i32 - 1);
        let top = a.1.min(b.1).clamp(0, self.height as i32 - 1);
        let dx = dx.clamp(-left, self.width as i32 - left - image.width as i32);
        let dy = dy.clamp(-top, self.height as i32 - top - image.height as i32);
        for y in top..top + image.height as i32 {
            for x in left..left + image.width as i32 {
                self.pixels[y as usize * self.width as usize + x as usize] = 0xffffff;
            }
        }
        for y in 0..image.height as i32 {
            for x in 0..image.width as i32 {
                self.pixels
                    [(top + dy + y) as usize * self.width as usize + (left + dx + x) as usize] =
                    image.pixels[y as usize * image.width as usize + x as usize];
            }
        }
        Ok((
            (left + dx, top + dy),
            (
                left + dx + image.width as i32 - 1,
                top + dy + image.height as i32 - 1,
            ),
        ))
    }
    pub fn flip(&mut self, horizontal: bool) {
        self.checkpoint();
        if horizontal {
            for row in self.pixels.chunks_mut(self.width as usize) {
                row.reverse();
            }
        } else {
            for y in 0..self.height / 2 {
                for x in 0..self.width {
                    self.pixels.swap(
                        (y * self.width + x) as usize,
                        ((self.height - 1 - y) * self.width + x) as usize,
                    );
                }
            }
        }
    }
    pub fn invert(&mut self) {
        self.checkpoint();
        for pixel in &mut self.pixels {
            *pixel ^= 0xffffff;
        }
    }
    pub fn get(&self, x: i32, y: i32) -> Option<u32> {
        if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
            None
        } else {
            Some(self.pixels[y as usize * self.width as usize + x as usize])
        }
    }
    fn dot(&mut self, x: i32, y: i32, color: u32, size: i32, round: bool) {
        let size = size.clamp(1, 32);
        let r = size / 2;
        for yy in (y - r).max(0)..(y - r + size).min(self.height as i32) {
            for xx in (x - r).max(0)..(x - r + size).min(self.width as i32) {
                let dx = 2 * (xx - (x - r)) - (size - 1);
                let dy = 2 * (yy - (y - r)) - (size - 1);
                if round && dx * dx + dy * dy > size * size {
                    continue;
                }
                self.pixels[yy as usize * self.width as usize + xx as usize] = color & 0xffffff;
            }
        }
    }
    pub fn draw(&mut self, tool: Tool, from: (i32, i32), to: (i32, i32), color: u32, size: i32) {
        let clamp = |(x, y): (i32, i32)| {
            (
                x.clamp(0, self.width as i32 - 1),
                y.clamp(0, self.height as i32 - 1),
            )
        };
        let (a, b) = (clamp(from), clamp(to));
        match tool {
            Tool::Brush => self.stroke(a, b, color, size, true),
            Tool::Pencil | Tool::Line | Tool::Eraser => self.line(
                a,
                b,
                if tool == Tool::Eraser {
                    0xffffff
                } else {
                    color
                },
                size,
            ),
            Tool::Rectangle => {
                self.line(a, (b.0, a.1), color, size);
                self.line((b.0, a.1), b, color, size);
                self.line(b, (a.0, b.1), color, size);
                self.line((a.0, b.1), a, color, size);
            }
            Tool::Ellipse => {
                let (cx, cy) = ((a.0 + b.0) as f64 / 2.0, (a.1 + b.1) as f64 / 2.0);
                let (rx, ry) = (
                    (a.0 - b.0).unsigned_abs() as f64 / 2.0,
                    (a.1 - b.1).unsigned_abs() as f64 / 2.0,
                );
                let steps = ((rx + ry) * 4.0).ceil().max(12.0) as usize;
                let mut previous = ((cx + rx).round() as i32, cy.round() as i32);
                for i in 1..=steps {
                    let angle = i as f64 / steps as f64 * std::f64::consts::TAU;
                    let point = (
                        (cx + rx * angle.cos()).round() as i32,
                        (cy + ry * angle.sin()).round() as i32,
                    );
                    self.line(previous, point, color, size);
                    previous = point;
                }
            }
            Tool::Fill => self.fill(a, color),
            Tool::Picker | Tool::Select | Tool::Text => {}
        }
    }
    fn line(&mut self, a: (i32, i32), b: (i32, i32), color: u32, size: i32) {
        self.stroke(a, b, color, size, false);
    }
    fn stroke(
        &mut self,
        (mut x, mut y): (i32, i32),
        (tx, ty): (i32, i32),
        color: u32,
        size: i32,
        round: bool,
    ) {
        let dx = (tx - x).abs();
        let sx = if x < tx { 1 } else { -1 };
        let dy = -(ty - y).abs();
        let sy = if y < ty { 1 } else { -1 };
        let mut err = dx + dy;
        loop {
            self.dot(x, y, color, size, round);
            if x == tx && y == ty {
                break;
            }
            let twice = 2 * err;
            if twice >= dy {
                err += dy;
                x += sx;
            }
            if twice <= dx {
                err += dx;
                y += sy;
            }
        }
    }
    fn fill(&mut self, (x, y): (i32, i32), color: u32) {
        let color = color & 0xffffff;
        let Some(old) = self.get(x, y) else {
            return;
        };
        if old == color {
            return;
        }
        let mut pending = vec![(x, y)];
        self.pixels[y as usize * self.width as usize + x as usize] = color;
        while let Some((x, y)) = pending.pop() {
            for (xx, yy) in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] {
                if self.get(xx, yy) == Some(old) {
                    self.pixels[yy as usize * self.width as usize + xx as usize] = color;
                    pending.push((xx, yy));
                }
            }
        }
    }
    pub fn dib_header(&self) -> [u8; 40] {
        let mut header = [0; 40];
        header[..4].copy_from_slice(&40u32.to_le_bytes());
        header[4..8].copy_from_slice(&self.width.to_le_bytes());
        header[8..12].copy_from_slice(&(-(self.height as i32)).to_le_bytes());
        header[12..14].copy_from_slice(&1u16.to_le_bytes());
        header[14..16].copy_from_slice(&32u16.to_le_bytes());
        header
    }
    pub fn bmp(&self) -> Result<Vec<u8>, String> {
        let header = self.dib_header();
        let dib = bitmap::dib_size(&header).map_err(|e| e.to_string())?;
        let mut bytes = Vec::with_capacity(dib as usize + 14);
        bytes.extend_from_slice(b"BM");
        bytes.extend_from_slice(&(dib + 14).to_le_bytes());
        bytes.extend_from_slice(&[0; 4]);
        bytes.extend_from_slice(&54u32.to_le_bytes());
        bytes.extend_from_slice(&header);
        for pixel in &self.pixels {
            bytes.extend_from_slice(&pixel.to_le_bytes());
        }
        Ok(bytes)
    }
    /// Import uncompressed 24/32-bit BMP, retaining top-down/bottom-up orientation.
    pub fn from_bmp(bytes: &[u8]) -> Result<Self, String> {
        if bytes.len() < 54 || &bytes[..2] != b"BM" {
            return Err("not a supported BMP file".into());
        }
        let word =
            |i| u32::from_le_bytes(bytes[i..i + 4].try_into().expect("validated BMP header"));
        let (offset, header, width, height, bits) = (
            word(10) as usize,
            word(14),
            word(18) as i32,
            word(22) as i32,
            u16::from_le_bytes([bytes[28], bytes[29]]),
        );
        if header < 40
            || width <= 0
            || height == 0
            || height == i32::MIN
            || !matches!(bits, 1 | 4 | 8 | 16 | 24 | 32)
            || !matches!(word(30), 0 | 3)
            || word(30) == 3 && !matches!(bits, 16 | 32)
            || bytes[26..28] != [1, 0]
        {
            return Err("unsupported bitmap encoding".into());
        }
        let mut canvas = Self::new(width as u32, height.unsigned_abs())?;
        let stride = (width as usize * bits as usize).div_ceil(32) * 4;
        let end = offset
            .checked_add(
                stride
                    .checked_mul(canvas.height as usize)
                    .ok_or("BMP size overflow")?,
            )
            .ok_or("BMP size overflow")?;
        if offset < 14 + header as usize || end > bytes.len() {
            return Err("truncated BMP pixels or invalid pixel offset".into());
        }
        let palette_count = if bits <= 8 {
            let declared = word(46) as usize;
            if declared == 0 {
                1usize << bits
            } else {
                declared
            }
        } else {
            0
        };
        let palette_start = 14 + header as usize;
        if palette_count > (1usize << bits.min(8))
            || palette_start
                .checked_add(palette_count * 4)
                .is_none_or(|n| n > offset)
        {
            return Err("invalid BMP palette".into());
        }
        let masks = if word(30) == 3 {
            let start = 54;
            if start + 12 > bytes.len() || header == 40 && start + 12 > offset {
                return Err("truncated bitmap color masks".into());
            }
            [word(start), word(start + 4), word(start + 8)]
        } else if bits == 16 {
            [0x7c00, 0x3e0, 0x1f]
        } else {
            [0xff0000, 0xff00, 0xff]
        };
        if masks.contains(&0)
            || masks[0] & masks[1] != 0
            || masks[0] & masks[2] != 0
            || masks[1] & masks[2] != 0
        {
            return Err("invalid bitmap color masks".into());
        }
        for y in 0..canvas.height as usize {
            let row = if height < 0 {
                y
            } else {
                canvas.height as usize - 1 - y
            };
            for x in 0..width as usize {
                let p = offset + row * stride + x * bits as usize / 8;
                let color = if bits <= 8 {
                    let index = match bits {
                        1 => (bytes[p] >> (7 - x % 8)) & 1,
                        4 => {
                            if x % 2 == 0 {
                                bytes[p] >> 4
                            } else {
                                bytes[p] & 15
                            }
                        }
                        _ => bytes[p],
                    } as usize;
                    if index >= palette_count {
                        return Err("bitmap palette index is out of range".into());
                    }
                    let p = palette_start + index * 4;
                    u32::from(bytes[p])
                        | (u32::from(bytes[p + 1]) << 8)
                        | (u32::from(bytes[p + 2]) << 16)
                } else if bits == 24 {
                    u32::from(bytes[p])
                        | (u32::from(bytes[p + 1]) << 8)
                        | (u32::from(bytes[p + 2]) << 16)
                } else {
                    let value = if bits == 16 {
                        u16::from_le_bytes([bytes[p], bytes[p + 1]]) as u32
                    } else {
                        word(p)
                    };
                    let channel = |mask: u32| {
                        let shifted = mask >> mask.trailing_zeros();
                        ((u64::from((value & mask) >> mask.trailing_zeros()) * 255
                            + u64::from(shifted) / 2)
                            / u64::from(shifted)) as u32
                    };
                    (channel(masks[0]) << 16) | (channel(masks[1]) << 8) | channel(masks[2])
                };
                canvas.pixels[y * width as usize + x] = color;
            }
        }
        Ok(canvas)
    }
}
