#include<bits/stdc++.h>
using namespace std;

constexpr int64_t MOD = 1e9+7;

int main() {
  int64_t N;
  cin >> N;
  int64_t p = 1;
  for(int64_t i=1; i<=N; ++i) {
    p *= i;
    p %= MOD;
  }
  cout << p << endl;
}

