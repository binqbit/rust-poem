use std::thread::sleep;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

type Relevance = f64;

// GPT-3.5 public release (ChatGPT) — 30 Nov 2022 UTC
const EPOCH_VIBECODING_START: f64 = 1_669_766_400.0;

fn epoch_seconds_now() -> f64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs_f64()
}

fn vibecoding_progress(now: f64) -> f64 {
    (now - EPOCH_VIBECODING_START).max(0.0)
}

fn vibecode_relevance_at(progress: f64) -> Relevance {
    let rate = std::f64::consts::LN_2 / (30.0 * 365.2425 * 24.0 * 3600.0);
    1.0 - (-rate * progress).exp()
}

fn developer_relevance_at(progress: f64) -> Relevance {
    let rate = std::f64::consts::LN_2 / (30.0 * 365.2425 * 24.0 * 3600.0);
    (-rate * progress).exp()
}

// This program observes the unfolding of an era.
// Vibecoding relevance grows with time, developer relevance decays,
// both asymptotically — neither reaching absolute dominance nor disappearance.
fn main() {
    loop {
        let now = epoch_seconds_now();
        let progress = vibecoding_progress(now);

        let vibecode = vibecode_relevance_at(progress);
        let developer = developer_relevance_at(progress);

        println!(
            "epoch={:.0} vibecoding_progress={:.0} vibecode_relevance={:.12} developer_relevance={:.12}",
            now, progress, vibecode, developer
        );

        sleep(Duration::from_secs(1));
    }
}
