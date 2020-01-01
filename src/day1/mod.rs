use std::cmp;

/// Calculates fuel needed to carry given mass.
pub fn calc_fuel(mass: &i64) -> i64 {
    cmp::max(0, mass / 3 - 2)
}

/// Calculates fuel needed to carry given mass + any fuel.
pub fn calc_fuel_recursive(mass: &i64) -> i64 {
    if mass <= &0i64 {
        0
    } else {
        let fuel = calc_fuel(mass);
        fuel + calc_fuel_recursive(&fuel)
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn mass_negative() {
        assert_eq!(super::calc_fuel(&-1), 0);
    }

    #[test]
    fn mass_0() {
        assert_eq!(super::calc_fuel(&0), 0);
    }

    #[test]
    fn mass_1() {
        assert_eq!(super::calc_fuel(&1), 0);
    }

    #[test]
    fn mass_12() {
        assert_eq!(super::calc_fuel(&12), 2);
    }

    #[test]
    fn mass_14() {
        assert_eq!(super::calc_fuel(&14), 2);
    }

    #[test]
    fn mass_1969() {
        assert_eq!(super::calc_fuel(&1969), 654);
    }

    #[test]
    fn mass_100756() {
        assert_eq!(super::calc_fuel(&100756), 33583);
    }

    #[test]
    fn mass_recursive_0() {
        assert_eq!(super::calc_fuel_recursive(&0), 0);
    }

    #[test]
    fn mass_recursive_1() {
        assert_eq!(super::calc_fuel_recursive(&1), 0);
    }

    #[test]
    fn mass_recursive_14() {
        assert_eq!(super::calc_fuel_recursive(&2), 0);
    }

    #[test]
    fn mass_recursive_1969() {
        assert_eq!(super::calc_fuel_recursive(&1969), 966);
    }

    #[test]
    fn mass_recursive_50346() {
        assert_eq!(super::calc_fuel_recursive(&100756), 50346);
    }
}

