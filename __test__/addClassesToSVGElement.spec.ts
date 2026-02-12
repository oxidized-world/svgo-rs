import { expect, test } from "vitest";
import { optimize as optimizeSvgo } from "svgo";
import { runWithPlugins } from "./test-utils";

test("addClassesToSVGElement className", () => {
  const input = "<svg class=\"base\"><g/></svg>";
  const out = runWithPlugins(input, ["addClassesToSVGElement"], {
    addClassesToSvgElementClassName: "mySvg",
  });
  const expected = optimizeSvgo(input, {
    plugins: [
      {
        name: "addClassesToSVGElement",
        params: { className: "mySvg" },
      },
    ],
  }).data;
  expect(out).toBe(expected);
});

test("addClassesToSVGElement classNames", () => {
  const input = "<svg><g/></svg>";
  const out = runWithPlugins(input, ["addClassesToSVGElement"], {
    addClassesToSvgElementClassNames: ["a", "b", "a"],
  });
  const expected = optimizeSvgo(input, {
    plugins: [
      {
        name: "addClassesToSVGElement",
        params: { classNames: ["a", "b", "a"] },
      },
    ],
  }).data;
  expect(out).toBe(expected);
});
