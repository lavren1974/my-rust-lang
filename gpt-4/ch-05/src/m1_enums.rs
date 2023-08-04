#[derive(Debug)]
enum CarColour {
    Blue,
    Red,
    Green,
    Silver,
}

#[derive(Debug)]
enum GivenResult<T, E> {
    Ok(T),
    Err(E),
}

fn create_car_colour_blue() -> CarColour {
    let my_car_colour: CarColour = CarColour::Blue;
    my_car_colour
}

fn check_under_five(num_check: u8) -> GivenResult<u8, String> {
    if num_check < 5 {
        GivenResult::Ok(num_check)
    } else {
        GivenResult::Err("Not num_check under five".to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tests_enums() {
        //dbg!("Hello!");
        let car_color = create_car_colour_blue();
        dbg!(&car_color);

        let check_under_five_res=check_under_five(8);
        dbg!(&check_under_five_res);
    }
}
