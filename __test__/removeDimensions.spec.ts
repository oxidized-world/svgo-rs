import { expect, test } from "vitest";
import { optimize as optimizeSvgo } from "svgo";
import { runWithPlugins } from "./test-utils";

test("removeDimensions drops width/height when viewBox exists", () => {
  const input = '<svg width="100" height="50" viewBox="0 0 100 50"><rect/></svg>';
  const out = runWithPlugins(input, ["removeDimensions"]);
  const expected = optimizeSvgo(input, { plugins: ["removeDimensions"] }).data;
  expect(out).toBe(expected);
});

test("removeDimensions creates viewBox from numeric width/height", () => {
  const input = '<svg width="100" height="50"><rect/></svg>';
  const out = runWithPlugins(input, ["removeDimensions"]);
  const expected = optimizeSvgo(input, { plugins: ["removeDimensions"] }).data;
  expect(out).toBe(expected);
});

test("removeDimensions handles px width/height", () => {
  const input = '<svg width="100px" height="50px"><rect/></svg>';
  const out = runWithPlugins(input, ["removeDimensions"]);
  const expected = optimizeSvgo(input, { plugins: ["removeDimensions"] }).data;
  expect(out).toBe(expected);
});
