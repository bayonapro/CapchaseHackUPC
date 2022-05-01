let quadrante = "capchase_putos_amos";

for q in 0..8 {
    if worker(q).x >=20 && worker(q).y >=20{
        quadrante = "topright";
    } else if worker(q).x <=20 && worker(q).y <=20{
        quadrante = "botleft";
    } else if worker(q).x >=20 && worker(q).y <=20{
        quadrante = "topleft";
    } else if worker(q).x >=20 && worker(q).y <=20{
        quadrante = "botright";
    }
}

for w in 0..8 {
    switch quadrante {
        "topright" => {
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
        },
        "topleft" => {
            let qtl = (rand() % 12).abs();
            switch qtl {
                0 => worker(w).move_down(),
                1 => worker(w).move_down(),
                2 => worker(w).move_right(),
                3 => worker(w).move_right(),
                4 => worker(w).move_up(),
            }
            if worker(w).x >=20 && worker(w).y <=20 {
                let qtr1 = (rand() % 4).abs();
                 switch qtr1 {
                    0 => worker(w).move_down(),
                    1 => worker(w).move_up(),
                    2 => worker(w).move_left(),
                    3 => worker(w).move_right(),
                }
            }
        },
        "botleft" => {
            let qbl = (rand() % 12).abs();
            switch qbl {
                0 => worker(w).move_up(),
                1 => worker(w).move_up(),
                2 => worker(w).move_right(),
                3 => worker(w).move_right(),
                4 => worker(w).move_down(),
            }
            if worker(w).x >=20 && worker(w).y >=20 {
                let qtr1 = (rand() % 4).abs();
                 switch qtr1 {
                    0 => worker(w).move_down(),
                    1 => worker(w).move_up(),
                    2 => worker(w).move_left(),
                    3 => worker(w).move_right(),
                }
            }
        },
        "botright" => {
            let qbr = (rand() % 12).abs();
            switch qbr {
                0 => worker(w).move_up(),
                1 => worker(w).move_up(),
                2 => worker(w).move_right(),
                3 => worker(w).move_right(),
                4 => worker(w).move_down(),
            }
            if worker(w).x >=20 && worker(w).y >=20 {
                let qtr1 = (rand() % 4).abs();
                 switch qtr1 {
                    0 => worker(w).move_down(),
                    1 => worker(w).move_up(),
                    2 => worker(w).move_left(),
                    3 => worker(w).move_right(),
                }
            }
        },
    }
}
