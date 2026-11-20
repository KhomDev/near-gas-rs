// Probe only (not for a PR): does each whole-unit constructor overflow at runtime?
use near_gas::NearGas;
use std::hint::black_box;
use std::panic::catch_unwind;

fn probe(name: &str, unit: u64, f: fn(u64) -> NearGas) {
    let n = black_box(u64::MAX / unit + 1);
    let mode = if cfg!(debug_assertions) { "debug" } else { "release" };
    match catch_unwind(|| f(n).as_gas()) {
        Ok(v) => println!("PROBE {mode} {name}({n}) = {v} gas; wrapped={}", v == n.wrapping_mul(unit)),
        Err(e) => println!(
            "PROBE {mode} {name}({n}) panicked: {}",
            e.downcast_ref::<String>().cloned().or_else(|| e.downcast_ref::<&str>().map(|s| s.to_string())).unwrap_or_default()
        ),
    }
}

#[test]
fn overflow_probe() {
    probe("from_pgas", 10u64.pow(15), NearGas::from_pgas);
    probe("from_tgas", 10u64.pow(12), NearGas::from_tgas);
    probe("from_ggas", 10u64.pow(9), NearGas::from_ggas);
}
