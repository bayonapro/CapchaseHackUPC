for w in map.workers {
    // Logic to check worker placement
}

for x in 0..50 {
    for y in 0..50 {
        if map[x][y] == Tile::EMPTY {
            // more logic
        }
        // other logic
    }
}

//for w in 0..1 {
//    let r = (rand() % 4).abs();
//    switch r {
//        0 => worker(w).move_up(),
//        1 => worker(w).move_down(),
//        2 => worker(w).move_right(),
//        3 => worker(w).move_left(),
//    }
for z in 0..8 {

    worker(z).move_down();
    info(`pos ${worker(0).x}, ${worker(0).y}`);
    if worker(z).y == 10{
        worker(z).move_left();
    if worker(z).x == 5{
        worker(z).move_up();
}

    }
  }


