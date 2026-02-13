import { expect, test } from "vitest";
import { optimize as optimizeSvgo } from "svgo";
import { runWithPlugins } from "./test-utils";

test("convertOneStopGradients keeps multi-stop gradients unchanged", () => {
  const input =
    '<svg><defs><linearGradient id="g"><stop offset="0" stop-color="red"/><stop offset="1" stop-color="blue"/></linearGradient></defs><rect fill="url(#g)"/></svg>';
  const out = runWithPlugins(input, ["convertOneStopGradients"]);
  const expected = optimizeSvgo(input, { plugins: ["convertOneStopGradients"] }).data;
  expect(out).toBe(expected);
});

test("convertOneStopGradients no-op without gradients", () => {
  const input = '<svg><rect fill="red"/></svg>';
  const out = runWithPlugins(input, ["convertOneStopGradients"]);
  const expected = optimizeSvgo(input, { plugins: ["convertOneStopGradients"] }).data;
  expect(out).toBe(expected);
});

test("convertOneStopGradients converts referenced one-stop gradient", () => {
  const input =
    '<svg><defs><linearGradient id="g"><stop offset="0" stop-color="red"/></linearGradient></defs><rect fill="url(#g)"/></svg>';
  const out = runWithPlugins(input, ["convertOneStopGradients"]);
  const expected = optimizeSvgo(input, { plugins: ["convertOneStopGradients"] }).data;
  expect(out).toBe(expected);
});
