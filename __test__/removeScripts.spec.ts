import { expect, test } from "vitest";
import { optimize as optimizeSvgo } from "svgo";
import { runWithPlugins } from "./test-utils";

test("removeScripts removes script and event attrs", () => {
  const input = '<svg><script>alert(1)</script><rect onclick="x()"/></svg>';
  const out = runWithPlugins(input, ["removeScripts"]);
  const expected = optimizeSvgo(input, { plugins: ["removeScripts"] }).data;
  expect(out).toBe(expected);
});

test("removeScripts clears javascript href on links", () => {
  const input = '<svg><a href="javascript:alert(1)"><text>go</text></a><a href="#ok"><text>ok</text></a></svg>';
  const out = runWithPlugins(input, ["removeScripts"]);
  const expected = optimizeSvgo(input, { plugins: ["removeScripts"] }).data;
  expect(out).toBe(expected);
});

test("removeScripts removes nested event attrs", () => {
  const input = '<svg><g><rect onload="x()" width="1" height="1"/></g></svg>';
  const out = runWithPlugins(input, ["removeScripts"]);
  const expected = optimizeSvgo(input, { plugins: ["removeScripts"] }).data;
  expect(out).toBe(expected);
});
