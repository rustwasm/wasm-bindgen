typedef (OffscreenCanvasRenderingContext2D or
         WebGLRenderingContext or
         WebGL2RenderingContext) OffscreenRenderingContext;

[Constructor(unsigned long width, unsigned long height),
 Exposed=(Window,Worker)]
interface OffscreenCanvas {
  attribute unsigned long width;
  attribute unsigned long height;
  OffscreenRenderingContext? getContext(DOMString contextId, any... arguments);

  // OffscreenCanvas, like HTMLCanvasElement, maintains an origin-clean flag.
  // ImageBitmaps created by calling this method also have an
  // origin-clean flag which is set to the value of the OffscreenCanvas's
  // flag at the time of their construction. Uses of the ImageBitmap
  // in other APIs, such as CanvasRenderingContext2D or
  // WebGLRenderingContext, propagate this flag like other
  // CanvasImageSource types do, such as HTMLImageElement.
  ImageBitmap transferToImageBitmap();

  // Throws a SecurityError if the OffscreenCanvas's origin-clean flag
  // is set to false.
  Promise<Blob> convertToBlob(optional ImageEncodeOptions options);
};

dictionary ImageEncodeOptions {
  DOMString type = "image/png";
  unrestricted double quality = 1.0; // Defaults to 1.0 if value is outside 0:1 range
};

OffscreenCanvas implements Transferable;

partial interface HTMLCanvasElement {
  OffscreenCanvas transferControlToOffscreen();
};

typedef (HTMLOrSVGImageElement or
         HTMLVideoElement or
         HTMLCanvasElement or
         ImageBitmap or
         OffscreenCanvas) CanvasImageSource;

[Exposed=Window, Worker]
interface OffscreenCanvasRenderingContext2D {
  // commit() can only be used when HTMLCanvasElement has transferred Control
  // to OffscreenCanvas. Otherwise, an InvalidStateError will be thrown.
  // commit() can be invoked on main thread or worker thread. When it is invoked,
  // it is expected to see the image drawn to OffscreenCanvasRenderingContext2D
  // be displayed in the associated HTMLCanvasElement.
  void commit();
  // back-reference to the canvas
  readonly attribute OffscreenCanvas canvas;
};
OffscreenCanvasRenderingContext2D implements CanvasState;
OffscreenCanvasRenderingContext2D implements CanvasTransform;
OffscreenCanvasRenderingContext2D implements CanvasCompositing;
OffscreenCanvasRenderingContext2D implements CanvasImageSmoothing;
OffscreenCanvasRenderingContext2D implements CanvasFillStrokeStyles;
OffscreenCanvasRenderingContext2D implements CanvasShadowStyles;
// Reference filters (e.g. 'url()') are not expected to work in Workers
OffscreenCanvasRenderingContext2D implements CanvasFilters;
OffscreenCanvasRenderingContext2D implements CanvasRect;
OffscreenCanvasRenderingContext2D implements CanvasDrawPath;
// Text support in workers poses very difficult technical challenges.
// Open issue: should we forgo text support in OffscreenCanvas v1?
OffscreenCanvasRenderingContext2D implements CanvasText;
OffscreenCanvasRenderingContext2D implements CanvasDrawImage;
OffscreenCanvasRenderingContext2D implements CanvasImageData;
OffscreenCanvasRenderingContext2D implements CanvasPathDrawingStyles;
OffscreenCanvasRenderingContext2D implements CanvasTextDrawingStyles;
OffscreenCanvasRenderingContext2D implements CanvasPath;

[Exposed=Window, Worker]
Partial interface CanvasPattern {
}

[Exposed=Window, Worker]
partial interface CanvasGradient {
}

partial interface WebGLRenderingContextBase {
  // back-reference to the canvas
  readonly attribute (HTMLCanvasElement or OffscreenCanvas) canvas;

  // If this context is associated with an OffscreenCanvas that was
  // created by HTMLCanvasElement's transferControlToOffscreen method,
  // causes this context's current rendering results to be pushed
  // to that canvas element. This has the same effect as returning
  // control to the main loop in a single-threaded application. Otherwise,
  // an InvalidStateError will be thrown.
  void commit();
};
