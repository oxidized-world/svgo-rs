import { expect, test } from "vitest";
import { runWithPlugins } from "./test-utils";

test("removeTitle removes <title>", () => {
  const input = "<svg><title>Icon</title><g/></svg>";
  const out = runWithPlugins(input, ["removeTitle"]);
  expect(out).not.toContain("<title>");
  expect(out).toContain("<g/>");
});
