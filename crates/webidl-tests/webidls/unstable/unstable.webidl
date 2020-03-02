enum UnstableEnum {
  "a",
  "b"
};

dictionary UnstableDictionary {
  UnstableEnum unstableEnum;
};

typedef unsigned long UnstableTypedef;

[NoInterfaceObject]
partial interface UnstableInterface {
  UnstableTypedef enum_value(optional UnstableDictionary unstableDictionary = {});
};

interface GetUnstableInterface {
  static UnstableInterface get();
};
