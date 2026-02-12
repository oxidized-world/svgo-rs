import { expect, test } from "vitest";
import { runWithPlugins } from "./test-utils";

test("removeEditorsNSData removes editor namespaces, prefixed attrs and elements", () => {
  const input =
    '<svg xmlns:sodipodi="http://sodipodi.sourceforge.net/DTD/sodipodi-0.dtd" width="10">' +
    "<sodipodi:namedview/>" +
    '<path sodipodi:nodetypes="cccc" d="M0 0"/>' +
    '<g id="ok"/>' +
    "</svg>";
  const out = runWithPlugins(input, ["removeEditorsNSData"]);

  expect(out).not.toContain("xmlns:sodipodi");
  expect(out).not.toContain("sodipodi:namedview");
  expect(out).not.toContain("sodipodi:nodetypes");
  expect(out).toContain('width="10"');
  expect(out).toContain('<g id="ok"/>');
});
