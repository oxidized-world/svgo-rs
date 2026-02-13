import { expect, test } from "vitest";
import { optimize as optimizeSvgo } from "svgo";
import { runWithPlugins } from "./test-utils";

test("mergePaths merges adjacent compatible paths", () => {
  const input = '<svg><path fill="red" d="M0 0L1 1"/><path fill="red" d="M2 2L3 3"/></svg>';
  const out = runWithPlugins(input, ["mergePaths"]);
  const expected = optimizeSvgo(input, { plugins: ["mergePaths"] }).data;
  expect(out).toBe(expected);
});

test("mergePaths keeps paths with different attrs", () => {
  const input = '<svg><path fill="red" d="M0 0"/><path fill="blue" d="L1 1"/></svg>';
  const out = runWithPlugins(input, ["mergePaths"]);
  const expected = optimizeSvgo(input, { plugins: ["mergePaths"] }).data;
  expect(out).toBe(expected);
});

test("mergePaths respects force option", () => {
  const input = '<svg><path d="M0 0"/><path d="M1 1" stroke="red"/></svg>';
  const out = runWithPlugins(input, ["mergePaths"], {
    mergePathsForce: true,
  });
  const expected = optimizeSvgo(input, {
    plugins: [{ name: "mergePaths", params: { force: true } }],
  }).data;
  expect(out).toBe(expected);
});
