use std::error::Error;
use std::str::FromStr;

pub enum Personality {
    Eager,
    Energetic,
    Lazy,
    Slow,
    Tired,
}

impl FromStr for Personality {
    type Err = Box<dyn Error>;
    fn from_str(p: &str) -> Result<Self, Self::Err> {
        return match p {
            "eager" => Ok(Personality::Eager),
            "energetic" => Ok(Personality::Energetic),
            "lazy" => Ok(Personality::Lazy),
            "slow" => Ok(Personality::Slow),
            "tired" => Ok(Personality::Tired),
            _ => Ok(Personality::Slow),
        };
    }
}

pub struct CronStats {
    stamina: u8,   // how many tasks it can execute
    reflexes: u8,  // how quickly it reacts
    dexterity: u8, // how many times it tries to run the task
}

pub fn get_from_personality(personality: Personality) -> CronStats {
    return match personality {
        Personality::Eager => CronStats {
            stamina: 10,
            reflexes: 10,
            dexterity: 10,
        },
        Personality::Energetic => CronStats {
            stamina: 10,
            reflexes: 10,
            dexterity: 8,
        },
        Personality::Lazy => CronStats {
            stamina: 9,
            reflexes: 8,
            dexterity: 6,
        },
        Personality::Slow => CronStats {
            stamina: 10,
            reflexes: 4,
            dexterity: 8,
        },
        Personality::Tired => CronStats {
            stamina: 4,
            reflexes: 4,
            dexterity: 2,
        },
    };
}

impl CronStats {
    pub fn complete_task(&mut self) {
        if self.stamina >= 1 {
            self.stamina = self.stamina - 1
        }
    }
    pub fn is_exhausted(&self) -> bool {
        return self.stamina == 0;
    }
}
