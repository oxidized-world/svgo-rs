import { expect, test } from "vitest";
import { optimize as optimizeSvgo } from "svgo";
import { runWithPlugins } from "./test-utils";

test("moveGroupAttrsToElems moves transform to children", () => {
  const input = '<svg><g transform="scale(2)"><path d="M0 0"/><path transform="rotate(10)" d="M1 1"/></g></svg>';
  const out = runWithPlugins(input, ["moveGroupAttrsToElems"]);
  const expected = optimizeSvgo(input, { plugins: ["moveGroupAttrsToElems"] }).data;
  expect(out).toBe(expected);
});

test("moveGroupAttrsToElems keeps group when child has id", () => {
  const input = '<svg><g transform="scale(2)"><path id="p" d="M0 0"/></g></svg>';
  const out = runWithPlugins(input, ["moveGroupAttrsToElems"]);
  const expected = optimizeSvgo(input, { plugins: ["moveGroupAttrsToElems"] }).data;
  expect(out).toBe(expected);
});

test("moveGroupAttrsToElems keeps group with url reference attrs", () => {
  const input = '<svg><g transform="scale(2)" clip-path="url(#c)"><path d="M0 0"/></g></svg>';
  const out = runWithPlugins(input, ["moveGroupAttrsToElems"]);
  const expected = optimizeSvgo(input, { plugins: ["moveGroupAttrsToElems"] }).data;
  expect(out).toBe(expected);
});
