//! 與瀏覽器無關的純計算邏輯，可直接以 `cargo test` 在本機測試。

const MS_PER_SECOND: f64 = 1_000.0;
const MS_PER_MINUTE: f64 = MS_PER_SECOND * 60.0;
const MS_PER_HOUR: f64 = MS_PER_MINUTE * 60.0;
const MS_PER_DAY: f64 = MS_PER_HOUR * 24.0;

/// SVG 圓環半徑（需與 index.html 中的 `r` 一致）。
pub const RING_RADIUS: f64 = 45.0;

/// 距離目標時間的剩餘時間。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Remaining {
    pub days: u64,
    pub hours: u64,
    pub minutes: u64,
    pub seconds: u64,
}

impl Remaining {
    /// 由毫秒差值計算剩餘的天、時、分、秒；差值為負時視為 0。
    pub fn from_millis(diff_ms: f64) -> Self {
        let diff = diff_ms.max(0.0);
        Self {
            days: (diff / MS_PER_DAY).floor() as u64,
            hours: ((diff / MS_PER_HOUR).floor() as u64) % 24,
            minutes: ((diff / MS_PER_MINUTE).floor() as u64) % 60,
            seconds: ((diff / MS_PER_SECOND).floor() as u64) % 60,
        }
    }
}

/// 年度完成百分比（0.0 ~ 100.0）。
pub fn year_progress(now_ms: f64, start_ms: f64, end_ms: f64) -> f64 {
    let total = end_ms - start_ms;
    if total <= 0.0 {
        return 0.0;
    }
    ((now_ms - start_ms) / total * 100.0).clamp(0.0, 100.0)
}

/// 圓環周長。
pub fn ring_circumference() -> f64 {
    2.0 * std::f64::consts::PI * RING_RADIUS
}

/// 依百分比計算 SVG `stroke-dashoffset`。
pub fn ring_offset(percent: f64) -> f64 {
    let c = ring_circumference();
    c - (percent / 100.0) * c
}

/// 兩位數補零。
pub fn pad2(n: u64) -> String {
    format!("{n:02}")
}

/// 格式化時間：`HH:MM:SS`。
pub fn format_time(hours: u32, minutes: u32, seconds: u32) -> String {
    format!("{hours:02}:{minutes:02}:{seconds:02}")
}

/// 格式化日期：`2026年1月1日 星期四`。`month` 為 1~12，`weekday` 為 0(日)~6(六)。
pub fn format_date(year: u32, month: u32, day: u32, weekday: u32) -> String {
    const WEEKDAYS: [&str; 7] = ["日", "一", "二", "三", "四", "五", "六"];
    let w = WEEKDAYS[(weekday % 7) as usize];
    format!("{year}年{month}月{day}日 星期{w}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remaining_splits_units() {
        let ms = 3.0 * MS_PER_DAY + 4.0 * MS_PER_HOUR + 5.0 * MS_PER_MINUTE + 6.0 * MS_PER_SECOND + 999.0;
        assert_eq!(
            Remaining::from_millis(ms),
            Remaining { days: 3, hours: 4, minutes: 5, seconds: 6 }
        );
    }

    #[test]
    fn remaining_negative_is_zero() {
        assert_eq!(
            Remaining::from_millis(-1000.0),
            Remaining { days: 0, hours: 0, minutes: 0, seconds: 0 }
        );
    }

    #[test]
    fn progress_bounds() {
        assert_eq!(year_progress(0.0, 0.0, 100.0), 0.0);
        assert_eq!(year_progress(50.0, 0.0, 100.0), 50.0);
        assert_eq!(year_progress(200.0, 0.0, 100.0), 100.0);
        assert_eq!(year_progress(10.0, 10.0, 10.0), 0.0);
    }

    #[test]
    fn ring_offset_ends() {
        assert!((ring_offset(0.0) - ring_circumference()).abs() < 1e-9);
        assert!(ring_offset(100.0).abs() < 1e-9);
    }

    #[test]
    fn formatting() {
        assert_eq!(pad2(7), "07");
        assert_eq!(format_time(9, 5, 0), "09:05:00");
        assert_eq!(format_date(2026, 1, 1, 4), "2026年1月1日 星期四");
    }
}
