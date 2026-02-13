import { expect, test } from "vitest";
import { optimize as optimizeSvgo } from "svgo";
import { runWithPlugins } from "./test-utils";

test("removeAttributesBySelector removes attrs on matched nodes", () => {
  const input = '<svg><rect id="x" fill="#00ff00" stroke="#00ff00"/></svg>';
  const out = runWithPlugins(input, ["removeAttributesBySelector"], {
    removeAttributesBySelectorSelector: "#x",
    removeAttributesBySelectorAttributes: ["fill", "stroke"],
  });
  const expected = optimizeSvgo(input, {
    plugins: [{ name: "removeAttributesBySelector", params: { selector: "#x", attributes: ["fill", "stroke"] } }],
  }).data;
  expect(out).toBe(expected);
});

test("removeAttributesBySelector supports child combinator", () => {
  const input = '<svg><g id="a"><rect fill="red"/><path fill="red"/></g><g><rect fill="red"/></g></svg>';
  const out = runWithPlugins(input, ["removeAttributesBySelector"], {
    removeAttributesBySelectorSelector: "#a > rect",
    removeAttributesBySelectorAttributes: ["fill"],
  });
  const expected = optimizeSvgo(input, {
    plugins: [
      {
        name: "removeAttributesBySelector",
        params: { selector: "#a > rect", attributes: ["fill"] },
      },
    ],
  }).data;
  expect(out).toBe(expected);
});

test("removeAttributesBySelector supports adjacent sibling combinator", () => {
  const input = '<svg><rect class="a" fill="red"/><rect class="b" fill="red"/><rect class="b" fill="red"/></svg>';
  const out = runWithPlugins(input, ["removeAttributesBySelector"], {
    removeAttributesBySelectorSelector: ".a + .b",
    removeAttributesBySelectorAttributes: ["fill"],
  });
  const expected = optimizeSvgo(input, {
    plugins: [
      {
        name: "removeAttributesBySelector",
        params: { selector: ".a + .b", attributes: ["fill"] },
      },
    ],
  }).data;
  expect(out).toBe(expected);
});
