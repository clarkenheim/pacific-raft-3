const DEFAULT_ENERGY:u8 = 100;
const DEFAULT_HUNGER:u8 = 0;
const DEFAULT_HYGIENE:u8 = 100;
// todo - illness ( vec of illness types )

// Health is a crude indicator of general health
#[derive (Debug, PartialEq)]
pub enum Health {
    Perfect,
    Great,
    Ok,
    Poor,
    Critical
}

// Hygiene is a crude indicator of general hygiene
#[derive (Debug, PartialEq)]
pub enum Hygiene {
    LynxAfrica,
    Passable,
    NotBad,
    Claggy,
    Minging
}

pub struct Player{
    energy:u8,
    hunger: u8,
    hygiene: u8
}

impl Player {

    pub fn new() -> Player {

        Player{
            energy: DEFAULT_ENERGY,
            hunger: DEFAULT_HUNGER,
            hygiene: DEFAULT_HYGIENE
        }
    }

    pub fn get_health(&self) -> Health {
        Health::Perfect
    }
}

#[cfg(test)]
mod tests{

    use player::*;

    #[test]
    fn it_creates_a_player_without_panicking(){
        Player::new();
    }

    #[test]
    fn it_can_get_health(){
        let p = Player::new();
        assert_eq!(p.get_health(), Health::Perfect);
    }
}
