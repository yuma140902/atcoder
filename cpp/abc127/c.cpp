#include <bits/stdc++.h>
using namespace std;

int main(){
  int N, M;
  cin >> N >> M;
  
  vector<int> L(M);
  vector<int> R(M);
  for(int i=0; i<M; ++i) {
    int l, r;
    cin >> l >> r;
    L[i] = l;
    R[i] = r;
  }
  
  int Lmax = 0;
  int Rmin = R[0];
  for(int i=0; i<M; ++i) {
    Lmax = max(Lmax, L[i]);
    Rmin = min(Rmin, R[i]);
  }
  
  int count = 0;
  for(int i=0; i<=N; ++i) {
    if(Lmax <= i && i <= Rmin) { ++count; }
  }
  
  cout << count << endl;
  
}

