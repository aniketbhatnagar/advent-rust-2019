#[path = "../../day1/mod.rs"]
mod day1;
#[path = "../../files/mod.rs"]
mod files;

/// Solution to https://adventofcode.com/2019/day/1.
fn main() {

    let file = files::TextFile {
        path : "src/bin/day1/input.txt".to_string()
    };

    // Part 1
    let total_fuel = file
        .read_lines_i64()
        .fold(0,
              |total_fuel, mass|
                  total_fuel + day1::calc_fuel(&mass));


    // prints 3262991
    println!("{}", total_fuel);

    // Part 2
    let total_fuel_recursive = file
        .read_lines_i64()
        .fold(0,
              |total_fuel, mass|
                  total_fuel + day1::calc_fuel_recursive(&mass));


    // prints 4891620
    println!("{}", total_fuel_recursive);
}
