[NoInterfaceObject]
interface NoInterfaceObject {
  readonly attribute double number;
  undefined foo();
};

interface GetNoInterfaceObject {
  static NoInterfaceObject get();
};
