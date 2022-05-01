fn dist(ax, ay, x, y) {
    (ax - x).abs() + (ay - y).abs();
}

for w in 0..8 {
    let closest_empty_x = 40 - worker(w).x;
    let closest_empty_y = 40 - worker(w).y;

    for x in 0..40 {
        for y in 0..40 {
            if dist(closest_empty_x, closest_empty_y, worker(w).x, worker(w).y) < dist(x, y, worker(w).x, worker(w).y) {
                closest_empty_x = x;
                closest_empty_y = y;
             }
        }
    }

    let px = closest_empty_x;
    let py = closest_empty_y;
    let dx = (worker(w).x - px).abs();
    let dy = (worker(w).y - py).abs();
    if dx > dy {
        if worker(w).x > px { worker(w).move_left();  }
        else                { worker(w).move_right(); }
    } else {
        if worker(w).y > py { worker(w).move_down(); }
        else                { worker(w).move_up();   }
    }
}
