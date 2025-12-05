const dayArg = process.argv[2];
const usePractice =
  process.argv.includes("--practice") || process.argv.includes("-p");

if (!dayArg) {
  console.error("Usage: bun index.ts <day> [--practice|-p]");
  process.exit(1);
}

const day = dayArg.padStart(2, "0");
const modulePath = `./day${day}/index.ts`;

const { default: run } = await import(modulePath);
await run(usePractice);
