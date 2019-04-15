[NoInterfaceObject]
interface NoInterfaceObject {
  readonly attribute double number;
  void foo();
};

interface GetNoInterfaceObject {
  static NoInterfaceObject get();
};
