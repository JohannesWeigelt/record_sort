use crate::data::reader::parallel_reader::area::Area;

pub fn split(areas: u8, input: u64) -> Result<Vec<Area>, String> {
    if areas == 0 || input == 0 {
        return Err(String::from("areas or input == 0"));
    } else if input < areas as u64 {
        return Err(String::from("input less then areas"));
    }

    let span = input / areas as u64;
    let mut result: Vec<Area> = Vec::new();
    let mut i = 0;

    loop {
        if i < (areas as u64) - 1 {
            result.push(Area::new(i * span, ((i + 1) * span) - 1));
        } else {
            result.push(Area::new(i * span, input - 1));
            break;
        }

        i = i + 1;
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use crate::data::reader::parallel_reader::area_splitter::split;

    #[test]
    fn success_same() {
        let input = 8;
        let area_count = 8;

        let areas = split(area_count, input).unwrap();

        assert_eq!(area_count as usize, areas.len())
    }

    #[test]
    fn success_divider_match() {
        let input = 8;
        let area_count = 2;

        let areas = split(area_count, input).unwrap();

        assert_eq!(area_count as usize, areas.len())
    }

    #[test]
    fn success_smaller_last_area() {
        let input = 8;
        let area_count = 3;

        let areas = split(area_count, input).unwrap();

        assert_eq!(area_count as usize, areas.len())
    }

    #[test]
    fn success_bigger_last_area() {
        let input = 11;
        let area_count = 3;

        let areas = split(area_count, input).unwrap();

        assert_eq!(area_count as usize, areas.len())
    }

    #[test]
    #[should_panic]
    fn failure_input_zero() {
        let input = 0;
        let area_count = 3;

        split(area_count, input).unwrap();
    }

    #[test]
    #[should_panic]
    fn failure_area_count_zero() {
        let input = 3;
        let area_count = 0;

        split(area_count, input).unwrap();
    }

    #[test]
    #[should_panic]
    fn failure_all_params_zero() {
        let input = 0;
        let area_count = 0;

        split(area_count, input).unwrap();
    }

    #[test]
    #[should_panic]
    fn failure_input_smaller_area_count() {
        let input = 3;
        let area_count = 4;

        split(area_count, input).unwrap();
    }
}