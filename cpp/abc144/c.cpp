#include<bits/stdc++.h>
using namespace std;

int main() {
  int64_t N;
  cin >> N;
  auto R = static_cast<int64_t>(floor(sqrt(N)));
  for(int r=R; r>0; --r) {
    if(N % r != 0) continue;
    int64_t p = N / r;
    cout << (p+r-2) << endl;
    break;
  }
}

