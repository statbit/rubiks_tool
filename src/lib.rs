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
                "list" => "cube-in-cube" ,
                "cube-in-cube" => "F L F U' R U F2 L2 U' L' B D' B' L2 U",
                "spots" => "E S E' S'",
                _x => "unknown pattern",
            };

            println!("{}", pattern);
        }
    }

    pub mod actions {
        use seahorse::Context;
        use super::three_by_three;

        pub fn pattern_action(c: &Context) {
            let pattern_name = c.args.first();
            three_by_three::display_pattern(pattern_name.unwrap());
        }

    }
}
