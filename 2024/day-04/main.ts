if (import.meta.main) {
  const inputFile = "input.txt";
  const text = await Deno.readTextFile(inputFile);

  const grid = textIntoGrid(text);

  const res1 = part1(grid);
  console.log("[LS] -> main.ts:20 -> res1: ", res1);

  const res2 = part2(grid);
  console.log("[LS] -> main.ts:13 -> res2: ", res2);
}

function part1(grid: Grid) {
  const xPositions = findPositionsForChar(grid, "X");
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

function part2(grid: Grid) {
  const aPositions = findPositionsForChar(grid, "A");

  const counts = aPositions
    .map(({ x, y }) => {
      const conf1 =
        grid[y + 1]?.[x + 1] === "M" &&
        grid[y + 1]?.[x - 1] === "M" &&
        grid[y - 1]?.[x + 1] === "S" &&
        grid[y - 1]?.[x - 1] === "S";

      const conf2 =
        grid[y + 1]?.[x + 1] === "S" &&
        grid[y + 1]?.[x - 1] === "S" &&
        grid[y - 1]?.[x + 1] === "M" &&
        grid[y - 1]?.[x - 1] === "M";

      const conf3 =
        grid[y + 1]?.[x + 1] === "M" &&
        grid[y + 1]?.[x - 1] === "S" &&
        grid[y - 1]?.[x + 1] === "M" &&
        grid[y - 1]?.[x - 1] === "S";

      const conf4 =
        grid[y + 1]?.[x + 1] === "S" &&
        grid[y + 1]?.[x - 1] === "M" &&
        grid[y - 1]?.[x + 1] === "S" &&
        grid[y - 1]?.[x - 1] === "M";

      return conf1 || conf2 || conf3 || conf4;
    })
    .filter(Boolean);

  return counts.length;
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

function findPositionsForChar(grid: Grid, theChar: "X" | "A") {
  const res: Pos[] = [];

  grid.forEach((_char, y) => {
    grid[y].forEach((char, x) => {
      if (char === theChar) {
        res.push({
          x,
          y,
        });
      }
    });
  });

  return res;
}
