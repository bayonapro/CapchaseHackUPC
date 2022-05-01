if "first_iteration" in memory == false {
    memory.tick_count = 0;
    memory.state = [ 0, 0, 0, 0, 0, 0, 0, 0 ];
    memory.achieved_goal_position = [ 0, 0, 0, 0, 0, 0, 0, 0 ];
    memory.moves_remaining = [ 4, 4, 4, 4, 4, 4, 4, 4 ];

    let x = worker(0).x;
    let y = worker(0).y;

    memory.cuadrant = 0;
    if x >= 20 {
        if y >= 20 {
            memory.cuadrant = 2;
        } else {
            memory.cuadrant = 3;
        }
    } else {
        if y >= 20 {
            memory.cuadrant = 1;
        }
    }
}

fn moveTowardsPosition(w, px, py) {
    let x = worker(w).x;
    let y = worker(w).y;
    let dx = (px - x).abs();
    let dy = (py - y).abs();
    if dx > dy {
        if x > px {
            worker(w).move_left();
        } else {
            worker(w).move_right();
        }
    } else {
        if y > py {
            worker(w).move_down();
        } else {
            worker(w).move_up();
        }
    }
}

fn computeInitialPosition(w, cuadrant) {
    let px = 0;
    let py = 0;
    switch cuadrant {
        0 => {
            px = w * 4;
            py = 0;
        },
        1 => {
            px = w * 4;
            py = 36;
        },
        2 => {
            px = w * 4;
            py = 36;
        },
        3 => {
            px = w * 4;
            py = 0;
        },
    }
    moveTowardsPosition(w, px, py);
}

for w in 0..8 {
    let x = worker(w).x;
    let y = worker(w).y;

    switch memory.state[w] {
        0 => {
            computeInitialPosition(w, memory.cuadrant);
        },
        1 => {
            worker(w).move_up();
            memory.moves_remaining[w] -= 1;
            if memory.moves_remaining[w] == 0 {
                memory.moves_remaining[w] = 4;
                memory.state = 2;
            }
        },
        2 => {
            worker(w).move_right();
            memory.moves_remaining[w] -= 1;
            if memory.moves_remaining[w] == 0 {
                memory.moves_remaining[w] = 4;
                memory.state = 3;
            }
        },
        3 => {
            worker(w).move_down();
            memory.moves_remaining[w] -= 1;
            if memory.moves_remaining[w] == 0 {
                memory.moves_remaining[w] = 4;
                memory.state = 4;
            }
        },
        4 => {
            worker(w).move_left();
            memory.moves_remaining[w] -= 1;
            if memory.moves_remaining[w] == 0 {
                memory.moves_remaining[w] = 4;
                memory.state = 1;
            }
        },
    }
}

memory.tick_count += 1;
