import { zip } from "@std/collections/zip";

function part1(lines: string[]) {
  const input = lines
    .filter(Boolean)
    .map((text) => text.split(/\s+/).map(Number));

  const listLeft = input.map((pair) => pair[0]).sort();
  const listRight = input.map((pair) => pair[1]).sort();

  return zip(listLeft, listRight)
    .map(([left, right]) => Math.abs(left - right))
    .reduce((acc, curr) => acc + curr, 0);
}

function part2() {

}

// Learn more at https://docs.deno.com/runtime/manual/examples/module_metadata#concepts
if (import.meta.main) {
  const inputFilename = "input-base.txt";
  const text = await Deno.readTextFile(inputFilename);
  const lines = text.split("\n");

  const res = part1(lines);

  console.log("[LS] -> main.ts:21 -> res: ", res);
}
