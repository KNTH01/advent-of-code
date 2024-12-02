// Learn more at https://docs.deno.com/runtime/manual/examples/module_metadata#concepts
if (import.meta.main) {
  // const inputFile = "input-base.txt";
  const inputFile = "input-1.txt";
  const text = await Deno.readTextFile(inputFile);
  const reports = text
    .split("\n")
    .filter(Boolean)
    .map((textReport) => textReport.split(" ").map(Number));

  const range = [1, 2, 3];
  const resPart1 = reports.filter((report) => {
    let isIncreasing = false;

    for (let i = 0; i < report.length; i++) {
      const a = report[i];
      const b = report[i + 1];
      const isInRange = range.includes(Math.abs(a - b));

      if (i === 0 && a < b) {
        isIncreasing = true;
      }

      if (i + 1 < report.length) {
        if (
          (i > 0 && ((a < b && !isIncreasing) || (a > b && isIncreasing))) ||
          !isInRange
        ) {
          return false;
        }
      }
    }
    return true;
  }).length;

  console.log("[LS] -> main.ts:11 -> restPart1: ", resPart1);
}
