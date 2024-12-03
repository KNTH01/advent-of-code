function extractContent(text: string) {
  const regex = /mul\((\d{1,3}),(\d{1,3})\)/g;

  return [...text.matchAll(regex)].map((match) => {
    return {
      x: Number(match[1]),
      y: Number(match[2]),
      index: match.index,
    };
  });
}

function part1(text: string) {
  const content = extractContent(text);

  const res = content
    .map(({ x, y }) => x * y)
    .reduce((acc, curr) => acc + curr, 0);

  return res;
}

function part2(text: string) {
  const stateRegex = /(?:do\(\)|don't\(\))/g;
  const states = [];
  let stateMatch;
  while ((stateMatch = stateRegex.exec(text)) !== null) {
    states.push({
      type: stateMatch[0],
      pos: stateMatch.index,
      end: stateMatch.index + stateMatch[0].length,
    });
  }

  const matches = [];
  let currPos = 0;
  let enabled = true;

  for (const state of states) {
    const textPart = text.slice(currPos, state.pos);
    const content = extractContent(textPart);

    if (enabled) {
      matches.push(...content);
    }

    enabled = state.type === "do()";
    currPos = state.end;
  }

  if (enabled && currPos < text.length) {
    const textPart = text.slice(currPos);
    const content = extractContent(textPart);
    matches.push(...content);
  }

  const res = matches
    .map(({ x, y }) => x * y)
    .reduce((acc, curr) => acc + curr, 0);

  return res;
}

if (import.meta.main) {
  const inputFile = "input.txt";
  const text = await Deno.readTextFile(inputFile);
  // const text =
  //   "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

  const res1 = part1(text);
  console.log("[LS] -> main.ts:21 -> res1: ", res1);

  const res2 = part2(text);
  console.log("[LS] -> main.ts:37 -> res2: ", res2);
}
