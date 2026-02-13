import { expect, test } from "vitest";
import { optimize as optimizeSvgo } from "svgo";
import { runWithPlugins } from "./test-utils";

test("removeNonInheritableGroupAttrs removes non-inheritable attrs from g", () => {
  const input = '<svg><g clip-rule="evenodd" fill="red"><path d="M0 0"/></g></svg>';
  const out = runWithPlugins(input, ["removeNonInheritableGroupAttrs"]);
  const expected = optimizeSvgo(input, { plugins: ["removeNonInheritableGroupAttrs"] }).data;
  expect(out).toBe(expected);
});

test("removeNonInheritableGroupAttrs keeps group exceptions", () => {
  const input = '<svg><g clip-path="url(#c)" filter="url(#f)" mask="url(#m)"><path d="M0 0"/></g></svg>';
  const out = runWithPlugins(input, ["removeNonInheritableGroupAttrs"]);
  const expected = optimizeSvgo(input, { plugins: ["removeNonInheritableGroupAttrs"] }).data;
  expect(out).toBe(expected);
});

test("removeNonInheritableGroupAttrs keeps non-presentation attrs", () => {
  const input = '<svg><g data-x="1" aria-label="a" fill="red"><path d="M0 0"/></g></svg>';
  const out = runWithPlugins(input, ["removeNonInheritableGroupAttrs"]);
  const expected = optimizeSvgo(input, { plugins: ["removeNonInheritableGroupAttrs"] }).data;
  expect(out).toBe(expected);
});
