#[derive(Debug, Default)]
struct Submarine {
    position: i32,
    depth: i32,
}

impl Submarine {
    fn doit(&mut self, task: &str) {
        let (word, num_word) = match task.split_once(" ") {
            Some(data) => data,
            None => panic!("як пахнет то")
        };

        let num: i32 = num_word.parse().unwrap();

        match word {
            "forward" => self.position += num,
            "up" => self.depth -= num,
            "down" => self.depth += num,
            _ =>  panic!("як пахнет то")
        };

    }
}

#[derive(Debug, Default)]
struct SubmarineWithAim {
    position: i32,
    depth: i32,
    aim: i32
}

impl SubmarineWithAim {
    fn doit(&mut self, task: &str) {
        let (word, num_word) = match task.split_once(" ") {
            Some(data) => data,
            None => panic!("як пахнет то")
        };

        let num: i32 = num_word.parse().unwrap();

        match word {
            "forward" => {
                self.position += num;
                self.depth += self.aim * num
            },
            "up" => self.aim -= num,
            "down" => self.aim += num,
            _ =>  panic!("як пахнет то")
        };

    }
}


fn part1(input: &str) -> i32 {
    let mut submarine = Submarine::default();

    input
        .lines()
        .for_each(|task| submarine.doit(task));

    return submarine.position * submarine.depth
}

fn part2(input: &str) -> i32 {
    let mut aimed_submarine = SubmarineWithAim::default();

    input
        .lines()
        .for_each(|task| aimed_submarine.doit(task));

    return aimed_submarine.position * aimed_submarine.depth
}


fn main() {
    let input = include_str!("day2.txt");
    println!("{:?}", part1(input));
    println!("{:?}", part2(input));
}