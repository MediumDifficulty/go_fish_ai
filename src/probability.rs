use std::ops;

/// Represents a probability
#[derive(Debug, Clone, Copy)]
pub enum Probability {
    /// We think that the actual value is about `x`
    Unknown(f32),
    /// We know that the value is `x`
    Known(usize),
    /// We know that the actual value is greater than or equal to `x`
    MoreThan(usize),
}


impl Probability {
    /// Gets the estimated value of the probability
    pub fn value(&self) -> f32 {
        match self {
            Probability::Unknown(x) => *x,
            Probability::Known(x) => *x as f32,
            Probability::MoreThan(x) => *x as f32,
        }
    }
}

impl ops::Add<Probability> for Probability {
    type Output = Self;

    fn add(self, rhs: Probability) -> Self::Output {
        match self {
            Probability::Unknown(x) => match rhs {
                Probability::Unknown(y) => Probability::Unknown(x + y),
                Probability::Known(y) => Probability::MoreThan(y),
                Probability::MoreThan(y) => Probability::MoreThan(y),
            },
            Probability::Known(x) => match rhs {
                Probability::Unknown(_) => Probability::MoreThan(x),
                Probability::Known(y) => Probability::Known(x + y),
                Probability::MoreThan(y) => Probability::MoreThan(x + y),
            },
            Probability::MoreThan(x) => match rhs {
                Probability::Unknown(_) => Probability::MoreThan(x),
                Probability::Known(y) => Probability::MoreThan(x + y),
                Probability::MoreThan(y) => Probability::MoreThan(x + y),
            },
        }
    }
}

impl ops::AddAssign<Probability> for Probability {
    fn add_assign(&mut self, rhs: Probability) {
        *self = *self + rhs;
    }
}

impl ops::Sub<Probability> for Probability {
    type Output = Self;

    fn sub(self, rhs: Probability) -> Self::Output {
        match self {
            Probability::Unknown(x) => match rhs {
                Probability::Unknown(y) => Probability::Unknown((x - y).max(0.)),
                Probability::Known(y) => Probability::Unknown((x - y as f32).max(0.)), // I'm not sure if this really makes sense but it'll work for now...
                Probability::MoreThan(y) => Probability::MoreThan(y),
            },
            Probability::Known(x) => match rhs {
                Probability::Unknown(_) => Probability::Known(x),
                Probability::Known(y) => Probability::Known(x.saturating_sub(y)),
                Probability::MoreThan(y) => Probability::MoreThan(x.saturating_sub(y)),
            },
            Probability::MoreThan(x) => match rhs {
                Probability::Unknown(_) => Probability::MoreThan(x),
                Probability::Known(y) => Probability::MoreThan(x.saturating_sub(y)),
                Probability::MoreThan(y) => Probability::MoreThan(x.saturating_sub(y)),
            },
        }
    }
}

impl ops::SubAssign<Probability> for Probability {
    fn sub_assign(&mut self, rhs: Probability) {
        *self = *self - rhs;
    }
}