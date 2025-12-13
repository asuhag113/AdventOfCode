import { readFile } from "fs/promises";

export default async function run(usePractice: boolean) {
  const inputFile = usePractice ? "/input.practice.txt" : "/input.txt";
  const input = await readFile(import.meta.dir + inputFile).then((d) =>
    d.toString().trim()
  );

  console.log("Part 1:", solve1(input));
  console.log("Part 2:", solve2(input));
}

function maxSegment(
  input: string,
  answer: string,
  batteryCount: number
): string {
  if (answer.length === batteryCount) {
    return answer;
  }
  const [max, idx] = input
    .split("")
    .slice(0, input.length - (batteryCount - answer.length) + 1)
    .reduce(
      (acc, e, i) =>
        Number.parseInt(e) > acc[0] ? [Number.parseInt(e), i] : acc,
      [0, 0]
    );
  return maxSegment(
    input.substring(idx + 1),
    answer + String(max),
    batteryCount
  );
}

function solve1(input: string) {
  return input
    .split("\n")
    .reduce((acc, e) => acc + parseInt(maxSegment(e, "", 2)), 0);
}

function solve2(input: string) {
  return input
    .split("\n")
    .reduce((acc, e) => acc + parseInt(maxSegment(e, "", 12)), 0);
}
