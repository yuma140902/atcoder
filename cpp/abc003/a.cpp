#include<bits/stdc++.h>
using namespace std;

int main() {
  int N;
  cin >> N;
  
  int64_t sum = 0;
  for(int i=0; i<N; ++i) {
    sum += (i+1) * 10000;
  }

  double ans = static_cast<double>(sum) / static_cast<double>(N);
  cout << ans << endl;
}

