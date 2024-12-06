type Grid = string[][];

type Pos = {
  x: number;
  y: number;
};

enum Dir {
  North,
  East,
  South,
  West,
}

if (import.meta.main) {
  // const inputFile = "input-base.txt";
  const inputFile = "input.txt";
  const text = await Deno.readTextFile(inputFile);

  const grid = textIntoGrid(text);
  const guardPos = findPosition(grid);

  if (!guardPos) {
    throw new Error("Cannot find the guard?");
  }

  const res1 = part1(grid, guardPos);
  console.log("[LS] -> main.ts:27 -> res1: ", res1);
}

function part1(grid: Grid, guardPos: Pos) {
  let currDir = Dir.North;
  let currPos = { ...guardPos };
  let currChar = grid[currPos.y]?.[currPos.x];
  const visitedPosList: Pos[] = [];

  while (currChar !== undefined) {
    currPos = getNextPos(currPos, currDir);
    currChar = grid[currPos.y]?.[currPos.x];

    if (
      visitedPosList.findIndex(
        ({ x, y }) => x === currPos.x && y === currPos.y,
      ) === -1
    ) {
      visitedPosList.push({ ...currPos });
    }

    const nextPos = getNextPos(currPos, currDir);
    if (grid[nextPos.y]?.[nextPos.x] === "#") {
      currDir++;
      if ((currDir as number) === 4) {
        currDir = Dir.North;
      }
    }
  }

  return visitedPosList.length - 1;
}

function getNextPos(currPos: Pos, currDir: Dir) {
  const newPos = { ...currPos };

  if (currDir === Dir.North) {
    newPos.y--;
  } else if (currDir === Dir.East) {
    newPos.x++;
  } else if (currDir === Dir.South) {
    newPos.y++;
  } else if (currDir === Dir.West) {
    newPos.x--;
  }

  return newPos;
}

function textIntoGrid(text: string) {
  return text
    .split("\n")
    .filter(Boolean)
    .map((line) => line.split(""));
}

function findPosition(grid: Grid): Pos | null {
  let pos = null;

  grid.forEach((_char, y) => {
    grid[y].forEach((char, x) => {
      if (char === "^") {
        pos = {
          x,
          y,
        };
      }
    });
  });

  return pos;
}
