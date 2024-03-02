#include<bits/stdc++.h>
using namespace std;

int main() {
  int N;
  cin >> N;
  vector<int64_t> A(N+1);
  vector<int64_t> B(N);
  for(int i=0; i<N+1; ++i) cin >> A[i];
  for(int i=0; i<N; ++i) cin >> B[i];
  
  int64_t cnt = 0;
  for(int i=0; i<N; ++i) {
    if(A[i] <= B[i]) {
      B[i] -= A[i];
      cnt += A[i];
      A[i] = 0;
    }
    else {
      A[i] -= B[i];
      cnt += B[i];
      B[i] = 0;
    }
    if(A[i+1] <= B[i]) {
      B[i] -= A[i+1];
      cnt += A[i+1];
      A[i+1] = 0;
    }
    else {
      A[i+1] -= B[i];
      cnt += B[i];
      B[i] = 0;
    }
  }
  
  cout << cnt << endl;
}
