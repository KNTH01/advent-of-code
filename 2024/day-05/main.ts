// Learn more at https://docs.deno.com/runtime/manual/examples/module_metadata#concepts
if (import.meta.main) {
  // const inputFile = "input-base.txt";
  const inputFile = "input.txt";
  const text = await Deno.readTextFile(inputFile);

  const input = text.split("\n");
  input.pop();

  const index = input.findIndex((item) => !item);

  const rules = input.slice(0, index).map(toRule);
  console.log("[LS] -> main.ts:12 -> rules: ", rules);

  const updates = input
    .slice(index + 1)
    .map((v) => v.split(",").map(Number) as Update);

  const res1 = part1(rules, updates);
  console.log("[LS] -> main.ts:19 -> res1: ", res1);

  const res2 = part2(rules, updates);
  console.log("[LS] -> main.ts:21 -> res2: ", res2);
}

function part1(rules: Rule[], updates: Update[]) {
  return updates
    .map((update) => {
      let count = 0;
      for (let i = 0; i < update.length; i++) {
        const ruleLookup = [update[i], update[i + 1]];
        if (
          update[i + 1] &&
          rules.some(
            // TODO: can optimize perf here
            (rule) => JSON.stringify(rule) === JSON.stringify(ruleLookup),
          )
        ) {
          count++;
        }
      }

      count++;

      return count === update.length ? update : undefined;
    })
    .filter((update) => !!update)
    .map((update) => {
      const index = Math.round(update.length / 2);
      return update[index - 1];
    })
    .reduce((acc, curr) => acc + curr, 0);
}

function part2(rules: Rule[], updates: Update[]) {
  return updates
    .map((update) => {
      let hasSwaped = false;
      let didUpdateSwap = false;

      do {
        hasSwaped = false;

        for (let i = 0; i < update.length; i++) {
          const ruleLookup = [update[i], update[i + 1]];
          const foundRule =
            rules.findIndex(
              (rule) => JSON.stringify(rule) === JSON.stringify(ruleLookup),
            ) > -1;

          if (update[i + 1] && !foundRule) {
            [update[i], update[i + 1]] = [update[i + 1], update[i]];
            hasSwaped = true;
            didUpdateSwap = true;
          }
        }
      } while (hasSwaped);

      return {
        update,
        didUpdateSwap,
      };
    })
    .filter((v) => !!v.didUpdateSwap)
    .map(({ update }) => {
      const index = Math.round(update.length / 2);
      return update[index - 1];
    })
    .reduce((acc, curr) => acc + curr, 0);
}

type Rule = [number, number];
type Update = [number];

function toRule(input: string) {
  return input
    .split("|")
    .filter((v) => v.length === 2)
    .map(Number) as Rule;
}
