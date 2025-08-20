use crate::rangemap::RangeMap;

pub struct Instructions {
    seed2soil_map: RangeMap,
    soil2fertilizer_map: RangeMap,
    fertilizer2water_map: RangeMap,
    water2light_map: RangeMap,
    light2temperature_map: RangeMap,
    temperature2humidity_map: RangeMap,
    humidity2location_map: RangeMap,
}

impl Instructions {
    pub fn new(
        seed2soil_map: RangeMap,
        soil2fertilizer_map: RangeMap,
        fertilizer2water_map: RangeMap,
        water2light_map: RangeMap,
        light2temperature_map: RangeMap,
        temperature2humidity_map: RangeMap,
        humidity2location_map: RangeMap,
    ) -> Self {
        Self {
            seed2soil_map,
            soil2fertilizer_map,
            fertilizer2water_map,
            water2light_map,
            light2temperature_map,
            temperature2humidity_map,
            humidity2location_map,
        }
    }

    pub fn seed2soil(&self) -> &RangeMap {
        &self.seed2soil_map
    }

    pub fn soil2fertilizer(&self) -> &RangeMap {
        &self.soil2fertilizer_map
    }

    pub fn fertilizer2water(&self) -> &RangeMap {
        &self.fertilizer2water_map
    }

    pub fn water2light(&self) -> &RangeMap {
        &self.water2light_map
    }

    pub fn light2temperature(&self) -> &RangeMap {
        &self.light2temperature_map
    }

    pub fn temperature2humidity(&self) -> &RangeMap {
        &self.temperature2humidity_map
    }

    pub fn humidity2location(&self) -> &RangeMap {
        &self.humidity2location_map
    }
}
