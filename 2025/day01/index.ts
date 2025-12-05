import { readFile } from "fs/promises";

export default async function run(usePractice: boolean) {
  const input = await readFile(
    import.meta.dir + (usePractice ? "/input.practice.txt" : "/input.txt")
  ).then((d) => d.toString().trim());
  console.log("Part 1: ", solve1(input));
  console.log("Part 2: ", solve2(input));
}

function solve1(input: string) {
  const rotations = input
    .split("\n")
    .filter((r) => r !== "")
    .map((r) => (r.charAt(0) === "L" ? -1 : 1) * Number.parseInt(r.slice(1)));
  let dial = 50;
  let zeros = 0;
  for (const r of rotations) {
    const next = (dial + r) % 100;
    // wrap around to the other side of the dial or just use real modulo
    dial = next + (next < 0 ? 100 : 0);
    if (dial === 0) {
      zeros++;
    }
  }
  return zeros;
}

function solve2(input: string) {
  const rotations = input
    .split("\n")
    .filter((r) => r !== "")
    .map((r) => (r.charAt(0) === "L" ? -1 : 1) * Number.parseInt(r.slice(1)));
  let dial = 50;
  let zeros = 0;
  for (const r of rotations) {
    const start = dial;
    const next = (dial + r) % 100;
    // wrap around to the other side of the dial or just use real modulo
    dial = next + (next < 0 ? 100 : 0);

    const zeroPassedOrLanded =
      (dial === 0 ? 1 : 0) ||
      (r > 0 && dial < start ? 1 : 0) ||
      (r < 0 && dial > start ? 1 : 0);

    zeros +=
      // count full rotations
      Math.floor(Math.abs(r) / 100) +
      // starting at zero doesn't count as passing through but we also need to check
      // with respect to the start position
      (start === 0 ? 0 : zeroPassedOrLanded);

    dial = next;
  }
  return zeros;
}
