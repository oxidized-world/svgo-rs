import { expect, test } from "vitest";
import { optimize as optimizeSvgo } from "svgo";
import { runWithPlugins } from "./test-utils";

test("convertColors default", () => {
  const input = '<svg><rect fill="rgb(255, 0, 0)" stroke="navy"/></svg>';
  const out = runWithPlugins(input, ["convertColors"]);
  const expected = optimizeSvgo(input, { plugins: ["convertColors"] }).data;
  expect(out).toBe(expected);
});

test("convertColors options", () => {
  const input = '<svg><rect fill="#FF0000" stroke="rgb(0, 0, 255)"/></svg>';
  const out = runWithPlugins(input, ["convertColors"], {
    convertColorsCurrentColorEnabled: false,
    convertColorsNames2hex: true,
    convertColorsRgb2hex: true,
    convertColorsConvertCase: "lower",
    convertColorsShorthex: true,
    convertColorsShortname: true,
  });
  const expected = optimizeSvgo(input, {
    plugins: [
      {
        name: "convertColors",
        params: {
          currentColor: false,
          names2hex: true,
          rgb2hex: true,
          convertCase: "lower",
          shorthex: true,
          shortname: true,
        },
      },
    ],
  }).data;
  expect(out).toBe(expected);
});

test("convertColors supports currentColor string option", () => {
  const input = '<svg><rect fill="red" stroke="red"/></svg>';
  const out = runWithPlugins(input, ["convertColors"], {
    convertColorsCurrentColor: "red",
  });
  const expected = optimizeSvgo(input, {
    plugins: [{ name: "convertColors", params: { currentColor: "red" } }],
  }).data;
  expect(out).toBe(expected);
});
