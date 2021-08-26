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
}
