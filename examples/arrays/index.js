const rust = import('./pkg');
rust
  .then(m => {
      console.log(m.asceding_array(10));
      console.log(m.product([1, 2, 3, 4]));
  })
  .catch(console.error);
