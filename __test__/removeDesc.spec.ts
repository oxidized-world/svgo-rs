import { expect, test } from "vitest";
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
