use super::*;
use crate::body::Habitability;
use crate::nation::FoodProductionTarget;

impl Colonies {
    pub fn update_food_production_rate(&mut self, nations: &Nations, bodies: &Bodies) {
        let food_required = self.population.iter()
            .map(Population::get_food_requirement);

        let habitability = self.body.iter()
            .map(|b| bodies.get_habitability(b));

        self.food_production.iter_mut()
            .zip(food_required)
            .zip(habitability)
            .zip(self.nation.iter())
            .for_each(|(((production, consumption), habitability), nation)| {
                let get_national_target = || nations.get_food_production_target(nation);

                *production += Self::get_new_food_production(
                    *production,
                    consumption,
                    habitability,
                    get_national_target,
                );
            });
    }

    fn get_new_food_production<F>(
        production: MassRate,
        consumption: MassRate,
        habitability: Habitability,
        get_national_target: F,
    ) -> MassRate
        where
            F: FnOnce() -> Option<FoodProductionTarget>,
    {
        let target = Self::get_colony_target_override(production, consumption, habitability)
            .or_else(get_national_target)
            .unwrap_or_else(|| FoodProductionTarget::Stable);

        let production_multiplier = target.get_multiplier() * habitability.get_food_production_factor(target);

        consumption * production_multiplier * YEAR_FRACTION
    }

    /// Expand production if colony is not self-sufficient and is well-suited to be so
    fn get_colony_target_override(
        production: MassRate,
        consumption: MassRate,
        habitability: Habitability,
    ) -> Option<FoodProductionTarget> {
        let self_sufficiency = production / consumption;

        if self_sufficiency < 1.02 && habitability == Habitability::Optimal {
            Some(FoodProductionTarget::Expand)
        } else {
            None
        }
    }
}

const YEAR_FRACTION: f64 = System::ColonyFoodProductionRate.get_interval_as_year_fraction();

#[cfg(test)]
mod tests {
    use super::*;
    use Habitability::*;
    use FoodProductionTarget::*;

    #[test]
    fn get_colony_target_override_returns_expand_if_hungry_and_able_to_farm() {
        assert_eq!(
            Some(Expand),
            Colonies::get_colony_target_override(MassRate::zero(), MassRate::in_kg_per_s(1.0), Optimal)
        );
    }

    #[test]
    fn get_colony_target_override_returns_none_if_hungry_and_unable_to_farm() {
        assert_eq!(
            None,
            Colonies::get_colony_target_override(MassRate::zero(), MassRate::in_kg_per_s(1.0), Hostile)
        );
    }

    #[test]
    fn get_colony_target_override_returns_none_if_well_fed_and_able_to_farm() {
        assert_eq!(
            None,
            Colonies::get_colony_target_override(MassRate::in_kg_per_s(2.0), MassRate::in_kg_per_s(1.0), Hostile)
        );
    }
}
