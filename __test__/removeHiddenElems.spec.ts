import { expect, test } from "vitest";
import { optimize as optimizeSvgo } from "svgo";
import { runWithPlugins } from "./test-utils";

test("removeHiddenElems removes display none", () => {
  const input = '<svg><rect display="none" width="10" height="10"/><rect width="5" height="5"/></svg>';
  const out = runWithPlugins(input, ["removeHiddenElems"]);
  const expected = optimizeSvgo(input, { plugins: ["removeHiddenElems"] }).data;
  expect(out).toBe(expected);
});

test("removeHiddenElems removes zero-size rect", () => {
  const input = '<svg xmlns="http://www.w3.org/2000/svg"><g><rect width="0"/><rect height="0"/></g></svg>';
  const out = runWithPlugins(input, ["removeHiddenElems"]);
  const expected = optimizeSvgo(input, { plugins: ["removeHiddenElems"] }).data;
  expect(out).toBe(expected);
});

test("removeHiddenElems keeps hidden group with visible descendant override", () => {
  const input =
    '<svg width="480" height="360" xmlns="http://www.w3.org/2000/svg"><style>.a{visibility:visible}</style><g visibility="hidden"><rect x="196" y="196" width="96" height="96" fill="lime" visibility="visible"/></g><rect x="96" y="96" width="96" height="96" visibility="hidden" class="a"/></svg>';
  const out = runWithPlugins(input, ["removeHiddenElems"]);
  const expected = optimizeSvgo(input, { plugins: ["removeHiddenElems"] }).data;
  expect(out).toBe(expected);
});
