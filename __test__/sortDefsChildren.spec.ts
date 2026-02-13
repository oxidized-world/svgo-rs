import { expect, test } from "vitest";
import { optimize as optimizeSvgo } from "svgo";
import { runWithPlugins } from "./test-utils";

test("sortDefsChildren sorts children in defs", () => {
  const input = '<svg><defs><g/><path/><path/></defs></svg>';
  const out = runWithPlugins(input, ["sortDefsChildren"]);
  const expected = optimizeSvgo(input, { plugins: ["sortDefsChildren"] }).data;
  expect(out).toBe(expected);
});

test("sortDefsChildren prefers frequently used ids first", () => {
  const input =
    '<svg><defs><linearGradient id="g1"/><linearGradient id="g2"/></defs><rect fill="url(#g2)"/><circle fill="url(#g2)"/><path fill="url(#g1)"/></svg>';
  const out = runWithPlugins(input, ["sortDefsChildren"]);
  const expected = optimizeSvgo(input, { plugins: ["sortDefsChildren"] }).data;
  expect(out).toBe(expected);
});

test("sortDefsChildren falls back to id length then lexical", () => {
  const input = '<svg><defs><g id="bb"/><g id="a"/><g id="cc"/></defs></svg>';
  const out = runWithPlugins(input, ["sortDefsChildren"]);
  const expected = optimizeSvgo(input, { plugins: ["sortDefsChildren"] }).data;
  expect(out).toBe(expected);
});
