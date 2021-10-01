pub fn display_pattern(name: &str) {
    let pattern : &str = match name {
        "list" => "smile\nfrown\nbar\nyin-yang\ncube-in-cube\nrings",
        "smile" => "2-5Fw2 3-4Rw2 3-4Fw2 2U 2-5Fw2 2U2 2-5Fw2 3U 2R2 2L2 3U2 2R2 2L2 2-3Uw 3-4Fw2 3-4Rw2 2-5Fw2",
        "frown" => "3-4Fw2 2R2 2L2 2U2 2F2 2B2 2R2 2L2 2U2 3D 3-4Fw2 3D2 3-4Fw2 3D 2-5Fw2",
        "bar" => "R L B2 U2 3U' 2D 2-5Fw 2D' 2B 3D 3B 3U 3F' 2U' 2F' 3D' 2U U2 B2 L' R'",
        "yin-yang" => "2D 1-3Bw2 3-4Lw2 1-3Fw2 2D 1-3Fw2 3R2 1-3Fw2 2U' 2D' 1-3Dw 2-5Rw2 1-3Uw2 L2' R2 1-3Uw 1-3Rw2 B2 1-3Rw2 1-3Bw2 2-4Rw2 1-3Bw2",
        "cube-in-cube" => "1-2Fw2 1-2Rw2 1-2Uw' 1-2Fw' 1-4Dw 1-2Bw' 1-2Uw2 1-2Bw 1-2Uw' 1-2Rw' 3R2 1-2Bw2 3R2 1-3Fw2 3L2 3B2",
        "rings" => "2-5Rw 2-4Bw' L2 2F 2B L2 2B' 2-5Dw' 2F' L2 2F 2B L2 2B' 2-5Dw2 3-4Rw U' 3-4Rw' 2-5Dw' 3-4Rw U 2-5Rw' 3-4Fw' 3-4Rw' 3-4Fw 2-5Fw 2-4Lw2 2F2 3-4Rw2 2-4Bw2 2L2",
        _x => "unknown pattern"
    };
    println!("{}", pattern)
}

