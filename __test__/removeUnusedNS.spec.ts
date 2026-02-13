import { expect, test } from "vitest";
import { optimize as optimizeSvgo } from "svgo";
import { runWithPlugins } from "./test-utils";

test("removeUnusedNS removes unused xmlns prefixes", () => {
  const input = '<svg xmlns:xlink="http://www.w3.org/1999/xlink"><rect width="10" height="10"/></svg>';
  const out = runWithPlugins(input, ["removeUnusedNS"]);
  const expected = optimizeSvgo(input, { plugins: ["removeUnusedNS"] }).data;
  expect(out).toBe(expected);
});

test("removeUnusedNS keeps used namespace prefixes", () => {
  const input = '<svg xmlns:xlink="http://www.w3.org/1999/xlink"><use xlink:href="#a"/></svg>';
  const out = runWithPlugins(input, ["removeUnusedNS"]);
  const expected = optimizeSvgo(input, { plugins: ["removeUnusedNS"] }).data;
  expect(out).toBe(expected);
});

test("removeUnusedNS handles multiple prefixes", () => {
  const input = '<svg xmlns:xlink="http://www.w3.org/1999/xlink" xmlns:foo="http://example.com/foo"><foo:bar/><rect/></svg>';
  const out = runWithPlugins(input, ["removeUnusedNS"]);
  const expected = optimizeSvgo(input, { plugins: ["removeUnusedNS"] }).data;
  expect(out).toBe(expected);
});
