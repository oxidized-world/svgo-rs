import { expect, test } from "vitest";
import { optimize as optimizeSvgo } from "svgo";
import { runWithPlugins } from "./test-utils";

test("removeOffCanvasPaths removes path outside viewBox", () => {
  const input = '<svg viewBox="0 0 10 10"><path d="M100 100 L120 120"/><path d="M1 1 L2 2"/></svg>';
  const out = runWithPlugins(input, ["removeOffCanvasPaths"]);
  const expected = optimizeSvgo(input, { plugins: ["removeOffCanvasPaths"] }).data;
  expect(out).toBe(expected);
});

test("removeOffCanvasPaths handles relative path commands", () => {
  const input =
    '<svg viewBox="0 0 50 50"><path d="M10 -90 h80 v80 h-80"/><path d="M10 10 h20 v20 h-20"/></svg>';
  const out = runWithPlugins(input, ["removeOffCanvasPaths"]);
  const expected = optimizeSvgo(input, { plugins: ["removeOffCanvasPaths"] }).data;
  expect(out).toBe(expected);
});

test("removeOffCanvasPaths keeps transformed path", () => {
  const input = '<svg viewBox="0 0 10 10"><path transform="translate(-100,-100)" d="M100 100 L120 120"/></svg>';
  const out = runWithPlugins(input, ["removeOffCanvasPaths"]);
  const expected = optimizeSvgo(input, { plugins: ["removeOffCanvasPaths"] }).data;
  expect(out).toBe(expected);
});
