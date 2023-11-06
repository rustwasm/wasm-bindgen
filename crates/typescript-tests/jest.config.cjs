/** @type {import('ts-jest').JestConfigWithTsJest} */
module.exports = {
  preset: "ts-jest/presets/default-esm",
  testEnvironment: 'node',
  extensionsToTreatAsEsm: [".ts"],
  verbose: true,
  testMatch: ['**/src/simple_struct.ts', '**/src/typescript_type.ts'],
  globals: {
    'ts-jest':
    {
      useESM: true,
      isolatedModules: true
    }
  }
};
