import { expect, test } from "vitest";
import { optimize as optimizeSvgo } from "svgo";
import { runWithPlugins } from "./test-utils";

test("inlineStyles applies simple class selector", () => {
  const input = '<svg><style>.a{fill:red}</style><rect class="a"/></svg>';
  const out = runWithPlugins(input, ["inlineStyles"]);
  expect(out).toContain('fill="red"');
});

test("inlineStyles supports compound and descendant selectors", () => {
  const input =
    '<svg><style>g .a{fill:red}rect.a#x{stroke:blue}</style><g><rect id="x" class="a"/></g></svg>';
  const out = runWithPlugins(input, ["inlineStyles"]);
  expect(out).toContain('fill="red"');
  expect(out).toContain('stroke="blue"');
});

test("inlineStyles supports child combinator", () => {
  const input =
    '<svg><style>g>.a{fill:red}</style><g><rect class="a"/></g></svg>';
  const out = runWithPlugins(input, ["inlineStyles"]);
  expect(out).toContain('fill="red"');
});

test("inlineStyles child combinator does not match deep descendants", () => {
  const input =
    '<svg><style>svg>.a{fill:red}</style><g><g><rect class="a"/></g></g></svg>';
  const out = runWithPlugins(input, ["inlineStyles"]);
  expect(out).not.toContain('fill="red"');
});

test("inlineStyles supports adjacent sibling combinator", () => {
  const input =
    '<svg><style>g + .a{fill:red}</style><g/><rect class="a"/></svg>';
  const out = runWithPlugins(input, ["inlineStyles"]);
  expect(out).toContain('fill="red"');
});

test("inlineStyles supports general sibling combinator", () => {
  const input =
    '<svg><style>g ~ .a{stroke:blue}</style><g/><path/><rect class="a"/></svg>';
  const out = runWithPlugins(input, ["inlineStyles"]);
  expect(out).toContain('stroke="blue"');
});

test("inlineStyles supports mixed combinator chain", () => {
  const input =
    '<svg><style>g + g > .a{fill:red}</style><g/><g><rect class="a"/></g></svg>';
  const out = runWithPlugins(input, ["inlineStyles"]);
  expect(out).toContain('fill="red"');
});

test("inlineStyles skips foreignObject subtree", () => {
  const input =
    '<svg><style>.a{fill:red}</style><foreignObject><rect class="a"/></foreignObject></svg>';
  const out = runWithPlugins(input, ["inlineStyles"]);
  expect(out).not.toContain('fill="red"');
});

test("inlineStyles keeps unsupported style type", () => {
  const input = '<svg><style type="text/less">.a{fill:red}</style><rect class="a"/></svg>';
  const out = runWithPlugins(input, ["inlineStyles"]);
  const expected = optimizeSvgo(input, { plugins: ["inlineStyles"] }).data;
  expect(out).toBe(expected);
});
