/* Source: https://wicg.github.io/performance-measure-memory/#idl-index */

dictionary MemoryMeasurement {
  unsigned long long bytes;
  sequence<MemoryBreakdownEntry> breakdown;
};

dictionary MemoryBreakdownEntry {
  unsigned long long bytes;
  sequence<MemoryAttribution> attribution;
  sequence<DOMString> types;
};

dictionary MemoryAttribution {
  USVString url;
  MemoryAttributionContainer container;
  DOMString scope;
};

dictionary MemoryAttributionContainer {
  DOMString id;
  USVString src;
};

partial interface Performance {
  [Exposed=(Window,ServiceWorker,SharedWorker), CrossOriginIsolated] Promise<MemoryMeasurement> measureUserAgentSpecificMemory();
};
