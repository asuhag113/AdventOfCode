import { exists, mkdir, writeFile } from "fs/promises";

const dayArg = process.argv[2];

if (!dayArg) {
  console.error("Usage: bun newDay.ts <day>");
  process.exit(1);
}

const day = dayArg.padStart(2, "0");
const dayDir = `${import.meta.dir}/day${day}`;

if (await exists(dayDir)) {
  console.error(`Day ${day} already exists!`);
  process.exit(1);
}

await mkdir(dayDir);

const template = `import { readFile } from "fs/promises";

export default async function run(usePractice: boolean) {
  const inputFile = usePractice ? "/input.practice.txt" : "/input.txt";
  const input = await readFile(import.meta.dir + inputFile).then((d) =>
    d.toString().trim()
  );

  console.log("Part 1:", solve1(input));
  console.log("Part 2:", solve2(input));
}

function solve1(input: string) {
  // TODO: implement
  return 0;
}

function solve2(input: string) {
  // TODO: implement
  return 0;
}
`;

await Promise.all([
  writeFile(`${dayDir}/index.ts`, template),
  writeFile(`${dayDir}/input.txt`, ""),
  writeFile(`${dayDir}/input.practice.txt`, ""),
]);
