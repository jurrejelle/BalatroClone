// Modifiers taken from https://balatrogame.fandom.com/wiki/Card_Modifiers
#[derive(Debug, Copy, Clone)]
pub enum CardEnhancement {
    None,
    Bonus,
    Mult,
    Wild,
    Glass,
    Steel,
    Stone,
    Gold,
    Lucky
}


impl Default for CardEnhancement {
    fn default() -> Self {
        CardEnhancement::None
    }
}

#[derive(Debug, Copy, Clone)]
pub enum CardSeals{
    None,
    GoldSeal,
    RedSeal,
    BlueSeal,
    PurpleSeal
}
impl Default for CardSeals {
    fn default() -> Self {
        CardSeals::None
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum JokerEdition {
    Base,
    Foil,
    Holographic,
    Polychrome,
    Negative
}
impl Default for JokerEdition {
    fn default() -> Self {
        JokerEdition::Base
    }
}