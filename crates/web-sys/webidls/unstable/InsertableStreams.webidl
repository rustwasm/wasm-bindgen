[Exposed=Window,DedicatedWorker]
interface MediaStreamTrackProcessor {
  constructor(MediaStreamTrackProcessorInit init);
  attribute ReadableStream readable;
};

dictionary MediaStreamTrackProcessorInit {
  required MediaStreamTrack track;
  [EnforceRange] unsigned short maxBufferSize;
};

[Exposed=Window,DedicatedWorker]
interface MediaStreamTrackGenerator : MediaStreamTrack {
    constructor(MediaStreamTrackGeneratorInit init);
    attribute WritableStream writable;  // VideoFrame or AudioData
};

dictionary MediaStreamTrackGeneratorInit {
  required DOMString kind;
};