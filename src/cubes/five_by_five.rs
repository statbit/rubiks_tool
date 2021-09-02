pub fn display_pattern(name: &str) {
    let pattern : &str = match name {
        "list" => "cube-in-cube",
        "cube-in-cube" =>  "F U' B L U' F2 U2 F U F' U2 D' B D L2 B2 U Fw Uw' Bw Lw Uw' Fw2 Uw2 Fw Uw Fw' Uw2 Dw' Bw Dw Lw2 Bw2 Uw",
        _x => "unknown pattern",
    };
    println!("{}", pattern);
}
