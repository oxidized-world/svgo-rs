import { expect, test } from "vitest";
import { optimize as optimizeSvgo } from "svgo";
import { runWithPlugins } from "./test-utils";

test("removeEmptyAttrs removes empty attrs except conditional ones", () => {
  const input = '<svg><rect fill="" requiredFeatures=""/></svg>';
  const out = runWithPlugins(input, ["removeEmptyAttrs"]);
  const expected = optimizeSvgo(input, { plugins: ["removeEmptyAttrs"] }).data;
  expect(out).toBe(expected);
});

test("removeEmptyAttrs keeps requiredExtensions and systemLanguage", () => {
  const input = '<svg><rect requiredExtensions="" systemLanguage="" fill=""/></svg>';
  const out = runWithPlugins(input, ["removeEmptyAttrs"]);
  const expected = optimizeSvgo(input, { plugins: ["removeEmptyAttrs"] }).data;
  expect(out).toBe(expected);
});

test("removeEmptyAttrs keeps non-empty attrs", () => {
  const input = '<svg><rect fill="red" stroke=""/></svg>';
  const out = runWithPlugins(input, ["removeEmptyAttrs"]);
  const expected = optimizeSvgo(input, { plugins: ["removeEmptyAttrs"] }).data;
  expect(out).toBe(expected);
});
