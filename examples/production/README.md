# Running a production server

This directory is an example of how `#[wasm_bindgen]` can be used in a production
environment.

## Quick Start

(Recommended) Globally install `yarn`:

```bash
$ npm i -g yarn
```

Install all dependencies:

```bash
$ yarn install
```

### Development watch mode

```bash
$ yarn serve
```

serves files at `http://localhost:8080`

### Production build

```bash
$ yarn build
```

will build your app in production mode and output it in `dist` folder.

```bash
$ yarn start
```

will run the production server on port `8080`,

### Notes

- `yarn` can be replaced by `npm`. Executing scripts can be done by `npm run ...`
- You can manually trigger a TypeScript typings update via `yarn rsbuild`

## Express

Express is used to serve files. Please note that there is
currently a workaround implemented for serving the correct
content-type for WebAssembly files, which will be fixed in
express@4.17.

## TypeScript

`#[wasm_bindgen]` is able to generate TypeScript typings, so
what we definitely need during development is a watch mode
on Rust files to retrigger a build, which updates our typings.
This is done by `nodemon`. Serving the files in development mode
will start all relevant watch modes.
