import { expect, test } from "vitest";
import { optimize as optimizeSvgo } from "svgo";
import { runWithPlugins } from "./test-utils";

test("mergeStyles merges multiple style tags", () => {
  const input =
    '<svg><style>.a{fill:red}</style><style media="screen">.b{fill:blue}</style><rect class="a b"/></svg>';
  const out = runWithPlugins(input, ["mergeStyles"]);
  const expected = optimizeSvgo(input, { plugins: ["mergeStyles"] }).data;
  expect(out).toBe(expected);
});

test("mergeStyles skips foreignObject subtree", () => {
  const input =
    '<svg><foreignObject><style>.x{fill:red}</style></foreignObject><style>.a{fill:red}</style><style>.b{fill:blue}</style></svg>';
  const out = runWithPlugins(input, ["mergeStyles"]);
  const expected = optimizeSvgo(input, { plugins: ["mergeStyles"] }).data;
  expect(out).toBe(expected);
});
