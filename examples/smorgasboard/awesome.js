export function bar_on_reset(s, token) {
  console.log(token);
  console.log(`this instance of bar was reset to ${s}`);
}

export class Awesome {
  constructor() {
    this.internal = 32;
  }

  get_internal() {
    return this.internal;
  }
}
