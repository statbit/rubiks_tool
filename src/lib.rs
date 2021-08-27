pub mod cubes {

    pub mod three_by_three {

        pub fn reverse(mut move_list : Vec<&str>) {
            move_list.reverse();
            for m in move_list {
                let m = m.to_uppercase();
                reverse_move(&m);
            }
        }

        pub fn reverse_move(m : &str) {
            let m = m.to_uppercase();
            if m.find('2').is_some() {
                print!("{} ", m);
                return;
            }
            if m.find('\'').is_some() {
                print!("{} ", m.replace("'", ""));
                return;
            }
            print!("{}' ", m);
        }

        pub fn display_pattern(name: &str) {
            let pattern : &str = match name {
                "list" => "cube-in-cube\nspots" ,
                "cube-in-cube" => "F L F U' R U F2 L2 U' L' B D' B' L2 U",
                "spots" => "E S E' S'",
                _x => "unknown pattern",
            };

            println!("{}", pattern);
        }
    }

    pub mod four_by_four {
        pub fn display_pattern(name: &str) {
            let pattern : &str = match name {
                "list" => "cube-in-cube\npeak" ,
                "cube-in-cube" => "B' 2R2 2L2 U2 2R2 2L2 B F2 R U' R U R2 U R2 F' U F' u l u' f2 d r' u f d2 r2",
                "peak" => "2B2 2D2 l2 U F2 L2 D' L' D L' F U' F l2 2D2 2B2",
                _x => "unknown pattern",
            };

            println!("{}", pattern);
        }
    }

    pub mod actions {
        use seahorse::Context;
        use super::three_by_three;
        use super::four_by_four;

        pub fn pattern_action(c: &Context) {

            let pattern_name = c.args.first();
            let upattern_name = pattern_name.unwrap();
            match c.int_flag("size") {
                Ok(cube_size) => {
                    match cube_size {
                        3 => three_by_three::display_pattern(upattern_name),
                        4 => four_by_four::display_pattern(upattern_name),
                        _x => three_by_three::display_pattern(upattern_name),
                    };
                },
                Err(e) => panic!("bad size argument, {}", e )
            }
        }
    }
}
