import { expect, test } from "vitest";
import { optimize as optimizeSvgo } from "svgo";
import { runWithPlugins } from "./test-utils";

test("removeXMLNS removes xmlns attr", () => {
  const input = '<svg xmlns="http://www.w3.org/2000/svg"><rect/></svg>';
  const out = runWithPlugins(input, ["removeXMLNS"]);
  const expected = optimizeSvgo(input, { plugins: ["removeXMLNS"] }).data;
  expect(out).toBe(expected);
});

test("removeXMLNS keeps non-root xmlns attrs", () => {
  const input = '<svg xmlns="http://www.w3.org/2000/svg"><g xmlns="http://www.w3.org/2000/svg"><rect/></g></svg>';
  const out = runWithPlugins(input, ["removeXMLNS"]);
  const expected = optimizeSvgo(input, { plugins: ["removeXMLNS"] }).data;
  expect(out).toBe(expected);
});

test("removeXMLNS no-op when xmlns absent", () => {
  const input = '<svg><rect/></svg>';
  const out = runWithPlugins(input, ["removeXMLNS"]);
  const expected = optimizeSvgo(input, { plugins: ["removeXMLNS"] }).data;
  expect(out).toBe(expected);
});
