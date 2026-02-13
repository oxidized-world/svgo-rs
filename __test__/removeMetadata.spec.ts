import { expect, test } from "vitest";
import { optimize as optimizeSvgo } from "svgo";
import { runWithPlugins } from "./test-utils";

test("removeMetadata removes <metadata>", () => {
  const input = "<svg><metadata><foo/></metadata><g/></svg>";
  const out = runWithPlugins(input, ["removeMetadata"]);
  const expected = optimizeSvgo(input, { plugins: ["removeMetadata"] }).data;
  expect(out).toBe(expected);
});

test("removeMetadata removes metadata with text content", () => {
  const input = "<svg><metadata>info</metadata><rect/></svg>";
  const out = runWithPlugins(input, ["removeMetadata"]);
  const expected = optimizeSvgo(input, { plugins: ["removeMetadata"] }).data;
  expect(out).toBe(expected);
});

test("removeMetadata no-op when metadata absent", () => {
  const input = "<svg><g/></svg>";
  const out = runWithPlugins(input, ["removeMetadata"]);
  const expected = optimizeSvgo(input, { plugins: ["removeMetadata"] }).data;
  expect(out).toBe(expected);
});
