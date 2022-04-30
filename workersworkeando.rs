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



for o in 5..6 {
    worker(o).move_right();
info(`pos ${worker(o).x}, ${worker(o).y}`);
     if worker(o).x == 36{
        worker(o).move_up();
}
    if worker(o).y == 39{
        worker(o).move_right();
}
    if worker(o).x == 39{
        worker(o).move_down();
}
    if worker(o).y == 20{
        worker(o).move_left();
}
//    if worker(o).x == 38{
//        worker(o).move_up();
//}

}






// worker(1).x or worker(1).y
// worker(1).color

// x horizontal
// y vertical