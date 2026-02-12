import { expect, test } from "vitest";
import { optimize as optimizeSvgo } from "svgo";
import { runWithPlugins } from "./test-utils";

test("convertStyleToAttrs default", () => {
  const input = '<svg><g style="fill:#000;stroke:#fff;--x:1"/></svg>';
  const out = runWithPlugins(input, ["convertStyleToAttrs"]);
  const expected = optimizeSvgo(input, { plugins: ["convertStyleToAttrs"] }).data;
  expect(out).toBe(expected);
});

test("convertStyleToAttrs keepImportant", () => {
  const input = '<svg><g style="fill:#000!important;stroke:#fff"/></svg>';
  const out = runWithPlugins(input, ["convertStyleToAttrs"], {
    convertStyleToAttrsKeepImportant: true,
  });
  const expected = optimizeSvgo(input, {
    plugins: [{ name: "convertStyleToAttrs", params: { keepImportant: true } }],
  }).data;
  expect(out).toBe(expected);
});
