// https://html.spec.whatwg.org/multipage/imagebitmap-and-animations.html#animation-frames

callback FrameRequestCallback = undefined (DOMHighResTimeStamp time);

interface mixin AnimationFrameProvider {
  [Throws] long requestAnimationFrame(FrameRequestCallback callback);
  [Throws] undefined cancelAnimationFrame(long handle);
};
Window includes AnimationFrameProvider;
DedicatedWorkerGlobalScope includes AnimationFrameProvider;
