use std::f64::consts::E;

#[derive(Clone, Copy)]
struct World {
    t: f64,
    power: f64,
    limit: f64,
    conscience: f64,
}

impl World {
    fn new() -> Self {
        Self {
            t: 0.0,
            power: 1.0,
            limit: 10_000.0,
            conscience: 0.01,
        }
    }

    fn exponential(&self) -> f64 {
        self.power * E.powf(0.4 * self.t)
    }

    fn logistic(&self) -> f64 {
        self.limit / (1.0 + E.powf(-0.5 * (self.t - 10.0)))
    }

    fn alignment(&self) -> f64 {
        1.0 / (1.0 + E.powf(-0.2 * (self.t - 18.0)))
    }

    fn risk(&self, capability: f64, alignment: f64) -> f64 {
        capability.powi(2) * (1.0 - alignment)
    }

    fn step(mut self) -> Self {
        self.t += 1.0;

        let exp = self.exponential();
        let sat = self.logistic();

        let w = (self.t / 20.0).clamp(0.0, 1.0);
        let capability = (1.0 - w) * exp + w * sat;

        let alignment = self.alignment();
        let risk = self.risk(capability, alignment);

        self.power = capability;
        self.conscience = alignment;

        println!(
            "t={:>4.0} | power={:>12.2} | conscience={:>6.3} | risk={:>14.2}",
            self.t, capability, alignment, risk
        );

        self
    }
}

fn main() {
    let mut world = World::new();

    for _ in 0..30 {
        world = world.step();
    }

    let infinity = f64::INFINITY;

    let meaning = if world.power.is_finite() {
        world.power / (1.0 + world.risk(world.power, world.conscience))
    } else {
        infinity
    };

    println!("meaning ≈ {:.6}", meaning);
}