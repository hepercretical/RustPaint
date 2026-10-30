//! Rectangle arithmetic recovered independently of the Windows UI.

/// Four signed 32-bit coordinates in Windows RECT field order.
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Rect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

impl Rect {
    /// Translation of `FUN_140068590` (`mspaint.c:94184`).
    ///
    /// Scale each coordinate with Win32 MulDiv rounding and error behavior.
    /// Each failed coordinate is -1; other coordinates are still calculated.
    pub fn scaled(self, numerator: i32, denominator: i32) -> Self {
        Self {
            left: mul_div(self.left, numerator, denominator),
            top: mul_div(self.top, numerator, denominator),
            right: mul_div(self.right, numerator, denominator),
            bottom: mul_div(self.bottom, numerator, denominator),
        }
    }

    /// Translation of `FUN_1400684b0` (`mspaint.c:94130`).
    ///
    /// Round each coordinate to a multiple of ten; ties go away from zero.
    /// Explicit wrapping preserves machine arithmetic at the i32 boundaries.
    pub fn round_to_tens(&mut self) {
        self.left = round_to_ten(self.left);
        self.right = round_to_ten(self.right);
        self.top = round_to_ten(self.top);
        self.bottom = round_to_ten(self.bottom);
    }
}

/// Portable implementation of the imported Win32 `MulDiv` helper.
///
/// Uses a signed 64-bit product, rounds ties away from zero, and returns -1 for
/// zero denominators or result magnitudes exceeding i32::MAX. In particular,
/// native Windows also rejects a nonzero product with an i32::MIN factor.
/// These boundary details are checked against recorded kernel32 results.
/// A legitimate result of -1 is
/// indistinguishable from failure, as in the original API.
/// See <https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-muldiv>.
pub fn mul_div(value: i32, numerator: i32, denominator: i32) -> i32 {
    if denominator == 0 {
        return -1;
    }
    let product = i64::from(value) * i64::from(numerator);
    if product != 0 && (value == i32::MIN || numerator == i32::MIN) {
        return -1;
    }
    let divisor = i64::from(denominator).abs();
    let magnitude = (product.abs() + divisor / 2) / divisor;
    if magnitude > i64::from(i32::MAX) {
        return -1;
    }
    let result = if (product < 0) != (denominator < 0) {
        -magnitude
    } else {
        magnitude
    };
    i32::try_from(result).unwrap_or(-1)
}

fn round_to_ten(value: i32) -> i32 {
    let base = (value / 10) * 10;
    match value % 10 {
        5..=9 => base.wrapping_add(10),
        -9..=-5 => base.wrapping_sub(10),
        _ => base,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signed_ties_round_away_from_zero() {
        let mut rect = Rect {
            left: -15,
            top: -14,
            right: 14,
            bottom: 15,
        };
        rect.round_to_tens();
        assert_eq!(
            rect,
            Rect {
                left: -20,
                top: -10,
                right: 10,
                bottom: 20
            }
        );
    }

    #[test]
    fn preserves_machine_overflow_at_extremes() {
        assert_eq!(round_to_ten(i32::MAX), -2_147_483_646);
        assert_eq!(round_to_ten(i32::MIN), 2_147_483_646);
        for value in -4..=4 {
            assert_eq!(round_to_ten(value), 0);
        }
    }

    #[test]
    fn scales_each_coordinate_and_preserves_individual_failures() {
        let rect = Rect {
            left: -3,
            top: 3,
            right: i32::MAX,
            bottom: 0,
        };
        assert_eq!(
            rect.scaled(3, 2),
            Rect {
                left: -5,
                top: 5,
                right: -1,
                bottom: 0
            }
        );
        assert_eq!(
            rect.scaled(1, 0),
            Rect {
                left: -1,
                top: -1,
                right: -1,
                bottom: -1
            }
        );
    }
}
