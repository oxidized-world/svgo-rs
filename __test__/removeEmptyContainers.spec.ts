import { expect, test } from "vitest";
import { optimize as optimizeSvgo } from "svgo";
import { runWithPlugins } from "./test-utils";

test("removeEmptyContainers removes empty defs and dependent use", () => {
  const input = '<svg><defs id="d"/><use href="#d"/><g/></svg>';
  const out = runWithPlugins(input, ["removeEmptyContainers"]);
  const expected = optimizeSvgo(input, { plugins: ["removeEmptyContainers"] }).data;
  expect(out).toBe(expected);
});

test("removeEmptyContainers keeps pattern with attributes", () => {
  const input = '<svg><pattern id="p" width="10" height="10"/></svg>';
  const out = runWithPlugins(input, ["removeEmptyContainers"]);
  const expected = optimizeSvgo(input, { plugins: ["removeEmptyContainers"] }).data;
  expect(out).toBe(expected);
});

test("removeEmptyContainers keeps non-empty groups", () => {
  const input = '<svg><g><path d="M0 0"/></g></svg>';
  const out = runWithPlugins(input, ["removeEmptyContainers"]);
  const expected = optimizeSvgo(input, { plugins: ["removeEmptyContainers"] }).data;
  expect(out).toBe(expected);
});
