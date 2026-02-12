import { expect, test } from "vitest";
import { runWithPlugins } from "./test-utils";

test("removeXMLProcInst removes only XML declaration (decl), keeps other PI", () => {
  const input =
    '<?xml version="1.0" encoding="UTF-8"?>' +
    '<?xml-stylesheet type="text/css" href="style.css"?>' +
    "<svg/>";
  const out = runWithPlugins(input, ["removeXMLProcInst"]);
  expect(out).not.toContain("<?xml version=");
  expect(out).toContain('<?xml-stylesheet type="text/css" href="style.css"?>');
  expect(out).toContain("<svg/>");
});
