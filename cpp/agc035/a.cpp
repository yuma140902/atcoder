#include<bits/stdc++.h>
using namespace std;

int main() {
  int N;
  cin >> N;
  
  vector<int> a(N);
  for(int i=0; i<N; ++i) {
    int in;
    cin >> in;
    a[i] = in;
  }
  
  int tmp = a[0];
  for(int i=1; i<N; ++i) {
    tmp = tmp ^ a[i];
  }
  
  if(tmp==0) cout << "Yes" << endl;
  else cout << "No" << endl;
  
  return 0;
}

