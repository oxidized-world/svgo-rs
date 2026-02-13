import { expect, test } from "vitest";
import { optimize as optimizeSvgo } from "svgo";
import { runWithPlugins } from "./test-utils";

test("removeAttrs removes attrs by pattern", () => {
  const input = '<svg><path fill="red" stroke="blue" d="M0 0"/></svg>';
  const out = runWithPlugins(input, ["removeAttrs"], {
    removeAttrsAttrs: ["(fill|stroke)"],
  });
  const expected = optimizeSvgo(input, {
    plugins: [{ name: "removeAttrs", params: { attrs: "(fill|stroke)" } }],
  }).data;
  expect(out).toBe(expected);
});

test("removeAttrs respects preserveCurrentColor", () => {
  const input = '<svg><path fill="currentColor" stroke="currentColor" d="M0 0"/></svg>';
  const out = runWithPlugins(input, ["removeAttrs"], {
    removeAttrsAttrs: ["(fill|stroke)"],
    removeAttrsPreserveCurrentColor: true,
  });
  const expected = optimizeSvgo(input, {
    plugins: [
      {
        name: "removeAttrs",
        params: { attrs: "(fill|stroke)", preserveCurrentColor: true },
      },
    ],
  }).data;
  expect(out).toBe(expected);
});

test("removeAttrs supports element-prefixed pattern", () => {
  const input = '<svg><path fill="red" d="M0 0"/><rect fill="red" width="1" height="1"/></svg>';
  const out = runWithPlugins(input, ["removeAttrs"], {
    removeAttrsAttrs: ["path:fill"],
  });
  const expected = optimizeSvgo(input, {
    plugins: [{ name: "removeAttrs", params: { attrs: "path:fill" } }],
  }).data;
  expect(out).toBe(expected);
});
