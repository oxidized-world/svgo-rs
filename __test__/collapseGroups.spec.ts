import { expect, test } from "vitest";
import { optimize as optimizeSvgo } from "svgo";
import { runWithPlugins } from "./test-utils";

test("collapseGroups collapses empty nested groups", () => {
  const input = "<svg><g><g><path d=\"M0 0\"/></g></g></svg>";
  const out = runWithPlugins(input, ["collapseGroups"]);
  const expected = optimizeSvgo(input, { plugins: ["collapseGroups"] }).data;
  expect(out).toBe(expected);
});

test("collapseGroups moves attributes to single child", () => {
  const input = '<svg><g fill="red"><path d="M0 0"/></g></svg>';
  const out = runWithPlugins(input, ["collapseGroups"]);
  const expected = optimizeSvgo(input, { plugins: ["collapseGroups"] }).data;
  expect(out).toBe(expected);
});

test("collapseGroups preserves group when filter is present", () => {
  const input =
    '<svg xmlns="http://www.w3.org/2000/svg"><filter id="b"/><g filter="url(#b)"><g clip-path="url(#a)"><circle cx="30" cy="10" r="10"/></g></g></svg>';
  const out = runWithPlugins(input, ["collapseGroups"]);
  const expected = optimizeSvgo(input, { plugins: ["collapseGroups"] }).data;
  expect(out).toBe(expected);
});

test("collapseGroups preserves class-sensitive grouping", () => {
  const input =
    '<svg xmlns="http://www.w3.org/2000/svg"><style>.n{display:none}.i{display:inline}</style><g id="a"><g class="i"/></g><g id="b" class="n"><g class="i"/></g></svg>';
  const out = runWithPlugins(input, ["collapseGroups"]);
  const expected = optimizeSvgo(input, { plugins: ["collapseGroups"] }).data;
  expect(out).toBe(expected);
});
