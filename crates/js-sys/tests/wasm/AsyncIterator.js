async function* createAsyncIterable(syncIterable) {
  for (const elem of syncIterable) {
    yield elem;
  }
}

exports.get_async_iterable = () => createAsyncIterable(['one', 'two', 'three']);
