pub fn format_micro_to_milli(micro: &u128) -> String {
    let milli = *micro as f64 / 1_000.0;

    format!("{:.2}ms", milli)
}
