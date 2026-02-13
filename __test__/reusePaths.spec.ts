import { expect, test } from "vitest";
import { optimize as optimizeSvgo } from "svgo";
import { runWithPlugins } from "./test-utils";

test("reusePaths converts duplicate paths into defs/use", () => {
  const input = '<svg><path d="M0 0" fill="red"/><path d="M0 0" fill="red"/></svg>';
  const out = runWithPlugins(input, ["reusePaths"]);
  const expected = optimizeSvgo(input, { plugins: ["reusePaths"] }).data;
  expect(out).toBe(expected);
});

test("reusePaths handles mixed ids and transforms", () => {
  const input =
    '<svg xmlns="http://www.w3.org/2000/svg"><path id="test0" d="M 10,50 l 20,30 L 20,30"/><path transform="translate(10, 10)" d="M 10,50 c 20,30 40,50 60,70 C 20,30 40,50 60,70"/><path transform="translate(20, 20)" d="M 10,50 c 20,30 40,50 60,70 C 20,30 40,50 60,70"/><path d="M 10,50 c 20,30 40,50 60,70 C 20,30 40,50 60,70"/><path id="test1" d="M 10,50 l 20,30 L 20,30"/></svg>';
  const out = runWithPlugins(input, ["reusePaths"]);
  const expected = optimizeSvgo(input, { plugins: ["reusePaths"] }).data;
  expect(out).toBe(expected);
});

test("reusePaths keeps unique paths unchanged", () => {
  const input = '<svg><path d="M0 0"/><path d="M1 1"/></svg>';
  const out = runWithPlugins(input, ["reusePaths"]);
  const expected = optimizeSvgo(input, { plugins: ["reusePaths"] }).data;
  expect(out).toBe(expected);
});
