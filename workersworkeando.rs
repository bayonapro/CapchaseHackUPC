if "first_iteration" in memory == false {
    memory.first_iteration = true;
    memory.direction = "pito";
    memory.horizontal = "cacahuete";
}

// pito -> going down
// polla -> going up
//
// cacahuete -> going left
// zapatilla -> going right

for o in 0..5 {
    worker(o).move_left();
}

for o in 6..8 {
    worker(o).move_left();
}

for o in 5..6 {
    worker(o).move_right();
    info(`pos ${worker(o).x}, ${worker(o).y}`);

    if worker(o).x == 39 && worker(o).y == 39 {
        memory.first_iteration = false;
    }

    if memory.first_iteration == true {
        if worker(o).x == 36 {
            worker(o).move_up();
        }
        if worker(o).y == 39 {
            worker(o).move_right();
        }
    } else {
        switch memory.direction {
            "pito" => {
                if worker(o).y == 20 {
                    switch memory.horizontal {
                        "cacahuete" => {
                            if worker(o).x == 20 {
                                worker(o).move_right();
                                memory.horizontal = "zapatilla";
                            } else {
                                worker(o).move_left();
                            }
                        },
                        "zapatilla" => {
                            if worker(o).x == 39 {
                                worker(o).move_left();
                                memory.horizontal = "cacahuete";
                            } else {
                                worker(o).move_right();
                            }
                        },
                    }
                    memory.direction = "polla";
                } else {
                    worker(o).move_down();
                }
            },
            "polla" => {
                if worker(o).y == 39 {
                    switch memory.horizontal {
                        "cacahuete" => {
                            if worker(o).x == 20 {
                                worker(o).move_right();
                                memory.horizontal = "zapatilla";
                            } else {
                                worker(o).move_left();
                            }
                        },
                        "zapatilla" => {
                            if worker(o).x == 39 {
                                worker(o).move_left();
                                memory.horizontal = "cacahuete";
                            } else {
                                worker(o).move_right();
                            }
                        },
                    }
                    memory.direction = "pito";
                } else {
                    worker(o).move_up();
                }
            },
        }
    }
}

