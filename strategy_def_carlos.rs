if "first_iteration" in memory == false {
    memory.last_move = [ -1, -1, -1, -1, -1, -1, -1, -1 ];
    memory.first_iteration = true;
}

for w in 0..8 {
    if memory.first_iteration == false {
        let r = (rand() % 4).abs();
        switch r {
            0 => worker(w).move_up(),
            1 => worker(w).move_down(),
            2 => worker(w).move_right(),
            3 => worker(w).move_left(),
        }
        memory.last_move[w] = r;
    } else {
        let r = (rand() % 4).abs();
        while r == memory.last_move[w] {
            r = (rand() % 4).abs();
        }

        switch r {
            0 => worker(w).move_up(),
            1 => worker(w).move_down(),
            2 => worker(w).move_right(),
            3 => worker(w).move_left(),
        }
        memory.last_move[w] = r;
    }
}

memory.first_iteration = true;
