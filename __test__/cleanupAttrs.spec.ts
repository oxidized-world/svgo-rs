import { expect, test } from "vitest";
import { optimize as optimizeSvgo } from "svgo";
import { runWithPlugins } from "./test-utils";

test("cleanupAttrs default", () => {
  const input = '<svg><g data-v="  a\n\n   b  "/></svg>';
  const out = runWithPlugins(input, ["cleanupAttrs"]);
  const expected = optimizeSvgo(input, {
    plugins: ["cleanupAttrs"],
  }).data;
  expect(out).toBe(expected);
});

test("cleanupAttrs options", () => {
  const input = '<svg><g data-v="  a\n\n   b  "/></svg>';
  const out = runWithPlugins(input, ["cleanupAttrs"], {
    cleanupAttrsNewlines: false,
    cleanupAttrsTrim: false,
    cleanupAttrsSpaces: true,
  });
  const expected = optimizeSvgo(input, {
    plugins: [
      {
        name: "cleanupAttrs",
        params: { newlines: false, trim: false, spaces: true },
      },
    ],
  }).data;
  expect(out).toBe(expected);
});
