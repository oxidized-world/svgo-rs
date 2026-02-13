import { expect, test } from "vitest";
import { optimize as optimizeSvgo } from "svgo";
import { runWithPlugins } from "./test-utils";

test("removeDoctype removes doctype", () => {
  const input =
    '<!DOCTYPE svg PUBLIC "-//W3C//DTD SVG 1.1//EN" "http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd"><svg/>';
  const out = runWithPlugins(input, ["removeDoctype"]);
  const expected = optimizeSvgo(input, { plugins: ["removeDoctype"] }).data;
  expect(out).toBe(expected);
});

test("removeDoctype no-op when doctype absent", () => {
  const input = "<svg><g/></svg>";
  const out = runWithPlugins(input, ["removeDoctype"]);
  const expected = optimizeSvgo(input, { plugins: ["removeDoctype"] }).data;
  expect(out).toBe(expected);
});

test("removeDoctype keeps xml declaration handling to other plugins", () => {
  const input = '<?xml version="1.0"?><!DOCTYPE svg><svg/>';
  const out = runWithPlugins(input, ["removeDoctype"]);
  const expected = optimizeSvgo(input, { plugins: ["removeDoctype"] }).data;
  expect(out).toBe(expected);
});
