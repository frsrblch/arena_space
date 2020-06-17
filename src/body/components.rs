use Surface::*;
use Pressure::*;
use Habitability::*;

/// The ability of an environment to support human life.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Habitability {
    /// Unable to support human life in any capacity, (i.e., Jupiter, Venus).
    Uninhabitable,
    /// Unable to support human life with significant assistance (i.e., Mars, full vacuum).
    Hostile,
    /// Barely able to support human life (i.e., dessert, tundra, high altitudes).
    Marginal,
    /// Able to support human life (i.e., grasslands, forests).
    Optimal,
}

impl Default for Habitability {
    fn default() -> Self {
        Habitability::Uninhabitable
    }
}

/// Describes the conditions of a body
#[derive(Debug, Copy, Clone)]
pub struct BodyProperties {
    pub surface: Surface,
    pub pressure: Pressure,
    pub oxygen: AtmosphericOxygen,
    pub hydrosphere: Hydrosphere,
    pub biosphere: Biosphere,
    pub magnetosphere: Magnetosphere,
}

impl BodyProperties {
    pub fn get_habitability(&self) -> Habitability {
        self.surface.get_habitability()
            .min(self.pressure.get_habitability())
            .min(self.hydrosphere.get_habitability())
            .min(self.biosphere.get_habitability())
            .min(self.oxygen.get_habitability())
    }
}

/// Describes the surface of a body (if it has one)
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Surface {
    /// The surface of a gas giant
    Gaseous,
    /// A rocky surface with significant volcanic activity
    Volcanic,
    /// A glacial planet covered by ice
    Frozen,
    /// A barren, rocky surface, e.g., Mars, the Moon
    Rocky,
    /// A mix of land mass and ocean
    Continental,
    /// A planet dominated by water
    Oceanic,
}

impl Surface {
    pub fn get_habitability(&self) -> Habitability {
        match self {
            Gaseous => Uninhabitable,
            Volcanic => Hostile,
            Frozen => Hostile,
            Rocky => Marginal,
            Oceanic => Marginal,
            Continental => Optimal,
        }
    }
}

/// Describes the atmospheric pressure of a body
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Pressure {
    /// No atmosphere
    Vacuum,
    /// An atmosphere that is thinner than is desirable
    Thin,
    /// An atmospheric pressure than is ideal for human life
    Ideal,
    /// An atmosphere that is thicker than is desirable
    High,
    /// An atmosphere so dense that it cannot support human life
    Crushing,
}

impl Pressure {
    pub fn get_habitability(&self) -> Habitability {
        match self {
            Vacuum => Hostile,
            Thin => Marginal,
            Ideal => Optimal,
            High => Marginal,
            Crushing => Uninhabitable,
        }
    }
}

/// A planetary magnetic field that preventing erosion of the atmosphere by stellar wind.
///
/// Created by a molten core in rocky planets and metallic hydrogen in gas giants.
///
/// Used during stellar system generation to determine whether the planet can support and atmosphere.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Magnetosphere {
    /// No strong magnetic field protects this body from stellar wind. Lighter elements will be ionized and blown away.
    Absent,
    /// A magnetic field protects this planet from the stellar wind
    Present,
}

/// The availability of water
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Hydrosphere {
    /// No water present
    None,
    /// Water available in its solid form
    Frozen,
    /// Water is found in all forms as part of the dynamic systems of the planet
    Dynamic,
}

impl Hydrosphere {
    pub fn get_habitability(&self) -> Habitability {
        match self {
            Hydrosphere::None => Marginal,
            Hydrosphere::Frozen => Optimal,
            Hydrosphere::Dynamic => Optimal,
        }
    }
}

/// The presence and evolutionary stage of life on this body
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Biosphere {
    /// Lifeless
    None,
    /// Simple organisms have developed and only started to terraform the planet (e.g., Precambrian Earth)
    Primordial,
    /// Advanced multicellular organisms have developed and transformed the planet, as happened on Earth
    Advanced,
}

impl Biosphere {
    pub fn get_habitability(&self) -> Habitability {
        match self {
            Biosphere::None => Marginal,
            Biosphere::Primordial => Marginal,
            Biosphere::Advanced => Optimal,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum AtmosphericOxygen {
    None,
    Partial,
    Ideal,
}

impl AtmosphericOxygen {
    pub fn get_habitability(&self) -> Habitability {
        match self {
            AtmosphericOxygen::None => Hostile,
            AtmosphericOxygen::Partial => Marginal,
            AtmosphericOxygen::Ideal => Optimal,
        }
    }
}

pub mod examples {
    use super::*;

    pub fn moon() -> BodyProperties {
        BodyProperties {
            surface: Surface::Rocky,
            pressure: Pressure::Vacuum,
            oxygen: AtmosphericOxygen::None,
            hydrosphere: Hydrosphere::None,
            biosphere: Biosphere::None,
            magnetosphere: Magnetosphere::Absent,
        }
    }

    #[test]
    fn moon_habitability() {
        assert_eq!(Hostile, moon().get_habitability());
    }

    pub fn earth() -> BodyProperties {
        BodyProperties {
            surface: Surface::Continental,
            pressure: Pressure::Ideal,
            oxygen: AtmosphericOxygen::Ideal,
            hydrosphere: Hydrosphere::Dynamic,
            biosphere: Biosphere::Advanced,
            magnetosphere: Magnetosphere::Present,
        }
    }

    #[test]
    fn earth_habitability() {
        assert_eq!(Optimal, earth().get_habitability());
    }

    pub fn jupiter() -> BodyProperties {
        BodyProperties {
            surface: Surface::Gaseous,
            pressure: Pressure::Crushing,
            oxygen: AtmosphericOxygen::None,
            hydrosphere: Hydrosphere::None,
            biosphere: Biosphere::None,
            magnetosphere: Magnetosphere::Present
        }
    }

    #[test]
    fn jupiter_habitability() {
        assert_eq!(Uninhabitable, jupiter().get_habitability());
    }
    
    pub fn venus() -> BodyProperties {
        BodyProperties {
            surface: Surface::Volcanic,
            pressure: Pressure::Crushing,
            oxygen: AtmosphericOxygen::None,
            hydrosphere: Hydrosphere::None,
            biosphere: Biosphere::None,
            magnetosphere: Magnetosphere::Absent,
        }
    }

    #[test]
    fn venus_habitability() {
        assert_eq!(Uninhabitable, venus().get_habitability());
    }
}