#include <cstdint>
#include <cstdio>
#include <iostream>

using i64 = std::int64_t;
int main(int argc, char *argv[]) {
  i64 n, x, y, z;
  std::cin >> n >> x >> y >> z;

  if (x < z && z < y || y < z && z < x) {
    std::cout << "Yes" << std::endl;
  } else {
    std::cout << "No" << std::endl;
  }

  /*FILE *fp = 0;
  std::fclose(fp);
  std::fclose(fp);*/
  return 0;
}
