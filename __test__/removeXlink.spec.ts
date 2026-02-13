import { expect, test } from "vitest";
import { optimize as optimizeSvgo } from "svgo";
import { runWithPlugins } from "./test-utils";

test("removeXlink converts xlink:href to href", () => {
  const input = '<svg xmlns:xlink="http://www.w3.org/1999/xlink"><use xlink:href="#a"/></svg>';
  const out = runWithPlugins(input, ["removeXlink"], { removeXlinkIncludeLegacy: false });
  const expected = optimizeSvgo(input, {
    plugins: [{ name: "removeXlink", params: { includeLegacy: false } }],
  }).data;
  expect(out).toBe(expected);
});

test("removeXlink converts xlink:show/title", () => {
  const input =
    '<svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" viewBox="0 0 50 50"><a xlink:href="https://duckduckgo.com" xlink:show="new" xlink:title="DuckDuckGo Homepage"><text x="0" y="10">uwu</text></a></svg>';
  const out = runWithPlugins(input, ["removeXlink"], { removeXlinkIncludeLegacy: false });
  const expected = optimizeSvgo(input, {
    plugins: [{ name: "removeXlink", params: { includeLegacy: false } }],
  }).data;
  expect(out).toBe(expected);
});

test("removeXlink keeps legacy xlink:href when includeLegacy is false and element is legacy", () => {
  const input = '<svg xmlns:xlink="http://www.w3.org/1999/xlink"><tref xlink:href="#t"/></svg>';
  const out = runWithPlugins(input, ["removeXlink"], { removeXlinkIncludeLegacy: false });
  const expected = optimizeSvgo(input, {
    plugins: [{ name: "removeXlink", params: { includeLegacy: false } }],
  }).data;
  expect(out).toBe(expected);
});
