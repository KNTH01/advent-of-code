function part1(text: string) {
  // text: "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"

  const regex = /mul\((\d{1,3}),(\d{1,3})\)/g;

  const res = [...text.matchAll(regex)]
    .map((match) => ({
      x: Number(match[1]),
      y: Number(match[2]),
    }))
    .map(({ x, y }) => x * y)
    .reduce((acc, curr) => acc + curr, 0);

  return res;
}

if (import.meta.main) {
  const inputFile = "input.txt";
  const text = await Deno.readTextFile(inputFile);

  const res1 = part1(text);
  console.log("[LS] -> main.ts:21 -> res1: ", res1);
}
