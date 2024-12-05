if (import.meta.main) {
  // const inputFile = "input-base.txt";
  const inputFile = "input.txt";
  const text = await Deno.readTextFile(inputFile);

  const input = text.split("\n");
  input.pop();

  const index = input.findIndex((item) => !item);

  const rules = input.slice(0, index);

  const updates = input
    .slice(index + 1)
    .map((v) => v.split(",").map(Number) as Update);

  const res1 = part1(rules, updates);
  console.log("[LS] -> main.ts:19 -> res1: ", res1);

  const res2 = part2(rules, updates);
  console.log("[LS] -> main.ts:21 -> res2: ", res2);
}

type Rule = [number, number];
type Update = [number];

function part1(rules: string[], updates: Update[]) {
  return updates
    .map((update) => {
      let count = 0;
      for (let i = 0; i < update.length; i++) {
        const ruleToLookup: Rule = [update[i], update[i + 1]];

        if (
          update[i + 1] &&
          rules.some((rule) => rule === ruleToLookup.join("|"))
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

function part2(rules: string[], updates: Update[]) {
  return updates
    .map((update) => {
      let hasSwaped = false;
      let didUpdateSwap = false;

      do {
        hasSwaped = false;

        for (let i = 0; i < update.length; i++) {
          const ruleToLookup: Rule = [update[i], update[i + 1]];
          const foundRule =
            rules.findIndex((rule) => rule === ruleToLookup.join("|")) > -1;

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
