import { expect, test } from "vitest";
import { optimize as optimizeSvgo } from "svgo";
import { runWithPlugins } from "./test-utils";

test("removeEditorsNSData removes editor namespaces, prefixed attrs and elements", () => {
  const input =
    '<svg xmlns:sodipodi="http://sodipodi.sourceforge.net/DTD/sodipodi-0.dtd" width="10">' +
    "<sodipodi:namedview/>" +
    '<path sodipodi:nodetypes="cccc" d="M0 0"/>' +
    '<g id="ok"/>' +
    "</svg>";
  const out = runWithPlugins(input, ["removeEditorsNSData"]);

  const expected = optimizeSvgo(input, { plugins: ["removeEditorsNSData"] }).data;
  expect(out).toBe(expected);
});

test("removeEditorsNSData supports additionalNamespaces", () => {
  const input =
    '<svg xmlns:inkscape="http://www.inkscape.org/namespaces/inkscape"><g inkscape:label="x"/><g id="ok"/></svg>';
  const out = runWithPlugins(input, ["removeEditorsNSData"], {
    removeEditorsNsDataAdditionalNamespaces: ["inkscape"],
  });
  const expected = optimizeSvgo(input, {
    plugins: [
      { name: "removeEditorsNSData", params: { additionalNamespaces: ["inkscape"] } },
    ],
  }).data;
  expect(out).toBe(expected);
});

test("removeEditorsNSData keeps non-editor namespaces", () => {
  const input =
    '<svg xmlns:foo="http://example.com/foo"><foo:bar/><rect width="1" height="1"/></svg>';
  const out = runWithPlugins(input, ["removeEditorsNSData"]);
  const expected = optimizeSvgo(input, { plugins: ["removeEditorsNSData"] }).data;
  expect(out).toBe(expected);
});
