use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash)]
struct Point {
    x: u32,
    y: u32
}


pub struct Path {
    points: HashSet<Point>
}

impl Path {

    pub fn add_point(&mut self, x: u32, y: u32) {
        self.points.insert(Point {x, y});
    }

    pub fn point_exists(&self, x: &u32, y: &u32) -> bool {
        self.points.contains(&Point {x: x.clone(), y: y.clone()})
    }
}

enum Movement {
    Left { steps: u32 },
    Right { steps: u32 },
    Up { steps: u32 },
    Down { steps: u32 }
}

impl Movement {
    fn parse(str: &str) -> Movement {
        let movement: char = str.chars().next().unwrap();
        let steps = str[1 ..].parse::<u32>().unwrap();
        match movement {
            'L' => {
                Movement::Left {steps }
            },
            'R' => {
                Movement::Right {steps }
            },
            'U' => {
                Movement::Up {steps }
            },
            'D' => {
                Movement::Down {steps }
            },
            _ => {
                panic!("Can't parse movement from string {}", str);
            }
        }
    }


}
