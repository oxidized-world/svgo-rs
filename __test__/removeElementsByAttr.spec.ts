import { expect, test } from "vitest";
import { optimize as optimizeSvgo } from "svgo";
import { runWithPlugins } from "./test-utils";

test("removeElementsByAttr removes by id and class", () => {
  const input = '<svg><g id="rm"/><path class="a b"/><rect class="c"/></svg>';
  const out = runWithPlugins(input, ["removeElementsByAttr"], {
    removeElementsByAttrId: ["rm"],
    removeElementsByAttrClass: ["b"],
  });
  const expected = optimizeSvgo(input, {
    plugins: [{ name: "removeElementsByAttr", params: { id: ["rm"], class: ["b"] } }],
  }).data;
  expect(out).toBe(expected);
});

test("removeElementsByAttr removes by class only", () => {
  const input = '<svg><path class="a b"/><rect class="a"/><circle/></svg>';
  const out = runWithPlugins(input, ["removeElementsByAttr"], {
    removeElementsByAttrClass: ["a"],
  });
  const expected = optimizeSvgo(input, {
    plugins: [{ name: "removeElementsByAttr", params: { class: ["a"] } }],
  }).data;
  expect(out).toBe(expected);
});

test("removeElementsByAttr no-op when lists empty", () => {
  const input = '<svg><g id="x"/><path class="a"/></svg>';
  const out = runWithPlugins(input, ["removeElementsByAttr"]);
  const expected = optimizeSvgo(input, { plugins: ["removeElementsByAttr"] }).data;
  expect(out).toBe(expected);
});
