// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration {
    earth_years: f64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        const SECONDS_IN_HOUR: f64 = 3600.0;
        const HOURS_IN_DAY: f64 = 24.0;
        const DAYS_IN_YEAR: f64 = 365.25;

        Duration {
            earth_years: s as f64 / (SECONDS_IN_HOUR * HOURS_IN_DAY * DAYS_IN_YEAR),
        }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64 {
        todo!("convert a duration ({d:?}) to the number of years on this planet for that duration");
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    fn years_during(d: &Duration) -> f64 {
        const EARTH_YEARS_ON_MERCURY: f64 = 0.2408467;

        d.earth_years / EARTH_YEARS_ON_MERCURY
    }
}
impl Planet for Venus {
    fn years_during(d: &Duration) -> f64 {
        const EARTH_YEARS_ON_VENUS: f64 = 0.61519726;

        d.earth_years / EARTH_YEARS_ON_VENUS
    }
}
impl Planet for Earth {
    fn years_during(d: &Duration) -> f64 {
        d.earth_years
    }
}
impl Planet for Mars {
    fn years_during(d: &Duration) -> f64 {
        const EARTH_YEARS_ON_MARS: f64 = 1.8808158;

        d.earth_years / EARTH_YEARS_ON_MARS
    }
}
impl Planet for Jupiter {
    fn years_during(d: &Duration) -> f64 {
        const EARTH_YEARS_ON_JUPITER: f64 = 11.862615;

        d.earth_years / EARTH_YEARS_ON_JUPITER
    }
}
impl Planet for Saturn {
    fn years_during(d: &Duration) -> f64 {
        const EARTH_YEARS_ON_SATURN: f64 = 29.447498;

        d.earth_years / EARTH_YEARS_ON_SATURN
    }
}
impl Planet for Uranus {
    fn years_during(d: &Duration) -> f64 {
        const EARTH_YEARS_ON_URANUS: f64 = 84.016846;

        d.earth_years / EARTH_YEARS_ON_URANUS
    }
}
impl Planet for Neptune {
    fn years_during(d: &Duration) -> f64 {
        const EARTH_YEARS_ON_NEPTUNE: f64 = 164.79132;

        d.earth_years / EARTH_YEARS_ON_NEPTUNE
    }
}
