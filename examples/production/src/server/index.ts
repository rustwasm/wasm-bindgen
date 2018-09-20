import * as express from 'express';

import * as path from 'path';

const app = express();
const port = 8080;

/**
 * will be fixed in express@4.17
 * see https://github.com/expressjs/express/issues/3589
 */
// @ts-ignore
express.static.mime.types['wasm'] = 'application/wasm';

app.use('/', express.static(path.join(__dirname, '../client')));

app.listen(port, () => {
  console.info(`Server listening on port ${port}`)
});
