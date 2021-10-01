pub mod cubes;

pub mod actions {
    use seahorse::Context;
    use crate::cubes;

    pub fn pattern_action(c: &Context) {
        let pattern_name = c.args.first().unwrap();

        if let Ok(cube_size) = c.int_flag("size") {
            match cube_size {
                3 => cubes::three_by_three::display_pattern(pattern_name),
                4 => cubes::four_by_four::display_pattern(pattern_name),
                5 => cubes::five_by_five::display_pattern(pattern_name),
                6 => cubes::six_by_six::display_pattern(pattern_name),
                _x => cubes::three_by_three::display_pattern(pattern_name),
            };
        } else {
            cubes::three_by_three::display_pattern(pattern_name);
        }
    }
}
