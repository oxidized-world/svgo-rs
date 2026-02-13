import { expect, test } from "vitest";
import { optimize as optimizeSvgo } from "svgo";
import { runWithPlugins } from "./test-utils";

test("removeTitle removes <title>", () => {
  const input = "<svg><title>Icon</title><g/></svg>";
  const out = runWithPlugins(input, ["removeTitle"]);
  const expected = optimizeSvgo(input, { plugins: ["removeTitle"] }).data;
  expect(out).toBe(expected);
});

test("removeTitle removes multiple titles", () => {
  const input = "<svg><title>a</title><g><title>b</title></g><rect/></svg>";
  const out = runWithPlugins(input, ["removeTitle"]);
  const expected = optimizeSvgo(input, { plugins: ["removeTitle"] }).data;
  expect(out).toBe(expected);
});

test("removeTitle no-op when title absent", () => {
  const input = "<svg><desc>x</desc><g/></svg>";
  const out = runWithPlugins(input, ["removeTitle"]);
  const expected = optimizeSvgo(input, { plugins: ["removeTitle"] }).data;
  expect(out).toBe(expected);
});
