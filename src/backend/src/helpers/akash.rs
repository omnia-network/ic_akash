const UAKT_IN_AKT: f64 = 1_000_000.0;

pub fn uakt_to_akt(uakt: u64) -> f64 {
    uakt as f64 / UAKT_IN_AKT
}
