pub fn display_pattern(name: &str) {
    let pattern : &str = match name {
        "list" => "cube-in-cube\npeak\nstripes\nchecker-dot" ,
        "cube-in-cube" => "B' 2R2 2L2 U2 2R2 2L2 B F2 R U' R U R2 U R2 F' U F' u l u' f2 d r' u f d2 r2",
        "cic" => "B' 2R2 2L2 U2 2R2 2L2 B F2 R U' R U R2 U R2 F' U F' u l u' f2 d r' u f d2 r2",
        "peak" => "2B2 2D2 l2 U F2 L2 D' L' D L' F U' F l2 2D2 2B2",
        "stripes" => "d 2-3Rw2 d2 2-3Fw2 d 2-3Rw2 b2 r2 B2 2F2",
        "checker-dot" => "2-3Dw' 2-3Rw' 2U' 2D2 2F' 2D' 2U' 2B 2U 2-3Fw 2-3Rw",
        _x => "unknown pattern",
    };

    println!("{}", pattern);
}
