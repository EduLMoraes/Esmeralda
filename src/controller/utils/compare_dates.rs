use chrono::Datelike;
use chrono::NaiveDate;

type BaseDT = NaiveDate;
type UnitSize = i32;

pub fn signed_month_difference(start: BaseDT, end: BaseDT) -> UnitSize {
    let end_naive = end;
    let start_naive = start;

    let month_diff = end_naive.month() as UnitSize - start_naive.month() as UnitSize;
    let years_diff = (end_naive.year() - start_naive.year()) as UnitSize;

    if month_diff > 0 {
        (years_diff * 12) + month_diff
    } else {
        (years_diff - 1) * 12 + (month_diff + 12)
    }
}
