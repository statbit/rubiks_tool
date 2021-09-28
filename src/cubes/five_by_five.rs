pub fn display_pattern(name: &str) {
    let pattern : &str = match name {
        "list" => "cube-in-cube\nhearts\npeak",
        "cube-in-cube" =>  "F U' B L U' F2 U2 F U F' U2 D' B D L2 B2 U Fw Uw' Bw Lw Uw' Fw2 Uw2 Fw Uw Fw' Uw2 Dw' Bw Dw Lw2 Bw2 Uw",
        "hearts" => "M2 2-4Rw2 S2 D 2-4Fw2 D2 2-4Fw2 D S2 2-4Rw M2 2D 2-4Rw 2-4Dw2 2-4Rw' 2D 2-4Rw",
        "peak" => "F' 2L2 F R2 2B U' 2B' Dw' 2B U 2B' Dw R2 F' 2L2 F2 R U' M2 U R' U' M2 U F'",
        _x => "unknown pattern",
    };
    println!("{}", pattern);
}
