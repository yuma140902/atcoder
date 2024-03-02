#include <bits/stdc++.h>
using namespace std;

int main(){
  int r, D, x0;
  cin >> r >> D >> x0;
  
  vector<int> x(11, 0);
  x[0] = x0;
  
  for(int i=0; i<10+1; ++i) {
    x[i+1] = r*x[i] - D;
  }
  
  for(int i=1; i<10+1; ++i) {
    cout << x[i] << endl;
  }
}

