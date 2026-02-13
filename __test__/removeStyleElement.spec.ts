import { expect, test } from "vitest";
import { optimize as optimizeSvgo } from "svgo";
import { runWithPlugins } from "./test-utils";

test("removeStyleElement removes style node", () => {
  const input = '<svg><style>.a{fill:red}</style><rect class="a"/></svg>';
  const out = runWithPlugins(input, ["removeStyleElement"]);
  const expected = optimizeSvgo(input, { plugins: ["removeStyleElement"] }).data;
  expect(out).toBe(expected);
});

test("removeStyleElement removes multiple style nodes", () => {
  const input = '<svg><style>.a{fill:red}</style><g><style>.b{fill:blue}</style></g><rect/></svg>';
  const out = runWithPlugins(input, ["removeStyleElement"]);
  const expected = optimizeSvgo(input, { plugins: ["removeStyleElement"] }).data;
  expect(out).toBe(expected);
});

test("removeStyleElement keeps non-style elements", () => {
  const input = '<svg><desc>x</desc><rect/></svg>';
  const out = runWithPlugins(input, ["removeStyleElement"]);
  const expected = optimizeSvgo(input, { plugins: ["removeStyleElement"] }).data;
  expect(out).toBe(expected);
});
