// goal 0 : mid -> ( 1,  1)
// goal 1 : mid -> ( 1, 48)
// goal 2 : mid -> (48,  1)
// goal 3 : mid -> (48, 48)
//
// runners:
// 247
// 1 6
// 035
//
// let w0x = [  0,  0, 47, 47 ];
// let w0y = [  0, 47,  0, 47 ];

// let w1x = [  0,  0, 47, 47 ];
// let w1y = [  1, 48,  1, 48 ];

// let w2x = [  0,  0, 47, 47 ];
// let w2y = [  2, 49,  2, 49 ];

// let w3x = [  1,  1, 48, 48 ];
// let w3y = [  0, 47,  0, 47 ];

// let w4x = [  1,  1, 48, 48 ];
// let w4y = [  2, 49,  2, 49 ];

// let w5x = [  2,  2, 49, 49 ];
// let w5y = [  0, 47,  0, 47 ];

// let w6x = [  2,  2, 49, 49 ];
// let w6y = [  0, 48,  0, 48 ];

// let w7x = [  2,  2, 49, 49 ];
// let w7y = [  2, 49,  2, 49 ];

let wx = [  [  0,  0, 47, 47 ],
            [  0,  0, 47, 47 ],
            [  0,  0, 47, 47 ],
            [  1,  1, 48, 48 ],
            [  1,  1, 48, 48 ],
            [  2,  2, 49, 49 ],
            [  2,  2, 49, 49 ],
            [  2,  2, 49, 49 ]];

let wy = [  [  0, 47,  0, 47 ],
            [  1, 48,  1, 48 ],
            [  2, 49,  2, 49 ],
            [  0, 47,  0, 47 ],
            [  2, 49,  2, 49 ],
            [  0, 47,  0, 47 ],
            [  0, 48,  0, 48 ],
            [  2, 49,  2, 49 ]];

for w in 0..8 {
    let r = (rand() % 100).abs();
    
    let qx = worker(w).x;
    let qy = worker(w).y;

    let cuadrante = 0;
    if qx >= 25 {
        if qy >= 25 {
            cuadrante = 2;
        } else {
            cuadrante = 3;
        }
    } else {
        if qy >= 25 {
            cuadrante = 1;
        } else {
            cuadrante = 0;
        }
    }

    let current_goal = cuadrante;
    if r > 10 && r <= 35 {
        // cuadrante vecino horizontal
        switch cuadrante {
            0 => current_goal = 3,
            1 => current_goal = 2,
            2 => current_goal = 1,
            3 => current_goal = 0,
        }
    } else if r <= 60 {
        // cuadrante vecino vertical
        switch cuadrante {
            0 => current_goal = 1,
            1 => current_goal = 0,
            2 => current_goal = 3,
            3 => current_goal = 2,
        }
    } else {
        // cuadrante contrario
        switch cuadrante {
            0 => current_goal = 2,
            1 => current_goal = 3,
            2 => current_goal = 0,
            3 => current_goal = 1,
        }
    }

    let x = wx[w][current_goal];
    let y = wy[w][current_goal];

    info(`worker ${w} moving towards ${current_goal}`);

    if worker(w).x == x {
        if worker(w).y < y {
            worker(w).move_up();
        } else {
            worker(w).move_down();
        }
    } else if worker(w).y == y {
        if worker(w).x < x {
            worker(w).move_right();
        } else {
            worker(w).move_left();
        }
    } else {
        let dx = abs(worker(w).x - x);
        let dy = abs(worker(w).y - y);
        if dx > dy {
            if worker(w).x > x {
                worker(w).move_left();
            } else {
                worker(w).move_right();
            }
        } else {
            if worker(w).y > y {
                worker(w).move_down();
            } else {
                worker(w).move_up();
            }
        }
    }
}
