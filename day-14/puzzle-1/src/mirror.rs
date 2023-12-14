pub mod mirror {
    use std::fs;
    use std::ops::Range;

    pub fn parse_input(input: &String) -> Map {
        let lines = fs::read_to_string(input)
            .expect("Could not read file");
        let lines: Vec<&str> = lines.trim().split('\n').collect();

        Map::from(lines.as_slice())
    }

    pub struct Map {
        data: Vec<Vec<Material>>
    }

    impl Map {
        pub fn from(lines: &[&str]) -> Self {
            let d: Vec<Vec<Material>> = lines.iter().map(|&line|
                line.chars().map(|c|
                    Material::from(c)
                ).collect()
            ).collect();

            // instead of tilting to north, we flipped the data and need
            // to tilt to the east
            Map { data: Self::transpose(&d) }
        }

        pub fn tilt(&mut self) {
            self.data.iter_mut().for_each(|line|
                Self::tilt_line(line)
            );
        }

        fn tilt_line(line: &mut Vec<Material>) {
            let mut start = 0usize;

            loop {
                let (next_obstacle, rocks_between) = Self::next_obstacle(line, start);
                Self::tilt_in_range(line, rocks_between, start..next_obstacle);
                if next_obstacle == line.len() {
                    break;
                }
                start = next_obstacle + 1;
            }
        }

        fn tilt_in_range(line: &mut Vec<Material>, rocks: u32, range: Range<usize>) {
            // there is for sure a rusty way of doing the same
            for i in range.start..range.start + rocks as usize {
                line[i] = Material::RoundedRock;
            }

            for i in range.start + rocks as usize .. range.end {
                line[i] = Material::Nothing;
            }
        }

        fn next_obstacle(line: &Vec<Material>, start_at: usize) -> (usize, u32) {
            let mut num_rounded_rock = 0u32;

            for r in start_at..line.len() {
                if *line.get(r).unwrap() == Material::RoundedRock {
                    num_rounded_rock = num_rounded_rock + 1;
                }

                if *line.get(r).unwrap() == Material::CubicRock {
                    return (r, num_rounded_rock);
                }
            }

            // if we didn't found an obstacle, we report the EOL
            (line.len(), num_rounded_rock)
        }

        pub fn weight(&self) -> u32 {
             self.data.iter().map(|line|
                Self::weight_line(line)
            ).sum()
        }

        fn weight_line(line: &Vec<Material>) -> u32 {
            let mut weight = 0u32;

            for (index, item) in line.iter().enumerate() {
                if *item == Material::RoundedRock {
                    weight = weight + (line.len() - index) as u32;
                }
            }

            weight
        }

        fn transpose(data: &Vec<Vec<Material>>) -> Vec<Vec<Material>> {
            let mut transposed_data: Vec<Vec<Material>> = Vec::new();

            // for every column i, fetch every ith element from the sub vectors
            let sub_vector_length = data.first().unwrap().len();

            for i in 0 .. sub_vector_length {
                let ith_data: Vec<Material> = data.iter().map(|v|
                    Material::clone(v.get(i).unwrap())
                ).collect();
                transposed_data.push(ith_data);
            }

            transposed_data
        }
    }


    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    enum Material {
        Nothing,
        RoundedRock,
        CubicRock
    }

    impl Material {
        fn from(c: char) -> Self {
            match c {
                '.' => Self::Nothing,
                'O' => Self::RoundedRock,
                '#' => Self::CubicRock,
                _ => panic!("Unknown material {}", c)
            }
        }
    }
}