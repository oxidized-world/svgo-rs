import { expect, test } from "vitest";
import { optimize as optimizeSvgo } from "svgo";
import { runWithPlugins } from "./test-utils";

test("removeEmptyText removes empty text/tspan/tref", () => {
  const input = '<svg><text/><tspan/><tref/><text>ok</text></svg>';
  const out = runWithPlugins(input, ["removeEmptyText"]);
  const expected = optimizeSvgo(input, { plugins: ["removeEmptyText"] }).data;
  expect(out).toBe(expected);
});

test("removeEmptyText removes empty tspan", () => {
  const input = '<svg xmlns="http://www.w3.org/2000/svg"><g><tspan></tspan></g></svg>';
  const out = runWithPlugins(input, ["removeEmptyText"]);
  const expected = optimizeSvgo(input, { plugins: ["removeEmptyText"] }).data;
  expect(out).toBe(expected);
});

test("removeEmptyText removes tref without href", () => {
  const input = '<svg xmlns="http://www.w3.org/2000/svg"><text><tref/></text></svg>';
  const out = runWithPlugins(input, ["removeEmptyText"]);
  const expected = optimizeSvgo(input, { plugins: ["removeEmptyText"] }).data;
  expect(out).toBe(expected);
});
