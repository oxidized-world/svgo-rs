import { expect, test } from "vitest";
import { optimize as optimizeSvgo } from "svgo";
import { runWithPlugins } from "./test-utils";

test("cleanupListOfValues default", () => {
  const input = '<svg viewBox="0 0 200.28423 200.28423"><polygon points="208.250977,77.1308594 223.069336,98.4394531"/></svg>';
  const out = runWithPlugins(input, ["cleanupListOfValues"]);
  const expected = optimizeSvgo(input, { plugins: ["cleanupListOfValues"] }).data;
  expect(out).toBe(expected);
});

test("cleanupListOfValues options", () => {
  const input = '<svg><g enable-background="new 0 0 200.28423 200.28423" stroke-dasharray="1.25cm, 10"/></svg>';
  const out = runWithPlugins(input, ["cleanupListOfValues"], {
    cleanupListOfValuesFloatPrecision: 2,
    cleanupListOfValuesConvertToPx: true,
    cleanupListOfValuesDefaultPx: true,
    cleanupListOfValuesLeadingZero: true,
  });
  const expected = optimizeSvgo(input, {
    plugins: [
      {
        name: "cleanupListOfValues",
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
