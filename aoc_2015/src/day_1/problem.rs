use crate::utils::util::parse_input;


struct Building {
    basement_pos: usize,
    instructions: String,
}

impl Building {
    fn with_file_name(file_name: &str) -> Building { 
        Self {
            basement_pos: 0,
            instructions: parse_input(file_name),
        }
    }

    fn take_instructions(&mut self) -> i64 {
        let mut final_floor = 0i64;

        for (floor_pos, instruction) in self.instructions.chars().enumerate() {
            match instruction {
                '(' => final_floor += 1,
                ')' => {
                    final_floor -= 1;
                    if final_floor.is_negative() && self.basement_pos == 0 {
                        self.basement_pos = floor_pos + 1;
                        println!("First basement position is: {}", self.basement_pos);
                    }
                },
                _ => ()
            }
        }

        println!("Final floor position is: {}", self.basement_pos);
        final_floor
    }
}


#[cfg(test)]
mod test {
    use crate::utils::util::parse_input;
    use super::Building;

    #[test]
    fn demo() {
        let mut building = Building::with_file_name("src/day_1/demo.txt");
        let val = building.take_instructions();
        assert_eq!(val, -3);
    }

    #[test]
    fn actual() {
        let mut building = Building::with_file_name("src/day_1/actual.txt");
        let val = building.take_instructions();
        println!("{}", val);
    }
}
