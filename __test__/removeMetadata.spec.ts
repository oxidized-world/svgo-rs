import { expect, test } from "vitest";
import { runWithPlugins } from "./test-utils";

test("removeMetadata removes <metadata>", () => {
  const input = "<svg><metadata><foo/></metadata><g/></svg>";
  const out = runWithPlugins(input, ["removeMetadata"]);
  expect(out).not.toContain("<metadata");
  expect(out).toContain("<g/>");
});
