import { readFile } from "fs/promises";

export default async function run(usePractice: boolean) {
  const inputFile = usePractice ? "/input.practice.txt" : "/input.txt";
  const input = await readFile(import.meta.dir + inputFile).then((d) =>
    d.toString().trim()
  );

  console.log("Part 1:", solve1(input));
  console.log("Part 2:", solve2(input));
}

function inBounds(i: number, j: number, m: number, n: number): boolean {
  return i >= 0 && i < m && j >= 0 && j < n;
}

function solve1(input: string) {
  const grid = input.split("\n").map((r) => r.split(""));
  const m = grid.length;
  const n = grid[0]?.length ?? 0;
  let count = 0;
  for (let i = 0; i < m; i++) {
    for (let j = 0; j < n; j++) {
      if (grid[i]?.[j] !== "@") {
        continue;
      }
      const directions = [
        [-1, 0],
        [-1, 1],
        [0, 1],
        [1, 1],
        [1, 0],
        [1, -1],
        [0, -1],
        [-1, -1],
      ];
      let neighbors = 0;
      for (const [di, dj] of directions) {
        if (neighbors >= 4) break;
        const ni = i + (di as number);
        const nj = j + (dj as number);
        if (inBounds(ni, nj, m, n) && grid[ni]?.[nj] === "@") {
          neighbors++;
        }
      }
      if (neighbors < 4) {
        count += 1;
      }
    }
  }
  return count;
}

function solve2(input: string) {
  const grid = input.split("\n").map((r) => r.split(""));
  const m = grid.length;
  const n = grid[0]?.length ?? 0;
  let count = 0;
  while (true) {
    const removed = [];
    for (let i = 0; i < m; i++) {
      for (let j = 0; j < n; j++) {
        if (grid[i]?.[j] !== "@") {
          continue;
        }
        const directions = [
          [-1, 0],
          [-1, 1],
          [0, 1],
          [1, 1],
          [1, 0],
          [1, -1],
          [0, -1],
          [-1, -1],
        ];
        let neighbors = 0;
        for (const [di, dj] of directions) {
          if (neighbors >= 4) break;
          const ni = i + (di as number);
          const nj = j + (dj as number);
          if (inBounds(ni, nj, m, n) && grid[ni]?.[nj] === "@") {
            neighbors++;
          }
        }
        if (neighbors < 4) {
          removed.push([i, j]);
        }
      }
    }
    count += removed.length;
    removed.forEach((coord) => {
      const i = coord[0];
      const j = coord[1];
      if (i != null && j != null && grid[i]) {
        grid[i][j] = ".";
      }
    });
    if (removed.length === 0) {
      break;
    }
  }
  return count;
}
