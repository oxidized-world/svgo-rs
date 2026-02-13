import { expect, test } from "vitest";
import { optimize as optimizeSvgo } from "svgo";
import { runWithPlugins } from "./test-utils";

test("sortAttrs sorts attributes", () => {
  const input = '<svg><rect stroke="blue" id="x" fill="red" width="10"/></svg>';
  const out = runWithPlugins(input, ["sortAttrs"]);
  const expected = optimizeSvgo(input, { plugins: ["sortAttrs"] }).data;
  expect(out).toBe(expected);
});

test("sortAttrs supports xmlnsOrder front", () => {
  const input =
    '<svg><g xmlns:xlink="http://www.w3.org/1999/xlink" id="a" xmlns="http://www.w3.org/2000/svg"/></svg>';
  const out = runWithPlugins(input, ["sortAttrs"], {
    sortAttrsXmlnsOrder: "front",
  });
  const expected = optimizeSvgo(input, {
    plugins: [{ name: "sortAttrs", params: { xmlnsOrder: "front" } }],
  }).data;
  expect(out).toBe(expected);
});

test("sortAttrs supports custom order", () => {
  const input = '<svg><rect fill="red" id="x" stroke="blue" width="10"/></svg>';
  const out = runWithPlugins(input, ["sortAttrs"], {
    sortAttrsOrder: ["id", "width", "fill", "stroke"],
  });
  const expected = optimizeSvgo(input, {
    plugins: [
      {
        name: "sortAttrs",
        params: { order: ["id", "width", "fill", "stroke"] },
      },
    ],
  }).data;
  expect(out).toBe(expected);
});
