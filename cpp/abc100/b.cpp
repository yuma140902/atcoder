#include <bits/stdc++.h>
using namespace std;

int main() {
  int D, N;
  cin >> D >> N;

  if (N == 100)
    ++N;
  int ans = pow(100, D) * N;
  cout << ans << endl;

  return 0;
}
