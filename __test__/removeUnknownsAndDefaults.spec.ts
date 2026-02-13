import { expect, test } from "vitest";
import { optimize as optimizeSvgo } from "svgo";
import { runWithPlugins } from "./test-utils";

test("removeUnknownsAndDefaults removes unknown attr", () => {
  const input = '<svg><rect foo="bar" width="10" height="10"/></svg>';
  const out = runWithPlugins(input, ["removeUnknownsAndDefaults"]);
  const expected = optimizeSvgo(input, { plugins: ["removeUnknownsAndDefaults"] }).data;
  expect(out).toBe(expected);
});

test("removeUnknownsAndDefaults keeps data and aria attrs by default", () => {
  const input = '<svg><rect data-x="1" aria-hidden="true" unknown="x" width="10" height="10"/></svg>';
  const out = runWithPlugins(input, ["removeUnknownsAndDefaults"]);
  const expected = optimizeSvgo(input, { plugins: ["removeUnknownsAndDefaults"] }).data;
  expect(out).toBe(expected);
});

test("removeUnknownsAndDefaults removes default presentation attrs", () => {
  const input = '<svg><path stroke="none" stroke-opacity="1" d="M0 0"/></svg>';
  const out = runWithPlugins(input, ["removeUnknownsAndDefaults"]);
  const expected = optimizeSvgo(input, { plugins: ["removeUnknownsAndDefaults"] }).data;
  expect(out).toBe(expected);
});

test("removeUnknownsAndDefaults respects keepRoleAttr option", () => {
  const input = '<svg><rect role="img" foo="x" width="10" height="10"/></svg>';
  const out = runWithPlugins(input, ["removeUnknownsAndDefaults"], {
    removeUnknownsAndDefaultsKeepRoleAttr: true,
  });
  const expected = optimizeSvgo(input, {
    plugins: [{ name: "removeUnknownsAndDefaults", params: { keepRoleAttr: true } }],
  }).data;
  expect(out).toBe(expected);
});

test("removeUnknownsAndDefaults can drop data and aria attrs", () => {
  const input = '<svg><rect data-id="a" aria-hidden="true" width="10" height="10"/></svg>';
  const out = runWithPlugins(input, ["removeUnknownsAndDefaults"], {
    removeUnknownsAndDefaultsKeepDataAttrs: false,
    removeUnknownsAndDefaultsKeepAriaAttrs: false,
  });
  const expected = optimizeSvgo(input, {
    plugins: [
      {
        name: "removeUnknownsAndDefaults",
        params: { keepDataAttrs: false, keepAriaAttrs: false },
      },
    ],
  }).data;
  expect(out).toBe(expected);
});
