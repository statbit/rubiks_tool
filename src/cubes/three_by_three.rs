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
        "list" => "cube-in-cube\n\
                   cube-in-cube-in-cube\n\
                   cross\n\
                   spots\n\
                   stripes\n",
        "cube-in-cube" => "F L F U' R U F2 L2 U' L' B D' B' L2 U",
        "cic" => "F L F U' R U F2 L2 U' L' B D' B' L2 U",
        "cube-in-cube-in-cube" => "U' L' U' F' R2 B' R F U B2 U B' L U' F U R F'",
        "ccc" => "U' L' U' F' R2 B' R F U B2 U B' L U' F U R F'",
        "cross" => "F' B L' E2 L2 S2 M' L' E' B' E2 F2 M2 F'",
        "spots" => "E S E' S'",
        "stripes" => "F U F R L2 B D' R D2 L D' B R2 L F U F",
        _x => "unknown pattern",
    };

    println!("{}", pattern);
}
