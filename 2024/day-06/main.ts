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

  let count = 0;
  let currDir = Dir.North;
  const currPos = { ...guardPos };
  let currChar = grid[currPos.y]?.[currPos.x];
  const visitedPosList: Pos[] = [];

  while (currChar !== undefined) {
    if (currDir === Dir.North) {
      currPos.y--;
    } else if (currDir === Dir.East) {
      currPos.x++;
    } else if (currDir === Dir.South) {
      currPos.y++;
    } else if (currDir === Dir.West) {
      currPos.x--;
    }

    currChar = grid[currPos.y]?.[currPos.x];
    if (
      visitedPosList.findIndex(
        ({ x, y }) => x === currPos.x && y === currPos.y,
      ) === -1
    ) {
      count++;
      visitedPosList.push({ ...currPos });
    }

    const newDir = nextPosObstacle(grid, currPos, currDir);
    if (newDir) {
      currDir++;
      if ((currDir as number) === 4) {
        currDir = Dir.North;
      }
    }
  }

  console.log("[LS] -> main.ts:28 -> count: ", count - 1);
}

function nextPosObstacle(grid: Grid, currPos: Pos, currDir: Dir) {
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

  if (grid[newPos.y]?.[newPos.x] === "#") {
    return true;
  }

  return false;
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
        console.log("abcabc");
        pos = {
          x,
          y,
        };
      }
    });
  });

  return pos;
}
