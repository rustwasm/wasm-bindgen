export function new_custom_event() {
  return new Promise(resolve => {
    window.addEventListener("test-custom-event", resolve);
    window.dispatchEvent(new CustomEvent("test-custom-event", {
      detail: "detail",
      bubbles: true,
      cancelable: true,
      composed: true,
    }));
  });
}
