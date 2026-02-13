import { expect, test } from "vitest";
import { optimize as optimizeSvgo } from "svgo";
import { runWithPlugins } from "./test-utils";

test("removeXMLProcInst removes only XML declaration (decl), keeps other PI", () => {
  const input =
    '<?xml version="1.0" encoding="UTF-8"?>' +
    '<?xml-stylesheet type="text/css" href="style.css"?>' +
    "<svg/>";
  const out = runWithPlugins(input, ["removeXMLProcInst"]);
  const expected = optimizeSvgo(input, { plugins: ["removeXMLProcInst"] }).data;
  expect(out).toBe(expected);
});

test("removeXMLProcInst no-op when declaration absent", () => {
  const input = "<svg><g/></svg>";
  const out = runWithPlugins(input, ["removeXMLProcInst"]);
  const expected = optimizeSvgo(input, { plugins: ["removeXMLProcInst"] }).data;
  expect(out).toBe(expected);
});

test("removeXMLProcInst keeps non-xml processing instructions", () => {
  const input = "<?target data?><svg/>";
  const out = runWithPlugins(input, ["removeXMLProcInst"]);
  const expected = optimizeSvgo(input, { plugins: ["removeXMLProcInst"] }).data;
  expect(out).toBe(expected);
});
