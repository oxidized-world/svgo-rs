import { expect, test } from "vitest";
import { optimize as optimizeSvgo } from "svgo";
import { runWithPlugins } from "./test-utils";

test("cleanupEnableBackground removes when no filter", () => {
  const input = '<svg width="100" height="50" enable-background="new 0 0 100 50"><g/></svg>';
  const out = runWithPlugins(input, ["cleanupEnableBackground"]);
  const expected = optimizeSvgo(input, { plugins: ["cleanupEnableBackground"] }).data;
  expect(out).toBe(expected);
});

test("cleanupEnableBackground keeps/cleans with filter", () => {
  const input = '<svg width="100" height="50" enable-background="new 0 0 100 50"><filter id="f"/></svg>';
  const out = runWithPlugins(input, ["cleanupEnableBackground"]);
  const expected = optimizeSvgo(input, { plugins: ["cleanupEnableBackground"] }).data;
  expect(out).toBe(expected);
});

test("cleanupEnableBackground no-op without attribute", () => {
  const input = '<svg width="100" height="50"><filter id="f"/></svg>';
  const out = runWithPlugins(input, ["cleanupEnableBackground"]);
  const expected = optimizeSvgo(input, { plugins: ["cleanupEnableBackground"] }).data;
  expect(out).toBe(expected);
});
