import { expect, test } from "vitest";
import { optimize as optimizeSvgo } from "svgo";
import { runWithPlugins } from "./test-utils";

test("removeRasterImages removes image with raster href", () => {
  const input = '<svg xmlns:xlink="http://www.w3.org/1999/xlink"><image xlink:href="foo.jpg"/><image xlink:href="foo.svg"/></svg>';
  const out = runWithPlugins(input, ["removeRasterImages"]);
  const expected = optimizeSvgo(input, { plugins: ["removeRasterImages"] }).data;
  expect(out).toBe(expected);
});

test("removeRasterImages removes data uri png image", () => {
  const input = '<svg><image href="data:image/png;base64,aaaa"/><image href="data:image/svg+xml;base64,bbbb"/></svg>';
  const out = runWithPlugins(input, ["removeRasterImages"]);
  const expected = optimizeSvgo(input, { plugins: ["removeRasterImages"] }).data;
  expect(out).toBe(expected);
});

test("removeRasterImages no-op for vector refs", () => {
  const input = '<svg><image href="icon.svg"/><image href="#symbol"/></svg>';
  const out = runWithPlugins(input, ["removeRasterImages"]);
  const expected = optimizeSvgo(input, { plugins: ["removeRasterImages"] }).data;
  expect(out).toBe(expected);
});
