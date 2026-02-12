import { optimizeWithPlugins } from "../index";

export const runWithPlugins = (
  input: string,
  plugins: string[],
  extra?: Record<string, unknown>,
) => {
  return optimizeWithPlugins(input, {
    plugins,
    ...(extra ?? {}),
  } as any);
};
