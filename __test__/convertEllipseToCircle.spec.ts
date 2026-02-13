import { expect, test } from "vitest";
import { optimize as optimizeSvgo } from "svgo";
import { runWithPlugins } from "./test-utils";

test("convertEllipseToCircle converts equal radii ellipse", () => {
  const input = '<svg><ellipse rx="10" ry="10" cx="4" cy="5"/></svg>';
  const out = runWithPlugins(input, ["convertEllipseToCircle"]);
  const expected = optimizeSvgo(input, { plugins: ["convertEllipseToCircle"] }).data;
  expect(out).toBe(expected);
});

test("convertEllipseToCircle keeps eccentric ellipse", () => {
  const input = '<svg><ellipse rx="10" ry="11"/></svg>';
  const out = runWithPlugins(input, ["convertEllipseToCircle"]);
  const expected = optimizeSvgo(input, { plugins: ["convertEllipseToCircle"] }).data;
  expect(out).toBe(expected);
});

test("convertEllipseToCircle keeps ellipse with missing radii", () => {
  const input = '<svg><ellipse cx="10" cy="10"/></svg>';
  const out = runWithPlugins(input, ["convertEllipseToCircle"]);
  const expected = optimizeSvgo(input, { plugins: ["convertEllipseToCircle"] }).data;
  expect(out).toBe(expected);
});
