if "first_iteration" in memory == false {
    memory.first_iteration = [ true, true, true, true, true, true, true, true ];
    memory.direction = [ "d", "d", "d", "d", "d", "d", "d", "d" ];
    memory.horizontal = [ "l", "l", "l", "l", "l", "l", "l", "l" ];
}

for o in 0..8 {
    worker(o).move_right();
    info(`worker ${o} pos ${worker(o).x}, ${worker(o).y}`);

    if worker(o).x == 39 && worker(o).y == 39 {
        memory.first_iteration[o] = false;
    }

    if memory.first_iteration[o] == true {
        // let dx = (worker(o).x - 39).abs();
        // let dy = (worker(o).y - 39).abs();
        // if dx > dy {
        //     worker(o).move_right();
        // } else {
        //     worker(o).move_up();
        // }
        if worker(o).y == 39 {
            worker(o).move_right();
        } else {
            worker(o).move_up();
        }
    } else {
        switch memory.direction[o] {
            "d" => {
                if worker(o).y == 20 {
                    switch memory.horizontal[o] {
                        "l" => {
                            if worker(o).x == 20 {
                                worker(o).move_right();
                                memory.horizontal[o] = "r";
                            } else {
                                worker(o).move_left();
                            }
                        },
                        "r" => {
                            if worker(o).x == 39 {
                                worker(o).move_left();
                                memory.horizontal[o] = "left";
                            } else {
                                worker(o).move_right();
                            }
                        },
                    }
                    memory.direction[o] = "u";
                } else {
                    worker(o).move_down();
                }
            },
            "u" => {
                if worker(o).y == 39 {
                    switch memory.horizontal[o] {
                        "l" => {
                            if worker(o).x == 20 {
                                worker(o).move_right();
                                memory.horizontal[o] = "r";
                            } else {
                                worker(o).move_left();
                            }
                        },
                        "r" => {
                            if worker(o).x == 39 {
                                worker(o).move_left();
                                memory.horizontal[o] = "left";
                            } else {
                                worker(o).move_right();
                            }
                        },
                    }
                    memory.direction[o] = "d";
                } else {
                    worker(o).move_up();
                }
            },
        }
    }
}

