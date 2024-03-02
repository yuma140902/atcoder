#include <bits/stdc++.h>
using namespace std;

int main(){
  int N, M;
  cin >> N >> M;
  
  vector<int> A(N);
  for(int i=0; i<N; ++i) {
    int a;
    cin >> a;
    A[i] = a;
  }
  
  vector<int> B(M);
  vector<int> C(M);
  for(int j=0; j<M; ++j) {
    int b, c;
    cin >> b >> c;
    B[j] = b;
    C[j] = c;
  }
  
  sort(A.begin(), A.end());
  
  for(int j=0; j<M; ++j) {
    if(A[B[j]-1] <= C[j]) {
      for(int i=0; i<B[j]; ++i) {
        A[i] = C[j];
      }
      if(A[B[j]-1] > A[B[j]]) {
        sort(A.begin(), A.end());
      }
    }
  }
  
  int sum = accumulate(A.begin(), A.end(), 0);
  cout << sum << endl;
}

