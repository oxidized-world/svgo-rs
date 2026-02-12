import { expect, test } from "vitest";
import { runWithPlugins } from "./test-utils";

test("addAttributesToSVGElement adds attributes to outer svg and does not overwrite existing", () => {
  const input = '<svg viewBox="0 0 10 10"><g><svg/></g></svg>';
  const out = runWithPlugins(input, ["addAttributesToSVGElement"], {
    addAttributesToSvgElementAttributes: [
      "viewBox=1 1 1 1",
      "focusable=false",
      "data-image=icon",
    ],
  });

  expect(out).toContain('viewBox="0 0 10 10"');
  expect(out).toContain('focusable="false"');
  expect(out).toContain('data-image="icon"');
  expect(out).toContain("<g><svg/></g>");
});

test("addAttributesToSVGElement supports single attribute option", () => {
  const input = "<svg><g/></svg>";
  const out = runWithPlugins(input, ["addAttributesToSVGElement"], {
    addAttributesToSvgElementAttribute: "mySvg",
  });

  expect(out).toContain('mySvg=""');
});

test("addAttributesToSVGElement without params keeps svg unchanged", () => {
  const input = "<svg><g/></svg>";
  const out = runWithPlugins(input, ["addAttributesToSVGElement"]);

  expect(out).toBe(input);
});
