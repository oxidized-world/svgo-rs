import { expect, test } from "vitest";
import { optimize as optimizeSvgo } from "svgo";
import { runWithPlugins } from "./test-utils";

test("cleanupNumericValues default", () => {
  const input = '<svg><rect width="10.12345px" x="0.5000" version="1.1"/></svg>';
  const out = runWithPlugins(input, ["cleanupNumericValues"]);
  const expected = optimizeSvgo(input, { plugins: ["cleanupNumericValues"] }).data;
  expect(out).toBe(expected);
});

test("cleanupNumericValues options", () => {
  const input = '<svg viewBox="0 0 200.28423 200.28423"><rect width="1.25cm"/></svg>';
  const out = runWithPlugins(input, ["cleanupNumericValues"], {
    cleanupNumericValuesFloatPrecision: 2,
    cleanupNumericValuesConvertToPx: true,
    cleanupNumericValuesDefaultPx: true,
    cleanupNumericValuesLeadingZero: true,
  });
  const expected = optimizeSvgo(input, {
    plugins: [
      {
        name: "cleanupNumericValues",
        params: {
          floatPrecision: 2,
          convertToPx: true,
          defaultPx: true,
          leadingZero: true,
        },
      },
    ],
  }).data;
  expect(out).toBe(expected);
});

test("cleanupNumericValues handles scientific notation", () => {
  const input = '<svg><rect width="1e2" height="5e1"/></svg>';
  const out = runWithPlugins(input, ["cleanupNumericValues"]);
  const expected = optimizeSvgo(input, { plugins: ["cleanupNumericValues"] }).data;
  expect(out).toBe(expected);
});
