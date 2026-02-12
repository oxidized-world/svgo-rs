import { expect, test } from "vitest";
import { optimize as optimizeSvgo } from "svgo";
import { runWithPlugins } from "./test-utils";

test("convertPathData minifies simple separators", () => {
  const input = '<svg><path d="M 0,0 L 10,10 L 20,20 z"/></svg>';
  const out = runWithPlugins(input, ["convertPathData"]);
  const expected = optimizeSvgo(input, { plugins: ["convertPathData"] }).data;
  expect(out).toBe(expected);
});

test("convertPathData keeps already compact path stable", () => {
  const input = '<svg><path d="M0 0H10V10z"/></svg>';
  const out = runWithPlugins(input, ["convertPathData"]);
  const expected = optimizeSvgo(input, { plugins: ["convertPathData"] }).data;
  expect(out).toBe(expected);
});
