// init
if "myColor" in memory != true {
    memory["myColor"] = map[worker(0).x][worker(0).y];
}
let usedPositions = [];
for w in map.workers{
    usedPositions.push([w.x, w.y]);
}
fn MoveToDirection(workerNum, axis) {
    if axis == 0 {
        worker(workerNum).move_left();
        //info(`${workerNum}->LEFT`);
    }
    else if axis == 1 {
        worker(workerNum).move_right();
        //info(`${workerNum}->RIGHT`);
    }
    else if axis == 2 {
        worker(workerNum).move_up();
        //info(`${workerNum}->UP`);
    }
    else if axis == 3 {
        worker(workerNum).move_down();
        //info(`${workerNum}->DOWN`);
    }
}
if "t" in memory{
    memory.t += 1
}
else {
    let x = worker(0).x;
    let y = worker(0).y;
    memory.atacante1stat = 1;
    memory.atacante2stat = 1;
    memory.cuadrante = 0;
    if x < 20 && y < 20 {
        memory.cuadrante = 0;
        memory.atacante1 = [4, "up", "right", "down", "zigzag2"];
        memory.atacante2 = [6, "right", "zigzag2", "right", "down"];
    }
    else if x < 20 && y > 20 {
        memory.cuadrante = 1;
        memory.atacante1 = [6, "right","down", "left", "zigzag7"];
        memory.atacante2 = [3, "down", "zigzag7", "down", "left"];
    }
    else if x > 20 && y < 20 {
        memory.cuadrante = 2;
        memory.atacante1 = [1, "left", "up", "right", "zigzag0"];
        memory.atacante2 = [4, "up", "zigzag0", "up", "right"];
    }
    else if x > 20 && y > 20 {
        memory.cuadrante = 3;
        memory.atacante1 = [3, "down", "left", "up", "zigzag5"];
        memory.atacante2 = [1, "left", "zigzag5", "left", "up"];
    }
    memory.t = 0;
}
fn cuadrante (x, y){
    let cuadrante = 0;
    if x < 20 && y < 20 {
        cuadrante = 0;
    }
    else if x < 20 && y > 20 {
        cuadrante = 1;
    }
    else if x > 20 && y < 20 {
        cuadrante = 2;
    }
    else if x > 20 && y > 20 {
        cuadrante = 3;
    }
    return cuadrante;
}
fn outside_range_improved(x,y){
    return x == 5 || x == 35|| y == 5 || y == 35;
}
info(`TICK ${memory.t}`);
for w in 0..8 {
    if w == memory.atacante1[0]{
        if outside_range_improved(worker(w).x, worker(w).y){
            memory.atacante1stat += 1;
            if memory.atacante1stat == 5 {memory.atacante1stat = 2}
            info(`jugador ${w} coords ${worker(w).x}:${worker(w).y} estado ${memory.atacante1stat} toca limite`);
            let l = (rand() % 2).abs();
            switch cuadrante(worker(w).x, worker(w).y) {
                3 => if l == 0 {worker(w).move_down()} else {worker(w).move_left()},
                2 => if l == 0 {worker(w).move_up()} else {worker(w).move_left()},
                1 => if l == 0 {worker(w).move_down()} else {worker(w).move_right()},
                0 => if l == 0 {worker(w).move_up()} else {worker(w).move_right()},
            }
        }
        else {
            switch memory.atacante1[memory.atacante1stat]{
                "up" => worker(w).move_up(),
                "down" => worker(w).move_down(),
                "right" => worker(w).move_right(),
                "left" => worker(w).move_left(),
                "zigzag0" => if memory.t % 2 == 0 {worker(w).move_down()} else {worker(w).move_left()},
                "zigzag2" => if memory.t % 2 == 0 {worker(w).move_left()} else {worker(w).move_up()},
                "zigzag5" => if memory.t % 2 == 0  {worker(w).move_down()} else {worker(w).move_right()},
                "zigzag7" => if memory.t % 2 == 0{worker(w).move_up()} else {worker(w).move_right()},
            }
        }
    }
    else if w == memory.atacante2[0]{
        if outside_range_improved(worker(w).x, worker(w).y){
            memory.atacante2stat += 1;
            if memory.atacante2stat == 5 {memory.atacante2stat = 2}
            info(`jugador ${w} coords ${worker(w).x}:${worker(w).y} toca limite`);
            let l = (rand() % 2).abs();
            switch cuadrante(worker(w).x, worker(w).y) {
                3 => if l == 0 {worker(w).move_down()} else {worker(w).move_left()},
                2 => if l == 0 {worker(w).move_up()} else {worker(w).move_left()},
                1 => if l == 0 {worker(w).move_down()} else {worker(w).move_right()},
                0 => if l == 0 {worker(w).move_up()} else {worker(w).move_right()},
            }
        }
        else {
            switch memory.atacante2[memory.atacante2stat]{
                "up" => worker(w).move_up(),
                "down" => worker(w).move_down(),
                "right" => worker(w).move_right(),
                "left" => worker(w).move_left(),
                "zigzag0" => if memory.t % 2 == 0 {worker(w).move_down()} else {worker(w).move_left()},
                "zigzag2" => if memory.t % 2 == 0 {worker(w).move_left()} else {worker(w).move_up()},
                "zigzag5" => if memory.t % 2 == 0  {worker(w).move_down()} else {worker(w).move_right()},
                "zigzag7" => if memory.t % 2 == 0{worker(w).move_up()} else {worker(w).move_right()},
            }
        }
    }
    else {
        let xPosition = worker(w).x;
        let yPosition = worker(w).y;
        let newPositions = [];
        if xPosition - 1 >= 0 {
            let leftPos = #{
                dir: 0,
                xPos: xPosition - 1,
                yPos: yPosition,
                color: map[xPosition - 1][yPosition],
            };
            newPositions.push(leftPos);
        }
        if xPosition + 1 < 40 {
            let rightPos = #{
                dir: 1,
                xPos: xPosition + 1,
                yPos: yPosition,
                color: map[xPosition + 1][yPosition],
            };
            newPositions.push(rightPos);
        }
        if yPosition + 1 < 40 {
            let upPos = #{
                dir: 2,
                xPos: xPosition,
                yPos: yPosition + 1,
                color: map[xPosition][yPosition + 1],
            };
            newPositions.push(upPos);
        }
        if yPosition - 1 >= 0 {
            let downPos = #{
                dir: 3,
                xPos: xPosition,
                yPos: yPosition - 1,
                color: map[xPosition][yPosition - 1],
            };
            newPositions.push(downPos);
        }
        let movedToEmpty = false;
        for (newPosition, count) in newPositions {
            let newX = newPosition.xPos;
            let newY = newPosition.yPos;
            let dir = newPosition.dir;
            if (newPosition.color != memory["myColor"]) {
                if (newPosition.color != Tile::EMPTY) {
                    if !usedPositions.contains([newX,newY]) {
                        MoveToDirection(w, dir);
                        if (movedToEmpty) {usedPositions.pop();}
                        usedPositions.push([newX,newY]);
                        movedToEmpty = true;
                        break;
                    }
                }
                if !usedPositions.contains([newX,newY]) && !movedToEmpty {
                    MoveToDirection(w, dir);
                    usedPositions.push([newX,newY]);
                    movedToEmpty = true;
                }
            }
        }
        if !movedToEmpty {
            let r = (rand() % 4).abs();
            switch r {
                0 => worker(w).move_up(),
                1 => worker(w).move_down(),
                2 => worker(w).move_right(),
                3 => worker(w).move_left(),
            }
            let l = (rand() % 2).abs();
            let xx = worker(w).x;
            let yy = worker(w).y;
            switch cuadrante(worker(w).x, worker(w).y) {
                3 =>
                    if l == 0 && !usedPositions.contains([xx,yy-1]) {
                        usedPositions.push([xx,yy-1]);
                        worker(w).move_down();
                    }
                    else {
                        usedPositions.push([xx-1,yy]);
                        worker(w).move_left();
                    },
                2 =>
                    if l == 0  && !usedPositions.contains([xx,yy+1]){
                        usedPositions.push([xx,yy+1]);
                        worker(w).move_up();
                    }
                    else {
                        usedPositions.push([xx-1,yy]);
                        worker(w).move_left();
                    },
                1 =>
                    if l == 0  && !usedPositions.contains([xx,yy-1]) {
                        usedPositions.push([xx,yy-1]);
                        worker(w).move_down();
                    }
                    else {
                        usedPositions.push([xx+1,yy]);
                        worker(w).move_right();
                    },
                0 =>
                    if l == 0  && !usedPositions.contains([xx,yy+1]) {
                        usedPositions.push([xx,yy+1]);
                        worker(w).move_up();
                    }
                    else {
                        usedPositions.push([xx+1,yy]);
                        worker(w).move_right();
                    },
            }
        }
    }
}
