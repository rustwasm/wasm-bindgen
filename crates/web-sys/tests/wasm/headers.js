export function new_headers() {
  return new Headers({'Content-Type': 'text/plain'});
}

export function new_headers_2() {
  return new Headers({'Content-Type': 'text/plain', 'Cookie': 'foobarbaz'});
}
