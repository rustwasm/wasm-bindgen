[Constructor(double value)]
interface Method {
  [Pure]
    boolean myCmp(Method bar);
};

[Constructor(double value)]
interface Property {
  [Pure]
    attribute double value;
};

[NamedConstructor=NamedConstructorBar(double value)]
interface NamedConstructor {
  [Pure]
    readonly attribute double value;
};

interface StaticMethod {
  static double swap(double value);
};

interface StaticProperty {
  static attribute double value;
};

[Constructor()]
interface UndefinedMethod {
  boolean ok_method();
  boolean bad_method(UndefinedType undef);
};

[Constructor()]
interface Unforgeable {
  [Unforgeable] readonly attribute short uno;
  readonly attribute short dos;
};

[Constructor]
interface PartialInterface {
  readonly attribute short un;
  short deux();
};

partial interface PartialInterface {
  readonly attribute short trois;
  short quatre();
};

[Constructor(short bar)]
interface MixinFoo {
  static attribute short defaultBar;
};

interface mixin MixinBar {
  readonly attribute short bar;
};

partial interface mixin MixinBar {
  void addToBar(short other);
};

MixinFoo includes MixinBar;
