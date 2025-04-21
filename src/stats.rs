use std::error::Error;
use std::str::FromStr;
use std::sync::RwLock;

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
    stamina: RwLock<u16>, // how many tasks it can execute
    reflexes: u16,        // how quickly it reacts
    dexterity: u16,       // how many times it runs the task
}

pub fn get_from_personality(personality: Personality) -> CronStats {
    return match personality {
        Personality::Eager => CronStats {
            stamina: RwLock::new(10),
            reflexes: 10,
            dexterity: 10,
        },
        Personality::Energetic => CronStats {
            stamina: RwLock::new(10),
            reflexes: 10,
            dexterity: 8,
        },
        Personality::Lazy => CronStats {
            stamina: RwLock::new(9),
            reflexes: 8,
            dexterity: 6,
        },
        Personality::Slow => CronStats {
            stamina: RwLock::new(10),
            reflexes: 4,
            dexterity: 8,
        },
        Personality::Tired => CronStats {
            stamina: RwLock::new(4),
            reflexes: 4,
            dexterity: 2,
        },
    };
}

impl CronStats {
    pub fn complete_task(&mut self) {
        let mut stamina = self.stamina.write().unwrap();
        if *stamina >= 1 {
            *stamina = *stamina - 1
        }
    }
    pub fn is_exhausted(&self) -> bool {
        let stamina = self.stamina.read().unwrap();
        return *stamina == 0;
    }
    pub fn reaction_time(&self) -> u16 {
        return 10 - self.reflexes;
    }
    pub fn tries(&self) -> u16 {
        return 10 - self.dexterity;
    }
}
