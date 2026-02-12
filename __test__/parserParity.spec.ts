import { expect, test } from "vitest";
import { optimize as optimizeSvgo } from "svgo";
import { runWithPlugins } from "./test-utils";

test("parser trims whitespace text outside text elements", () => {
  const input = "<svg>\n  <g/>\n</svg>";
  const out = runWithPlugins(input, []);
  const expected = optimizeSvgo(input, { plugins: [] }).data;
  expect(out).toBe(expected);
});

test("parser keeps whitespace inside text elements", () => {
  const input = "<svg><text>\n  hi\n</text></svg>";
  const out = runWithPlugins(input, []);
  const expected = optimizeSvgo(input, { plugins: [] }).data;
  expect(out).toBe(expected);
});

test("parser supports internal doctype entities", () => {
  const input = '<!DOCTYPE svg [<!ENTITY x "ABC">]><svg><text>&x;</text></svg>';
  const out = runWithPlugins(input, []);
  const expected = optimizeSvgo(input, { plugins: [] }).data;
  expect(out).toBe(expected);
});

test("parser trims comment node value", () => {
  const input = "<svg><!--  hi  --></svg>";
  const out = runWithPlugins(input, []);
  const expected = optimizeSvgo(input, { plugins: [] }).data;
  expect(out).toBe(expected);
});

test("parser rejects non-whitespace before first tag", () => {
  expect(() => runWithPlugins("x<svg/>", [])).toThrow();
});
