import { expect, test } from "vitest";
import { optimize as optimizeSvgo } from "svgo";
import { runWithPlugins } from "./test-utils";

test("convertTransform trims simple transforms", () => {
  const input = '<svg><g transform="translate(10, 0) scale(1,1)"/></svg>';
  const out = runWithPlugins(input, ["convertTransform"]);
  const expected = optimizeSvgo(input, { plugins: ["convertTransform"] }).data;
  expect(out).toBe(expected);
});

test("convertTransform removes identity matrix", () => {
  const input = '<svg><g transform="matrix(1,0,0,1,0,0)"/></svg>';
  const out = runWithPlugins(input, ["convertTransform"]);
  const expected = optimizeSvgo(input, { plugins: ["convertTransform"] }).data;
  expect(out).toBe(expected);
});

test("convertTransform normalizes rotate with center", () => {
  const input = '<svg><g transform="rotate(45, 10, 10)"/></svg>';
  const out = runWithPlugins(input, ["convertTransform"]);
  const expected = optimizeSvgo(input, { plugins: ["convertTransform"] }).data;
  expect(out).toBe(expected);
});
