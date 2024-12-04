import { assertEquals } from "@std/assert";

if (import.meta.main) {
  // const inputFile = "input-base.txt";
  const inputFile = "input.txt";
  const text = await Deno.readTextFile(inputFile);

  const grid = textIntoGrid(text);

  const xPositions = findAllX(grid);

  const res1 = part1(grid, xPositions);
  console.log("[LS] -> main.ts:20 -> res1: ", res1);
}

function part1(grid: Grid, xPositions: Pos[]) {
  const theChars = "XMAS".split("");

  const counts = xPositions.map(({ x, y }) => {
    const res = [
      theChars.every((char, index) => grid[y]?.[x + index] === char),
      theChars.every((char, index) => grid[y]?.[x - index] === char),

      theChars.every((char, index) => grid[y + index]?.[x] === char),
      theChars.every((char, index) => grid[y - index]?.[x] === char),

      theChars.every((char, index) => grid[y + index]?.[x + index] === char),
      theChars.every((char, index) => grid[y + index]?.[x - index] === char),

      theChars.every((char, index) => grid[y - index]?.[x + index] === char),
      theChars.every((char, index) => grid[y - index]?.[x - index] === char),
    ].filter(Boolean);

    return res.length;
  });

  return counts.reduce((acc, curr) => acc + curr, 0);
}

type Grid = string[][];

type Pos = {
  x: number;
  y: number;
};

function textIntoGrid(text: string) {
  return text
    .split("\n")
    .filter(Boolean)
    .map((line) => line.split(""));
}

function findAllX(grid: Grid) {
  const res: Pos[] = [];

  grid.forEach((_char, y) => {
    grid[y].forEach((char, x) => {
      if (char === "X") {
        res.push({
          x,
          y,
        });
      }
    });
  });

  return res;
}
