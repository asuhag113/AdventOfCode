import { readFile } from "fs/promises";

export default async function run(usePractice: boolean) {
  const inputFile = usePractice ? "/input.practice.txt" : "/input.txt";
  const input = await readFile(import.meta.dir + inputFile).then((d) =>
    d.toString().trim()
  );

  console.log("Part 1:", solve1(input));
  console.log("Part 2:", solve2(input));
}

function solve1(input: string) {
  const invalidIds: number[] = [];
  input
    .split(",")
    .map((r) => {
      const [i, j] = r.split("-") as [string, string];
      return [Number.parseInt(i), Number.parseInt(j)] as [number, number];
    })
    .forEach(([l, r]) => {
      for (let i = l; i < r + 1; i++) {
        const numAsString = String(i);
        if (numAsString.length % 2 !== 0) {
          continue;
        }
        const window = numAsString.length / 2;
        if (numAsString.slice(0, window) === numAsString.slice(window)) {
          invalidIds.push(i);
        }
      }
    });
  return invalidIds.reduce((acc, e) => acc + e, 0);
}

function solve2(input: string) {
  const invalidIds: number[] = [];
  input
    .split(",")
    .map((r) => {
      const [i, j] = r.split("-") as [string, string];
      return [Number.parseInt(i), Number.parseInt(j)] as [number, number];
    })
    .forEach(([l, r]) => {
      for (let i = l; i < r + 1; i++) {
        const numAsString = String(i);
        if (numAsString.length === 1) {
          continue;
        }
        for (let w = 1; w <= numAsString.length / 2; w++) {
          if (numAsString.length % w !== 0) {
            continue;
          }
          const chunks = Array.from(
            { length: numAsString.length / w },
            (_, i) => numAsString.substring(i * w, (i + 1) * w)
          );

          if (chunks.every((c, i, p) => i === 0 || c === p[i - 1])) {
            invalidIds.push(i);
            break;
          }
        }
      }
    });
  return invalidIds.reduce((acc, e) => acc + e, 0);
}
