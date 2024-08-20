mod names;
mod males;
mod females;
mod occupations;

pub(super) struct Warehouse;

impl Warehouse {
    pub(super) fn gen_gender(rng: &mut impl rand::Rng) -> super::Gender {
        if rng.gen_bool(0.5) {
            super::Gender::Male
        } else {
            super::Gender::Female
        }
    }

    pub(super) fn gen_last_name(rng: &mut impl rand::Rng) -> &'static str {
        names::choose(rng)
    }

    pub(super) fn gen_first_name(rng: &mut impl rand::Rng, gender: super::Gender) -> &'static str {
        match gender {
            super::Gender::Male => males::choose(rng),
            super::Gender::Female => females::choose(rng),
        }
    }

    pub(super) fn gen_occupation(rng: &mut impl rand::Rng) -> &'static str {
        occupations::choose(rng)
    }
}