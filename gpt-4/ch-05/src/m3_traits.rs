trait Attacker {
    fn choose_style(&self) -> String;
    //fn choose_weapon(&self) -> String;
}

#[derive(Debug)]
enum Character {
    Warrior,
    Archer,
    Wisard,
}

impl Attacker for Character {
    fn choose_style(&self) -> String {
        match self {
            Character::Warrior => "my Warrior".to_string(),
            Character::Archer => "my Archer".to_string(),
            Character::Wisard => "my Wisard".to_string(),
            // _ => "my None".to_string(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tests_traits() {
        //dbg!("Hello tests_traits!");
        let my_character = Character::Wisard;
        let my_character_dbg = my_character.choose_style();
        dbg!(my_character_dbg);
    }
}
