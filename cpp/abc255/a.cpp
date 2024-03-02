#include <bits/stdc++.h>

using namespace std;

int main(void) {
  int R, C;
  int a[2][2];
  cin >> R >> C >> a[0][0] >> a[0][1] >> a[1][0] >> a[1][1];
  cout << a[R - 1][C - 1] << endl;
  return 0;
}

