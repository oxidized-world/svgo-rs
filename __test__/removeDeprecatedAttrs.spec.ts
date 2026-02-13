import { expect, test } from "vitest";
import { optimize as optimizeSvgo } from "svgo";
import { runWithPlugins } from "./test-utils";

test("removeDeprecatedAttrs removes deprecated safe attrs", () => {
  const input = '<svg version="1.1" baseProfile="full"><rect width="10" height="10"/></svg>';
  const out = runWithPlugins(input, ["removeDeprecatedAttrs"], {
    removeDeprecatedAttrsRemoveUnsafe: false,
  });
  const expected = optimizeSvgo(input, {
    plugins: [{ name: "removeDeprecatedAttrs", params: { removeUnsafe: false } }],
  }).data;
  expect(out).toBe(expected);
});

test("removeDeprecatedAttrs removes unsafe attrs when enabled", () => {
  const input = '<svg enable-background="new 0 0 10 10"><rect width="10" height="10"/></svg>';
  const out = runWithPlugins(input, ["removeDeprecatedAttrs"], {
    removeDeprecatedAttrsRemoveUnsafe: true,
  });
  const expected = optimizeSvgo(input, {
    plugins: [{ name: "removeDeprecatedAttrs", params: { removeUnsafe: true } }],
  }).data;
  expect(out).toBe(expected);
});

test("removeDeprecatedAttrs keeps attrs referenced in stylesheet selectors", () => {
  const input = '<svg version="1.1"><style><![CDATA[[version]{fill:red}]]></style><rect width="10" height="10"/></svg>';
  const out = runWithPlugins(input, ["removeDeprecatedAttrs"], {
    removeDeprecatedAttrsRemoveUnsafe: false,
  });
  const expected = optimizeSvgo(input, {
    plugins: [{ name: "removeDeprecatedAttrs", params: { removeUnsafe: false } }],
  }).data;
  expect(out).toBe(expected);
});
