callback interface CallbackInterface1 {
  void foo();
};

callback interface CallbackInterface2 {
  void foo();
  void bar();
};

[Constructor()]
interface TakeCallbackInterface {
  void a(CallbackInterface1 arg);
  void b(CallbackInterface2 arg);
};
