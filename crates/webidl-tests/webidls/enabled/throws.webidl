[Constructor(long value)]
interface Thang {
  [Throws]
    attribute long ok_attr;
  [Throws]
    attribute long err_attr;

  [Throws]
    long ok_method();
  [Throws]
    long err_method();

  [Throws]
    static long ok_static_method();
  [Throws]
    static long err_static_method();

  [Throws]
    static attribute long ok_static_attr;
  [Throws]
    static attribute long err_static_attr;
};
