import { expect, test } from "vitest";
import { runWithPlugins } from "./test-utils";

test("moveElemsAttrsToGroup moves common attrs up and deletes from children", () => {
  const input =
    "<svg><g>" +
    '<circle fill="red" stroke="blue"/>' +
    '<rect fill="red" stroke="green"/>' +
    "</g></svg>";
  const out = runWithPlugins(input, ["moveElemsAttrsToGroup"]);
  expect(out).toContain('<g fill="red">');
  expect(out).toContain('<circle stroke="blue"/>');
  expect(out).toContain('<rect stroke="green"/>');
  expect(out).not.toContain('<circle fill="red"');
  expect(out).not.toContain('<rect fill="red"');
});

test("moveElemsAttrsToGroup overwrites existing group attrs (svgo behavior)", () => {
  const input =
    '<svg><g fill="blue">' +
    '<circle fill="red"/>' +
    '<rect fill="red"/>' +
    "</g></svg>";
  const out = runWithPlugins(input, ["moveElemsAttrsToGroup"]);
  expect(out).toContain('<g fill="red">');
  expect(out).not.toContain('<g fill="blue"');
});

test("moveElemsAttrsToGroup is deoptimized when <style> exists", () => {
  const input =
    "<svg><style>.a{fill:red}</style>" +
    "<g>" +
    '<circle fill="red"/>' +
    '<rect fill="red"/>' +
    "</g></svg>";
  const out = runWithPlugins(input, ["moveElemsAttrsToGroup"]);
  expect(out).toContain("<g>");
  expect(out).toContain('<circle fill="red"/>');
  expect(out).toContain('<rect fill="red"/>');
});

test("moveElemsAttrsToGroup preserves transform when all children are paths", () => {
  const input =
    "<svg><g>" +
    '<path transform="scale(2)" d="M0 0"/>' +
    '<path transform="scale(2)" d="M1 1"/>' +
    "</g></svg>";
  const out = runWithPlugins(input, ["moveElemsAttrsToGroup"]);
  expect(out).not.toContain("<g transform=");
  expect(out).toContain('<path transform="scale(2)"');
});
