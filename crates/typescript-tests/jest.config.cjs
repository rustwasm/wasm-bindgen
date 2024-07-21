/** @type {import('ts-jest').JestConfigWithTsJest} */
module.exports = {
  preset: "ts-jest/presets/default-esm",
  testEnvironment: 'node',
  extensionsToTreatAsEsm: [".ts"],
  verbose: true,
  testMatch: ['**/src/*.ts', '!**/src/*.d.ts'],
  // TODO: migrate all test files and remove this
  testPathIgnorePatterns: [
    ".*/src/custom_section.ts$",
    ".*/src/getters_setters.ts$",
    ".*/src/inspectable.ts$",
    ".*/src/memory.ts$",
    ".*/src/omit_definition.ts$",
    ".*/src/optional_fields.ts$",
    ".*/src/opt_args_and_ret.ts$",
    ".*/src/simple_async_fn.ts$",
    ".*/src/simple_fn.ts$",
    ".*/src/web_sys.ts$",
    ".*/src/usize.ts$"
  ],
  injectGlobals: false,
  transform: {
    '^.+.tsx?$': ['ts-jest',
      {
        useESM: true,
        isolatedModules: true
      }]
  }
};
