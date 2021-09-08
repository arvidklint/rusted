use crate::character::Character;
use crate::math::clamp;

pub struct Line {
    characters: Vec<Character>,
}

impl Line {
    pub fn new(content: &str) -> Self {
        Self {
            characters: content
                .to_string()
                .chars()
                .map(|c| Character::new(c))
                .collect(),
        }
    }

    pub fn from_character_list(characters: Vec<Character>) -> Self {
        Self { characters }
    }

    pub fn len(&self) -> usize {
        self.characters.len()
    }

    pub fn insert(&mut self, col: usize, c: char) {
        let col = clamp(col as isize, 0, self.len() as isize);
        self.characters.insert(col as usize, Character::new(c));
    }

    pub fn append(&mut self, characters: &mut Vec<Character>) {
        self.characters.append(characters);
    }

    pub fn delete(&mut self, col: usize) {
        self.characters.remove(col);
    }

    pub fn split_off(&mut self, col: usize) -> Vec<Character> {
        self.characters.split_off(col)
    }

    pub fn get_string(&self) -> String {
        self.characters
            .iter()
            .map(|c| c.character.to_string())
            .collect()
    }
}
