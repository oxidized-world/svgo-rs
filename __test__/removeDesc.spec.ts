import { expect, test } from "vitest";
import { optimize as optimizeSvgo } from "svgo";
import { runWithPlugins } from "./test-utils";

test("removeDesc default: removes empty and standard editor desc; keeps meaningful desc", () => {
  const input =
    "<svg>" +
    "<desc></desc>" +
    "<desc>Created with Something</desc>" +
    "<desc>Accessible description</desc>" +
    "</svg>";
  const out = runWithPlugins(input, ["removeDesc"]);
  expect(out).not.toContain("<desc></desc>");
  expect(out).not.toContain("Created with");
  expect(out).toContain("<desc>Accessible description</desc>");
});

test("removeDesc removeAny=true: removes all desc", () => {
  const input = "<svg><desc>Accessible description</desc><g/></svg>";
  const out = runWithPlugins(input, ["removeDesc"], {
    removeDescRemoveAny: true,
  });
  expect(out).not.toContain("<desc>");
  expect(out).toContain("<g/>");
});

test("removeDesc keeps meaningful desc by default (svgo parity)", () => {
  const input = "<svg><desc>Meaningful text</desc><g/></svg>";
  const out = runWithPlugins(input, ["removeDesc"]);
  const expected = optimizeSvgo(input, { plugins: ["removeDesc"] }).data;
  expect(out).toBe(expected);
});
