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
interface NullableMethod {
  octet? opt(short? a);
};

[Global=GlobalMethod, Constructor()]
interface GlobalMethod {
  octet m();
};

[Constructor()]
interface Indexing {
  getter short (unsigned long index);
  setter void (unsigned long index, short value);
  deleter void (unsigned long index);
};

[Constructor()]
interface OptionalAndUnionArguments {
  DOMString m(
    DOMString a,
    optional boolean b = true,
    optional (short or DOMString) c = 123,
    optional (long long or boolean)? d = 456
  );
};

[Constructor()]
interface Variadic {
  short sum(short... values);
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

[Constructor()]
interface Overloads {
  void foo();
  void foo(DOMString arg, optional long a);
  void foo(DOMString arg, (float or short) b);
};

callback MyCallback = any();
callback AddCallback = long(long a, long b);
callback RepeatCallback = DOMString(DOMString a, long cnt);
callback GetAnswer = long();

[Constructor()]
interface InvokeCallback {
  void invoke(MyCallback callback);
  long callAdd(AddCallback callback);
  DOMString callRepeat(RepeatCallback callback);
};
