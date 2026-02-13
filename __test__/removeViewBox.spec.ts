import { expect, test } from "vitest";
import { optimize as optimizeSvgo } from "svgo";
import { runWithPlugins } from "./test-utils";

test("removeViewBox removes matched viewBox", () => {
  const input = '<svg width="100" height="50" viewBox="0 0 100 50"><rect/></svg>';
  const out = runWithPlugins(input, ["removeViewBox"]);
  const expected = optimizeSvgo(input, { plugins: ["removeViewBox"] }).data;
  expect(out).toBe(expected);
});

test("removeViewBox handles px dimensions", () => {
  const input = '<svg width="100px" height="50px" viewBox="0 0 100 50"><rect/></svg>';
  const out = runWithPlugins(input, ["removeViewBox"]);
  const expected = optimizeSvgo(input, { plugins: ["removeViewBox"] }).data;
  expect(out).toBe(expected);
});

test("removeViewBox keeps non-matching viewBox", () => {
  const input = '<svg width="100" height="50" viewBox="0 0 200 50"><rect/></svg>';
  const out = runWithPlugins(input, ["removeViewBox"]);
  const expected = optimizeSvgo(input, { plugins: ["removeViewBox"] }).data;
  expect(out).toBe(expected);
});
