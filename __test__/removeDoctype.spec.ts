import { expect, test } from "vitest";
import { runWithPlugins } from "./test-utils";

test("removeDoctype removes doctype", () => {
  const input =
    '<!DOCTYPE svg PUBLIC "-//W3C//DTD SVG 1.1//EN" "http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd"><svg/>';
  const out = runWithPlugins(input, ["removeDoctype"]);
  expect(out).not.toContain("<!DOCTYPE");
  expect(out).toContain("<svg/>");
});
