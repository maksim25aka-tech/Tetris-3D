// Tetris3D.java — трёхмерный тетрис на Java

import java.util.*;
import java.io.*;

public class Tetris3D {
    private static final int W=4, H=4, D=4;
    private int[][][] field = new int[W][H][D];
    private int score = 0, level = 1;
    private double fallInterval = 1.0;
    private List<int[]> currentPiece, nextPiece;
    private int[] currentPos = new int[3]; // x,y,z
    private boolean gameOver = false;
    private Random rand = new Random();
    private List<List<int[]>> shapes = new ArrayList<>();

    public Tetris3D() {
        // инициализация фигур
        int[][][] shapeData = {
            {{0,0,0},{1,0,0},{0,1,0},{1,1,0}},
            {{0,0,0},{1,0,0},{2,0,0},{3,0,0}},
            {{0,0,0},{1,0,0},{2,0,0},{1,1,0}},
            {{0,0,0},{1,0,0},{2,0,0},{0,1,0}},
            {{0,0,0},{1,0,0},{2,0,0},{2,1,0}},
            {{1,0,0},{2,0,0},{0,1,0},{1,1,0}},
            {{0,0,0},{1,0,0},{1,1,0},{2,1,0}}
        };
        for (int[][] s : shapeData) {
            List<int[]> list = new ArrayList<>();
            for (int[] p : s) list.add(p.clone());
            shapes.add(list);
        }
        nextPiece = shapes.get(rand.nextInt(shapes.size()));
        spawnPiece();
    }

    private boolean isValid(List<int[]> piece, int[] pos) {
        int px=pos[0], py=pos[1], pz=pos[2];
        for (int[] p : piece) {
            int x=px+p[0], y=py+p[1], z=pz+p[2];
            if (x<0||x>=W||y<0||y>=H||z<0||z>=D) return false;
            if (field[x][y][z]!=0) return false;
        }
        return true;
    }

    private void placePiece() {
        int px=currentPos[0], py=currentPos[1], pz=currentPos[2];
        for (int[] p : currentPiece) {
            field[px+p[0]][py+p[1]][pz+p[2]] = 1;
        }
        clearLayers();
        spawnPiece();
    }

    private void clearLayers() {
        int cleared = 0;
        for (int z=0; z<D; z++) {
            boolean full = true;
            for (int x=0; x<W; x++) for (int y=0; y<H; y++) if (field[x][y][z]==0) full=false;
            if (full) {
                for (int zz=z; zz<D-1; zz++) {
                    for (int x=0; x<W; x++) for (int y=0; y<H; y++) field[x][y][zz] = field[x][y][zz+1];
                }
                for (int x=0; x<W; x++) for (int y=0; y<H; y++) field[x][y][D-1] = 0;
                cleared++; z--;
            }
        }
        if (cleared>0) {
            score += cleared*100;
            level = 1 + score/500;
            fallInterval = Math.max(0.2, 1.0/(1+(level-1)*0.2));
        }
    }

    private void spawnPiece() {
        currentPiece = nextPiece;
        nextPiece = shapes.get(rand.nextInt(shapes.size()));
        currentPos = new int[]{1,1,3};
        if (!isValid(currentPiece, currentPos)) gameOver = true;
    }

    private boolean move(int dx, int dy, int dz) {
        int[] newPos = {currentPos[0]+dx, currentPos[1]+dy, currentPos[2]+dz};
        if (isValid(currentPiece, newPos)) {
            currentPos = newPos;
            return true;
        }
        return false;
    }

    private void rotate(String axis) {
        List<int[]> newPiece = new ArrayList<>();
        if (axis.equals("z")) {
            for (int[] p : currentPiece) newPiece.add(new int[]{-p[1], p[0], p[2]});
        } else if (axis.equals("x")) {
            for (int[] p : currentPiece) newPiece.add(new int[]{p[0], -p[2], p[1]});
        } else if (axis.equals("y")) {
            for (int[] p : currentPiece) newPiece.add(new int[]{p[2], p[1], -p[0]});
        }
        if (isValid(newPiece, currentPos)) currentPiece = newPiece;
    }

    private void hardDrop() {
        while (isValid(currentPiece, new int[]{currentPos[0], currentPos[1], currentPos[2]-1})) {
            currentPos[2]--;
        }
        placePiece();
    }

    private void update() {
        if (isValid(currentPiece, new int[]{currentPos[0], currentPos[1], currentPos[2]-1})) {
            currentPos[2]--;
        } else {
            placePiece();
        }
    }

    private void draw() {
        System.out.print("\033[H\033[2J");
        System.out.flush();
        System.out.println("ТЕТРИС 3D");
        System.out.println("Очки: "+score+" | Уровень: "+level);
        for (int z=D-1; z>=0; z--) {
            System.out.println("\nСЛОЙ "+(z+1)+" (Y="+z+")");
            System.out.println("  0 1 2 3");
            for (int y=H-1; y>=0; y--) {
                System.out.print(y+" ");
                for (int x=0; x<W; x++) {
                    if (field[x][y][z]!=0) System.out.print("X ");
                    else {
                        boolean inPiece = false;
                        for (int[] p : currentPiece) {
                            if (x==currentPos[0]+p[0] && y==currentPos[1]+p[1] && z==currentPos[2]+p[2]) inPiece=true;
                        }
                        System.out.print(inPiece ? "█ " : ". ");
                    }
                }
                System.out.println();
            }
        }
        System.out.println("\nСледующая фигура:");
        for (int dy=0; dy<2; dy++) {
            for (int dx=0; dx<2; dx++) {
                boolean found = false;
                for (int[] p : nextPiece) if (p[0]==dx && p[1]==dy && p[2]==0) found=true;
                System.out.print(found ? "█ " : ". ");
            }
            System.out.println();
        }
        System.out.println("\nWASD - движение, Q/E - поворот Z, R - поворот X, Space - падение, Esc - выход");
    }

    public void run() throws IOException {
        long lastFall = System.currentTimeMillis();
        while (!gameOver) {
            draw();
            // чтение ввода (неблокирующее с использованием System.in.available())
            if (System.in.available() > 0) {
                char ch = (char)System.in.read();
                if (ch == 'a') move(-1,0,0);
                else if (ch == 'd') move(1,0,0);
                else if (ch == 'w') move(0,1,0);
                else if (ch == 's') move(0,-1,0);
                else if (ch == 'q') rotate("z");
                else if (ch == 'e') { rotate("z"); rotate("z"); rotate("z"); }
                else if (ch == 'r') rotate("x");
                else if (ch == ' ') hardDrop();
                else if (ch == 27) break; // Esc
            }
            long now = System.currentTimeMillis();
            if ((now - lastFall) > fallInterval*1000) {
                update();
                lastFall = now;
            }
            try { Thread.sleep(50); } catch (InterruptedException e) {}
        }
        System.out.println("ИГРА ОКОНЧЕНА! Счёт: "+score);
    }

    public static void main(String[] args) throws IOException {
        new Tetris3D().run();
    }
}
