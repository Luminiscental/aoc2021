use crate::day::Day;

#[derive(Debug, Clone, Copy)]
pub enum Constraint {
    Equal(usize, i32),
    Greater(i32),
    Lesser(i32),
    Free,
}

impl Constraint {
    fn apply(self, i: usize, model: &mut [i32; 14]) {
        match self {
            Self::Equal(j, add) => model[i] = model[j] + add,
            Self::Greater(min) => model[i] = model[i].max(min),
            Self::Lesser(max) => model[i] = model[i].min(max),
            Self::Free => {}
        }
    }
}

pub struct Day24;

impl<'a> Day<'a> for Day24 {
    type Input = [Constraint; 14];
    type ProcessedInput = Self::Input;

    const DAY: usize = 24;

    fn parse(input: &'a str) -> Self::Input {
        let lines = input.trim().lines().map(str::trim).collect::<Vec<_>>();
        let mut z = Vec::<(usize, i32)>::new();
        let mut constraints = [Constraint::Free; 14];
        for (i, block) in lines[1..].split(|&line| line == lines[0]).enumerate() {
            let mut divs_and_adds = block
                .iter()
                .filter(|line| line.starts_with("div") || line.starts_with("add"));
            let div_z = divs_and_adds.nth(1).unwrap();
            if div_z == &"div z 26" {
                if let Some((j, mut add)) = z.pop() {
                    let add_x = divs_and_adds.next().unwrap();
                    add += add_x[6..].parse::<i32>().unwrap();
                    constraints[i] = Constraint::Equal(j, add);
                    constraints[j] = if add < 0 {
                        Constraint::Greater(1 - add)
                    } else {
                        Constraint::Lesser(9 - add)
                    };
                }
            } else {
                let add_y = divs_and_adds.nth(4).unwrap();
                z.push((i, add_y[6..].parse().unwrap()));
            }
        }
        constraints
    }

    fn solve_part1(constraints: Self::Input) -> (Self::ProcessedInput, String) {
        let mut model = [9; 14];
        for (i, c) in constraints.iter().enumerate() {
            c.apply(i, &mut model);
        }
        (constraints, model.iter().map(i32::to_string).collect())
    }

    fn solve_part2(constraints: Self::ProcessedInput) -> String {
        let mut model = [1; 14];
        for (i, c) in constraints.iter().enumerate() {
            c.apply(i, &mut model);
        }
        model.iter().map(i32::to_string).collect()
    }
}
