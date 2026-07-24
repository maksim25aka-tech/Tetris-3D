// tetris3d.js — трёхмерный тетрис на JavaScript (Node.js)

const W = 4, H = 4, D = 4;

const shapes = [
    [[0,0,0],[1,0,0],[0,1,0],[1,1,0]],
    [[0,0,0],[1,0,0],[2,0,0],[3,0,0]],
    [[0,0,0],[1,0,0],[2,0,0],[1,1,0]],
    [[0,0,0],[1,0,0],[2,0,0],[0,1,0]],
    [[0,0,0],[1,0,0],[2,0,0],[2,1,0]],
    [[1,0,0],[2,0,0],[0,1,0],[1,1,0]],
    [[0,0,0],[1,0,0],[1,1,0],[2,1,0]]
];

class Tetris3D {
    constructor() {
        this.field = Array.from({length: W}, () => 
            Array.from({length: H}, () => 
                Array(D).fill(0)
            )
        );
        this.score = 0;
        this.level = 1;
        this.fallInterval = 1.0;
        this.currentPiece = [];
        this.nextPiece = this.randomPiece();
        this.currentPos = [0,0,0];
        this.gameOver = false;
        this.lastFall = Date.now();
        this.spawnPiece();
    }

    randomPiece() {
        return shapes[Math.floor(Math.random() * shapes.length)].map(p => [...p]);
    }

    isValid(piece, pos) {
        const [px, py, pz] = pos;
        for (const [dx, dy, dz] of piece) {
            const x = px + dx;
            const y = py + dy;
            const z = pz + dz;
            if (x < 0 || x >= W || y < 0 || y >= H || z < 0 || z >= D) return false;
            if (this.field[x][y][z] !== 0) return false;
        }
        return true;
    }

    placePiece() {
        const [px, py, pz] = this.currentPos;
        for (const [dx, dy, dz] of this.currentPiece) {
            this.field[px+dx][py+dy][pz+dz] = 1;
        }
        this.clearLayers();
        this.spawnPiece();
    }

    clearLayers() {
        let cleared = 0;
        for (let z = 0; z < D; z++) {
            let full = true;
            for (let x = 0; x < W; x++) {
                for (let y = 0; y < H; y++) {
                    if (this.field[x][y][z] === 0) { full = false; break; }
                }
                if (!full) break;
            }
            if (full) {
                for (let zz = z; zz < D-1; zz++) {
                    for (let x = 0; x < W; x++) {
                        for (let y = 0; y < H; y++) {
                            this.field[x][y][zz] = this.field[x][y][zz+1];
                        }
                    }
                }
                for (let x = 0; x < W; x++) {
                    for (let y = 0; y < H; y++) {
                        this.field[x][y][D-1] = 0;
                    }
                }
                cleared++;
                z--; // проверяем тот же слой снова
            }
        }
        if (cleared) {
            this.score += cleared * 100;
            this.level = 1 + Math.floor(this.score / 500);
            this.fallInterval = Math.max(0.2, 1.0 / (1 + (this.level-1) * 0.2));
        }
    }

    spawnPiece() {
        this.currentPiece = this.nextPiece;
        this.nextPiece = this.randomPiece();
        this.currentPos = [1,1,3];
        if (!this.isValid(this.currentPiece, this.currentPos)) {
            this.gameOver = true;
        }
    }

    move(dx, dy, dz) {
        const newPos = [this.currentPos[0]+dx, this.currentPos[1]+dy, this.currentPos[2]+dz];
        if (this.isValid(this.currentPiece, newPos)) {
            this.currentPos = newPos;
            return true;
        }
        return false;
    }

    rotate(axis) {
        const newPiece = this.currentPiece.map(([x,y,z]) => {
            if (axis === 'z') return [-y, x, z];
            if (axis === 'x') return [x, -z, y];
            if (axis === 'y') return [z, y, -x];
            return [x,y,z];
        });
        if (this.isValid(newPiece, this.currentPos)) {
            this.currentPiece = newPiece;
        }
    }

    hardDrop() {
        while (this.isValid(this.currentPiece, [this.currentPos[0], this.currentPos[1], this.currentPos[2]-1])) {
            this.currentPos[2]--;
        }
        this.placePiece();
    }

    update() {
        if (this.isValid(this.currentPiece, [this.currentPos[0], this.currentPos[1], this.currentPos[2]-1])) {
            this.currentPos[2]--;
        } else {
            this.placePiece();
        }
    }

    draw() {
        console.clear();
        console.log("ТЕТРИС 3D");
        console.log(`Очки: ${this.score} | Уровень: ${this.level}`);
        for (let z = D-1; z >= 0; z--) {
            console.log(`\nСЛОЙ ${z+1} (Y=${z})`);
            console.log("  0 1 2 3");
            for (let y = H-1; y >= 0; y--) {
                let row = `${y} `;
                for (let x = 0; x < W; x++) {
                    if (this.field[x][y][z] !== 0) {
                        row += "X ";
                    } else {
                        let inPiece = false;
                        for (const [dx,dy,dz] of this.currentPiece) {
                            if (x === this.currentPos[0]+dx && y === this.currentPos[1]+dy && z === this.currentPos[2]+dz) {
                                inPiece = true;
                                break;
                            }
                        }
                        row += inPiece ? "█ " : ". ";
                    }
                }
                console.log(row);
            }
        }
        console.log("\nСледующая фигура:");
        for (let dy = 0; dy < 2; dy++) {
            let row = "";
            for (let dx = 0; dx < 2; dx++) {
                let found = false;
                for (const [ex,ey,ez] of this.nextPiece) {
                    if (ex === dx && ey === dy && ez === 0) { found = true; break; }
                }
                row += found ? "█ " : ". ";
            }
            console.log(row);
        }
        console.log("\nWASD - движение, Q/E - поворот Z, R - поворот X, Space - падение, Esc - выход");
    }

    run() {
        // Для неблокирующего ввода используем readline с keypress
        const readline = require('readline');
        readline.emitKeypressEvents(process.stdin);
        process.stdin.setRawMode(true);
        process.stdin.on('keypress', (ch, key) => {
            if (key) {
                if (key.name === 'a') this.move(-1,0,0);
                else if (key.name === 'd') this.move(1,0,0);
                else if (key.name === 'w') this.move(0,1,0);
                else if (key.name === 's') this.move(0,-1,0);
                else if (key.name === 'q') this.rotate('z');
                else if (key.name === 'e') { this.rotate('z'); this.rotate('z'); this.rotate('z'); }
                else if (key.name === 'r') this.rotate('x');
                else if (key.name === 'space') this.hardDrop();
                else if (key.name === 'escape') process.exit(0);
            }
        });

        setInterval(() => {
            if (!this.gameOver) {
                const now = Date.now();
                if ((now - this.lastFall) / 1000 > this.fallInterval) {
                    this.update();
                    this.lastFall = now;
                }
                this.draw();
            } else {
                console.log(`ИГРА ОКОНЧЕНА! Счёт: ${this.score}`);
                process.exit(0);
            }
        }, 50);
    }
}

const game = new Tetris3D();
game.run();
