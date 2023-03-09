callback interface CallbackInterface1 {
  undefined foo();
};

callback interface CallbackInterface2 {
  undefined foo();
  undefined bar();
};

[Constructor()]
interface TakeCallbackInterface {
  undefined a(CallbackInterface1 arg);
  undefined b(CallbackInterface2 arg);
};
