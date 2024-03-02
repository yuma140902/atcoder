#include<bits/stdc++.h>
using namespace std;

int main() {
  string S;
  cin >> S;
  set<int> s;

  for(int i=0; i<S.size(); ++i) {
    s.insert(S[i] - '0');
  }

  for(int i=0; i<=9; ++i) {
    if(s.find(i) == s.end()) {
      cout << i << endl;
      return 0;
    }
  }
}
