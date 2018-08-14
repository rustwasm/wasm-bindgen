interface ConstBool {
  const boolean not_true = false;
  const boolean not_false = true;
};

interface ConstByte {
  const byte imin = -128;
  const byte imax = 127;
  const octet umin = 0;
  const octet umax = 255;
};

interface ConstShort {
  const short imin = -32768;
  const short imax = 32767;
  const unsigned short umin = 0;
  const unsigned short umax = 65535;
};

interface ConstLong {
  const long imin = -2147483648;
  const long imax = 2147483647;
  const unsigned long umin = 0;
  const unsigned long umax = 4294967295;
};

interface ConstLongLong {
  const long long imin = -9223372036854775808;
  const long long imax = 9223372036854775807;
  const unsigned long long umin = 0;
  const unsigned long long umax = 18446744073709551615;
};

interface ConstFloats {
  const float f = 0.0;
  const unrestricted float neg_inf = -Infinity;
  const unrestricted float inf = Infinity;
  const unrestricted float nan = NaN;
};

interface ConstDoubles {
  const double d = 0.0;
  const unrestricted double neg_inf = -Infinity;
  const unrestricted double inf = Infinity;
  const unrestricted double nan = NaN;
};

interface mixin ConstDoublesMixin {
  const double one = 1.0;
};

ConstDoubles includes ConstDoublesMixin;
