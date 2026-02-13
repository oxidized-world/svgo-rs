import { expect, test } from "vitest";
import { optimize as optimizeSvgo } from "svgo";
import { runWithPlugins } from "./test-utils";

test("removeUselessStrokeAndFill removes useless stroke attrs", () => {
  const input = '<svg><path stroke="none" stroke-width="2" d="M0 0"/></svg>';
  const out = runWithPlugins(input, ["removeUselessStrokeAndFill"]);
  const expected = optimizeSvgo(input, { plugins: ["removeUselessStrokeAndFill"] }).data;
  expect(out).toBe(expected);
});

test("removeUselessStrokeAndFill removes useless fill attrs", () => {
  const input = '<svg><path fill="none" fill-opacity="0.5" fill-rule="evenodd" d="M0 0"/></svg>';
  const out = runWithPlugins(input, ["removeUselessStrokeAndFill"]);
  const expected = optimizeSvgo(input, { plugins: ["removeUselessStrokeAndFill"] }).data;
  expect(out).toBe(expected);
});

test("removeUselessStrokeAndFill respects removeNone option", () => {
  const input = '<svg><path stroke="none" fill="none" d="M0 0"/></svg>';
  const out = runWithPlugins(input, ["removeUselessStrokeAndFill"], {
    removeUselessStrokeAndFillRemoveNone: true,
  });
  const expected = optimizeSvgo(input, {
    plugins: [{ name: "removeUselessStrokeAndFill", params: { removeNone: true } }],
  }).data;
  expect(out).toBe(expected);
});
