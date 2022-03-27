[Exposed=DedicatedWorker]
interface MediaStreamTrackProcessor {
  constructor(MediaStreamTrackProcessorInit init);
  attribute ReadableStream readable;
};

dictionary MediaStreamTrackProcessorInit {
  required MediaStreamTrack track;
  [EnforceRange] unsigned short maxBufferSize;
};

[Exposed=DedicatedWorker]
interface VideoTrackGenerator {
  constructor();
  readonly attribute WritableStream writable;
  attribute boolean muted;
  readonly attribute MediaStreamTrack track;
};