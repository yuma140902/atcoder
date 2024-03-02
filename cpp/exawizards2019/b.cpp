#include<bits/stdc++.h>
using namespace std;

int main() {
  int n;
  int cnt = 0;
  
  for(int i=0; i<n; ++i) {
    char c;
    cin >> c;
    if(c=='R') ++cnt;
    else if(c=='B') --cnt;
  }
  
  cout << (0 < cnt ? "Yes" : "No");
}

