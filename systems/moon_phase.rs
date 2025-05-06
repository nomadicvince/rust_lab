use chrono::{Datelike, NaiveDate, Utc};

fn main() {
    let today = Utc::now().date_naive();
    let phase = moon_phase(today);
    let (desc, emoji) = phase_description(phase);

    println!("Date: {}", today);
    println!("Moon Phase: {} {}", emoji, desc);
}

fn moon_phase(date: NaiveDate) -> f64 {
    let year = date.year() as f64;
    let month = date.month() as f64;
    let day = date.day() as f64;

    let (mut y, mut m) = (year, month);
    if m < 3.0 {
        y -= 1.0;
        m += 12.0;
    }

    let a = (y / 100.0).floor();
    let b = 2.0 - a + (a / 4.0).floor();
    let jd = (365.25 * (y + 4716.0)).floor()
        + (30.6001 * (m + 1.0)).floor()
        + day + b - 1524.5;

    let days_since_new = jd - 2451549.5;
    let new_moons = days_since_new / 29.53058867;

    new_moons.fract()
}

fn phase_description(phase: f64) -> (&'static str, &'static str) {
    match phase {
        p if p < 0.03 || p > 0.97 => ("New Moon", "🌑"),
        p if p < 0.22 => ("Waxing Crescent", "🌒"),
        p if p < 0.28 => ("First Quarter", "🌓"),
        p if p < 0.47 => ("Waxing Gibbous", "🌔"),
        p if p < 0.53 => ("Full Moon", "🌕"),
        p if p < 0.72 => ("Waning Gibbous", "🌖"),
        p if p < 0.78 => ("Last Quarter", "🌗"),
        _ => ("Waning Crescent", "🌘"),
    }
}