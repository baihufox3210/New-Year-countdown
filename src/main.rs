//! 新年倒數計時器：以 Rust 編譯為 WebAssembly，直接操作 DOM。

mod countdown;

use countdown::{format_date, format_time, pad2, ring_circumference, ring_offset, year_progress, Remaining};
use js_sys::Date;
use wasm_bindgen::prelude::*;
use web_sys::Document;

/// 取得本地時間指定年份 1 月 1 日 00:00:00 的毫秒時間戳。
fn new_year_ms(year: u32) -> f64 {
    Date::new_with_year_month_day(year, 0, 1).get_time()
}

fn set_text(doc: &Document, id: &str, text: &str) {
    if let Some(el) = doc.get_element_by_id(id) {
        el.set_text_content(Some(text));
    }
}

fn set_attr(doc: &Document, id: &str, name: &str, value: &str) {
    if let Some(el) = doc.get_element_by_id(id) {
        let _ = el.set_attribute(name, value);
    }
}

/// 每秒執行一次：更新時鐘、倒數與年度進度。
fn tick(doc: &Document) {
    let now = Date::new_0();
    let now_ms = now.get_time();
    let year = now.get_full_year();
    let next_year = year + 1;

    // 目前時間與日期
    set_text(
        doc,
        "current-time",
        &format_time(now.get_hours(), now.get_minutes(), now.get_seconds()),
    );
    set_text(
        doc,
        "current-date",
        &format_date(year, now.get_month() + 1, now.get_date(), now.get_day()),
    );

    // 新年倒數
    set_text(doc, "title", &format!("{next_year} 新年倒數"));
    let remaining = Remaining::from_millis(new_year_ms(next_year) - now_ms);
    set_text(doc, "days", &pad2(remaining.days));
    set_text(doc, "hours", &pad2(remaining.hours));
    set_text(doc, "minutes", &pad2(remaining.minutes));
    set_text(doc, "seconds", &pad2(remaining.seconds));

    // 年度進度
    let percent = year_progress(now_ms, new_year_ms(year), new_year_ms(next_year));
    set_text(doc, "progress-title", &format!("{year} 年度完成度"));
    set_text(doc, "progress-text", &format!("{percent:.6}%"));
    set_text(doc, "percent-val", &format!("{}", percent.floor() as u32));
    set_attr(doc, "progress-bar", "stroke-dashoffset", &format!("{:.4}", ring_offset(percent)));
}

fn main() {
    let window = web_sys::window().expect("找不到 window");
    let doc = window.document().expect("找不到 document");

    set_attr(&doc, "progress-bar", "stroke-dasharray", &format!("{:.4}", ring_circumference()));
    tick(&doc);

    let callback = Closure::<dyn FnMut()>::new(move || tick(&doc));
    window
        .set_interval_with_callback_and_timeout_and_arguments_0(callback.as_ref().unchecked_ref(), 1000)
        .expect("無法設定計時器");
    // 計時器需在頁面存活期間持續存在
    callback.forget();
}
