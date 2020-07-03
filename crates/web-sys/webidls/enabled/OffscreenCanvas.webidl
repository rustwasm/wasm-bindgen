typedef (OffscreenCanvasRenderingContext2D or ImageBitmapRenderingContext or WebGLRenderingContext or WebGL2RenderingContext) OffscreenRenderingContext;

dictionary ImageEncodeOptions {
  DOMString type = "image/png";
  unrestricted double quality;
};

enum OffscreenRenderingContextId { "2d", "bitmaprenderer", "webgl", "webgl2" };

[Exposed=(Window,Worker), Transferable]
interface OffscreenCanvas : EventTarget {
  constructor([EnforceRange] unsigned long long width, [EnforceRange] unsigned long long height);

  attribute [EnforceRange] unsigned long long width;
  attribute [EnforceRange] unsigned long long height;

  OffscreenRenderingContext? getContext(OffscreenRenderingContextId contextId, optional any options = null);
  ImageBitmap transferToImageBitmap();
  Promise<Blob> convertToBlob(optional ImageEncodeOptions options = {});
};
