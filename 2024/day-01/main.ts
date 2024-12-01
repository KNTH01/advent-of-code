import { zip } from "@std/collections/zip";

// Learn more at https://docs.deno.com/runtime/manual/examples/module_metadata#concepts
if (import.meta.main) {
  const inputFilename = "input.txt";
  const text = await Deno.readTextFile(inputFilename);
  const lines = text.split("\n");

  const input = lines
    .filter(Boolean)
    .map((text) => text.split(/\s+/).map(Number));

  const listLeft = input.map((pair) => pair[0]).sort();
  const listRight = input.map((pair) => pair[1]).sort();

  const resPart1 = zip(listLeft, listRight)
    .map(([left, right]) => Math.abs(left - right))
    .reduce((acc, curr) => acc + curr, 0);

  console.log("[LS] -> main.ts:18 -> resPart1: ", resPart1);

  const countOccListRight = listRight.reduce(
    (acc: Record<number, number>, curr) => {
      if (!acc[curr]) {
        acc[curr] = 1;
      } else {
        acc[curr]++;
      }

      return acc;
    },
    {},
  );

  const resPart2 = listLeft
    .map((n) => (countOccListRight[n] ? n * countOccListRight[n] : 0))
    .reduce((acc, curr) => acc + curr, 0);

  console.log("[LS] -> main.ts:35 -> resPart2: ", resPart2);
}
