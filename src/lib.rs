pub mod cubes {

    pub mod three_by_three {

        pub fn reverse(mut move_list : Vec<&str>) {
            move_list.reverse();
            for m in move_list {
                let m = m.to_uppercase();
                match m.as_str() {
                    "R" => print!("R' "),
                    "R'" => print!("R "),

                    "L" => print!("L' "),
                    "L'" => print!("L "),

                    "U" => print!("U' "),
                    "U'" => print!("U "),

                    "B" => print!("B' "),
                    "B'" => print!("B "),

                    "D" => print!("D' "),
                    "D'" => print!("D "),

                    "F" => print!("F' "),
                    "F'" => print!("F "),

                    x => print!("{} ", x),

                }
            }
            println!("");
        }

        pub fn display_pattern(name: &str) {
            match name {
                "list" => {
                    println!("cube-in-cube");
                },
                "cube-in-cube" => print!("F L F U' R U F2 L2 U' L' B D' B' L2 U"),
                x => print!("unknown pattern: {}", x),
            }

            println!("");
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
