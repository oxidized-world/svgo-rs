import { expect, test } from "vitest";
import { optimize as optimizeSvgo } from "svgo";
import { runWithPlugins } from "./test-utils";

test("convertShapeToPath converts rect/line/poly", () => {
  const input =
    '<svg><rect x="1" y="2" width="3" height="4"/><line x1="0" y1="0" x2="10" y2="10"/><polygon points="0,0 10,0 10,10"/></svg>';
  const out = runWithPlugins(input, ["convertShapeToPath"]);
  const expected = optimizeSvgo(input, { plugins: ["convertShapeToPath"] }).data;
  expect(out).toBe(expected);
});

test("convertShapeToPath convertArcs option", () => {
  const input = '<svg><circle cx="10" cy="20" r="5"/><ellipse cx="4" cy="6" rx="2" ry="3"/></svg>';
  const out = runWithPlugins(input, ["convertShapeToPath"], {
    convertShapeToPathConvertArcs: true,
  });
  const expected = optimizeSvgo(input, {
    plugins: [{ name: "convertShapeToPath", params: { convertArcs: true } }],
  }).data;
  expect(out).toBe(expected);
});

test("convertShapeToPath keeps existing path element", () => {
  const input = '<svg><path d="M0 0L1 1"/></svg>';
  const out = runWithPlugins(input, ["convertShapeToPath"]);
  const expected = optimizeSvgo(input, { plugins: ["convertShapeToPath"] }).data;
  expect(out).toBe(expected);
});
