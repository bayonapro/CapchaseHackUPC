for w in map.workers {
    // Logic to check worker placement
}

for x in 0..40 {
    for y in 0..40 {
        if map[x][y] == Tile::EMPTY {
            // more logic
        }
        // other logic
    }
}

for w in 0..8 {
    info(`pos ${worker(w).x}, ${worker(w).y}`);
    let r = (rand() % 4).abs();
    switch r {
        0 => worker(w).move_up(),
        1 => worker(w).move_down(),
        2 => worker(w).move_right(),
        3 => worker(w).move_left(),
    }
}

let quadrante = "capchase_putos_amos";

for q in 0..8 {
    if worker(q).x >=20 && worker(q).y >=20{
        info(`pos ${worker(q).x}, ${worker(q).y}`);
        quadrante = "topright";
    } else if worker(q).x <=20 && worker(q).y <=20{
        quadrante = "botleft";
    } else if worker(q).x >=20 && worker(q).y <=20{
        quadrante = "topleft";
    } else if worker(q).x >=20 && worker(q).y <=20{
        quadrante = "botright";
    }
}
//---------------------------------------------------------------
// TOP RIGHT MOVEMENTS
for w in 0..6 {
    if quadrante == "topright" {
        let qtr = (rand() % 10).abs();
        switch qtr {
            0 => worker(w).move_down(),
            1 => worker(w).move_down(),
            2 => worker(w).move_left(),
            3 => worker(w).move_left(),
            4 => worker(w).move_up(),
        }
        if worker(w).x <=20 && worker(w).y <=20 {
            let qtr1 = (rand() % 4).abs();
             switch qtr1 {
                0 => worker(w).move_down(),
                1 => worker(w).move_up(),
                2 => worker(w).move_left(),
                3 => worker(w).move_right(),
            }
        }    
    }
}
// TOP RIGHT LURKERS
for e in 6..8 {
    if quadrante == "topright" {
        let qtl = (rand() % 12).abs();
        switch qtl {
            0 => worker(e).move_down(),
            1 => worker(e).move_left(),
            2 => worker(e).move_up(),
            3 => worker(e).move_right(),
            4 => worker(e).move_right(),
            5 => worker(e).move_up(),
        }
    }
}

//----------------------------------------------------------------
// TOP RIGHT MOVEMENTS
for e in 0..8 {
    if quadrante == "topleft" {
        let qtl = (rand() % 12).abs();
        switch qtl {
            0 => worker(e).move_down(),
            1 => worker(e).move_down(),
            2 => worker(e).move_right(),
            3 => worker(e).move_right(),
            4 => worker(e).move_up(),
        }
        if worker(e).x >=20 && worker(e).y <=20 {
            let qtr1 = (rand() % 4).abs();
             switch qtr1 {
                0 => worker(e).move_down(),
                1 => worker(e).move_up(),
                2 => worker(e).move_left(),
                3 => worker(e).move_right(),
            }
        }
    }
}
// TOP RIGHT LURKERS
for e in 6..8 {
    if quadrante == "topleft" {
        let qtl = (rand() % 12).abs();
        switch qtl {
            0 => worker(e).move_down(),
            1 => worker(e).move_left(),
            2 => worker(e).move_up(),
            3 => worker(e).move_left(),
            4 => worker(e).move_left(),
            5 => worker(e).move_up(),
        }
    }
}
//----------------------------------------------------------------
// BOTTOM LEFT MOVEMENTS
for r in 0..6 {
    if quadrante == "botleft" {
        let qbl = (rand() % 12).abs();
        switch qbl {
            0 => worker(r).move_up(),
            1 => worker(r).move_up(),
            2 => worker(r).move_right(),
            3 => worker(r).move_right(),
            4 => worker(r).move_down(),
        }
        if worker(r).x >=20 && worker(r).y >=20 {
            let qtr1 = (rand() % 4).abs();
             switch qtr1 {
                0 => worker(r).move_down(),
                1 => worker(r).move_up(),
                2 => worker(r).move_left(),
                3 => worker(r).move_right(),
            }
        }
    }
}
// BOTTOM LEFT LURKERS
for e in 6..8 {
    if quadrante == "botleft" {
        let qtl = (rand() % 12).abs();
        switch qtl {
            0 => worker(e).move_down(),
            1 => worker(e).move_left(),
            2 => worker(e).move_up(),
            3 => worker(e).move_left(),
            4 => worker(e).move_left(),
            5 => worker(e).move_down(),
        }
    }
}

//----------------------------------------------------------------
// BOTTOM RIGHT MOVEMENTS
for t in 0..6 {
    if quadrante == "botright" {
        let qbr = (rand() % 12).abs();
        switch qbr {
            0 => worker(t).move_up(),
            1 => worker(t).move_up(),
            2 => worker(t).move_right(),
            3 => worker(t).move_right(),
            4 => worker(t).move_down(),
        }
        if worker(t).x >=20 && worker(t).y >=20 {
            let qtr1 = (rand() % 4).abs();
             switch qtr1 {
                0 => worker(t).move_down(),
                1 => worker(t).move_up(),
                2 => worker(t).move_left(),
                3 => worker(t).move_right(),
            }
        }
    }
}
// BOTTOM RIGHT LURKERS
for e in 6..8 {
    if quadrante == "botright" {
        let qtl = (rand() % 12).abs();
        switch qtl {
            0 => worker(e).move_down(),
            1 => worker(e).move_left(),
            2 => worker(e).move_up(),
            3 => worker(e).move_right(),
            4 => worker(e).move_righ(),
            5 => worker(e).move_up(),
        }
    }
}



// worker(1).x or worker(1).y
// worker(1).color

// x horizontal
// y vertical