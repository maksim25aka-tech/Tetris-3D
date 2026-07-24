// tetris3d.cs — трёхмерный тетрис на C#

using System;
using System.Collections.Generic;
using System.Threading;
using System.Threading.Tasks;

class Tetris3D
{
    const int W=4, H=4, D=4;
    int[,,] field = new int[W,H,D];
    int score = 0, level = 1;
    double fallInterval = 1.0;
    List<(int,int,int)> currentPiece, nextPiece;
    (int x, int y, int z) currentPos;
    bool gameOver = false;
    Random rand = new Random();

    List<List<(int,int,int)>> shapes = new List<List<(int,int,int)>>();

    public Tetris3D()
    {
        // инициализация фигур
        shapes.Add(new List<(int,int,int)>{(0,0,0),(1,0,0),(0,1,0),(1,1,0)});
        shapes.Add(new List<(int,int,int)>{(0,0,0),(1,0,0),(2,0,0),(3,0,0)});
        shapes.Add(new List<(int,int,int)>{(0,0,0),(1,0,0),(2,0,0),(1,1,0)});
        shapes.Add(new List<(int,int,int)>{(0,0,0),(1,0,0),(2,0,0),(0,1,0)});
        shapes.Add(new List<(int,int,int)>{(0,0,0),(1,0,0),(2,0,0),(2,1,0)});
        shapes.Add(new List<(int,int,int)>{(1,0,0),(2,0,0),(0,1,0),(1,1,0)});
        shapes.Add(new List<(int,int,int)>{(0,0,0),(1,0,0),(1,1,0),(2,1,0)});
        nextPiece = shapes[rand.Next(shapes.Count)];
        SpawnPiece();
    }

    bool IsValid(List<(int,int,int)> piece, (int x,int y,int z) pos)
    {
        int px=pos.x, py=pos.y, pz=pos.z;
        foreach (var p in piece) {
            int x=px+p.Item1, y=py+p.Item2, z=pz+p.Item3;
            if (x<0||x>=W||y<0||y>=H||z<0||z>=D) return false;
            if (field[x,y,z]!=0) return false;
        }
        return true;
    }

    void PlacePiece()
    {
        int px=currentPos.x, py=currentPos.y, pz=currentPos.z;
        foreach (var p in currentPiece) {
            field[px+p.Item1, py+p.Item2, pz+p.Item3] = 1;
        }
        ClearLayers();
        SpawnPiece();
    }

    void ClearLayers()
    {
        int cleared = 0;
        for (int z=0; z<D; z++) {
            bool full = true;
            for (int x=0; x<W; x++) for (int y=0; y<H; y++) if (field[x,y,z]==0) full=false;
            if (full) {
                for (int zz=z; zz<D-1; zz++) {
                    for (int x=0; x<W; x++) for (int y=0; y<H; y++) field[x,y,zz] = field[x,y,zz+1];
                }
                for (int x=0; x<W; x++) for (int y=0; y<H; y++) field[x,y,D-1] = 0;
                cleared++; z--;
            }
        }
        if (cleared>0) {
            score += cleared*100;
            level = 1 + score/500;
            fallInterval = Math.Max(0.2, 1.0/(1+(level-1)*0.2));
        }
    }

    void SpawnPiece()
    {
        currentPiece = nextPiece;
        nextPiece = shapes[rand.Next(shapes.Count)];
        currentPos = (1,1,3);
        if (!IsValid(currentPiece, currentPos)) gameOver = true;
    }

    bool Move(int dx, int dy, int dz)
    {
        var newPos = (currentPos.x+dx, currentPos.y+dy, currentPos.z+dz);
        if (IsValid(currentPiece, newPos)) {
            currentPos = newPos;
            return true;
        }
        return false;
    }

    void Rotate(string axis)
    {
        List<(int,int,int)> newPiece = new List<(int,int,int)>();
        if (axis=="z") {
            foreach (var p in currentPiece) newPiece.Add((-p.Item2, p.Item1, p.Item3));
        } else if (axis=="x") {
            foreach (var p in currentPiece) newPiece.Add((p.Item1, -p.Item3, p.Item2));
        } else if (axis=="y") {
            foreach (var p in currentPiece) newPiece.Add((p.Item3, p.Item2, -p.Item1));
        }
        if (IsValid(newPiece, currentPos)) currentPiece = newPiece;
    }

    void HardDrop()
    {
        while (IsValid(currentPiece, (currentPos.x, currentPos.y, currentPos.z-1))) {
            currentPos.z--;
        }
        PlacePiece();
    }

    void Update()
    {
        if (IsValid(currentPiece, (currentPos.x, currentPos.y, currentPos.z-1))) {
            currentPos.z--;
        } else {
            PlacePiece();
        }
    }

    void Draw()
    {
        Console.Clear();
        Console.WriteLine("ТЕТРИС 3D");
        Console.WriteLine($"Очки: {score} | Уровень: {level}");
        for (int z=D-1; z>=0; z--) {
            Console.WriteLine($"\nСЛОЙ {z+1} (Y={z})");
            Console.WriteLine("  0 1 2 3");
            for (int y=H-1; y>=0; y--) {
                Console.Write(y+" ");
                for (int x=0; x<W; x++) {
                    if (field[x,y,z]!=0) Console.Write("X ");
                    else {
                        bool inPiece = false;
                        foreach (var p in currentPiece) {
                            if (x==currentPos.x+p.Item1 && y==currentPos.y+p.Item2 && z==currentPos.z+p.Item3) inPiece=true;
                        }
                        Console.Write(inPiece ? "█ " : ". ");
                    }
                }
                Console.WriteLine();
            }
        }
        Console.WriteLine("\nСледующая фигура:");
        for (int dy=0; dy<2; dy++) {
            for (int dx=0; dx<2; dx++) {
                bool found = false;
                foreach (var p in nextPiece) if (p.Item1==dx && p.Item2==dy && p.Item3==0) found=true;
                Console.Write(found ? "█ " : ". ");
            }
            Console.WriteLine();
        }
        Console.WriteLine("\nWASD - движение, Q/E - поворот Z, R - поворот X, Space - падение, Esc - выход");
    }

    public async Task Run()
    {
        var lastFall = DateTime.Now;
        while (!gameOver) {
            Draw();
            if (Console.KeyAvailable) {
                var key = Console.ReadKey(true).KeyChar;
                if (key == 'a') Move(-1,0,0);
                else if (key == 'd') Move(1,0,0);
                else if (key == 'w') Move(0,1,0);
                else if (key == 's') Move(0,-1,0);
                else if (key == 'q') Rotate("z");
                else if (key == 'e') { Rotate("z"); Rotate("z"); Rotate("z"); }
                else if (key == 'r') Rotate("x");
                else if (key == ' ') HardDrop();
                else if (key == 27) break;
            }
            var now = DateTime.Now;
            if ((now - lastFall).TotalSeconds > fallInterval) {
                Update();
                lastFall = now;
            }
            await Task.Delay(50);
        }
        Console.WriteLine($"ИГРА ОКОНЧЕНА! Счёт: {score}");
    }

    static void Main() => new Tetris3D().Run().Wait();
}
