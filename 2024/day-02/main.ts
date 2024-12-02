// Learn more at https://docs.deno.com/runtime/manual/examples/module_metadata#concepts
if (import.meta.main) {
  // const inputFile = "input-base.txt";
  const inputFile = "input-1.txt";
  const text = await Deno.readTextFile(inputFile);
  const reports = text
    .split("\n")
    .filter(Boolean)
    .map((textReport) => textReport.split(" ").map(Number));

  const resPart1 = reports.filter((report) => isSafe(report)).length;
  console.log("[LS] -> main.ts:11 -> restPart1: ", resPart1);

  const resPart2 = reports.filter((report) => {
    const isToleratedUnsafe = [...Array(report.length).keys()]
      .map((index) => {
        const testingReport = [...report];
        testingReport.splice(index, 1);

        return !isSafe(testingReport);
      })
      .every((unsafe) => unsafe === true);

    return isSafe(report) || !isToleratedUnsafe;
  }).length;
  console.log("[LS] -> main.ts:66 -> resPart2: ", resPart2);
}

function isInRange(a: number, b: number) {
  const range = [1, 2, 3];
  return range.includes(Math.abs(a - b));
}

function isSafe(report: number[]) {
  const swapCountAscending = findSwaps(
    report.map((val, index) => [index, val]).sort((a, b) => a[1] - b[1]),
  );
  const swapCountDescending = findSwaps(
    report.map((val, index) => [index, val]).sort((a, b) => b[1] - a[1]),
  );

  if (swapCountAscending > 0 && swapCountDescending > 0) {
    return false;
  }

  for (let i = 0; i < report.length; i++) {
    if (i + 1 < report.length) {
      const a = report[i];
      const b = report[i + 1];

      if (!isInRange(a, b)) {
        return false;
      }
    }
  }

  return true;
}

function findSwaps(arrPos: number[][]): number {
  const n = arrPos.length;
  let swaps = 0;
  const visited = new Array(n).fill(false);

  for (let i = 0; i < n; i++) {
    if (visited[i] || arrPos[i][0] === i) continue;

    let cycleSize = 0;
    let currentPos = i;

    while (!visited[currentPos]) {
      visited[currentPos] = true;
      currentPos = arrPos[currentPos][0];
      cycleSize++;
    }

    swaps += cycleSize - 1;
  }

  return swaps;
}
