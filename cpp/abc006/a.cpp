#include<bits/stdc++.h>
using namespace std;

int main() {
  string s;
  cin >> s;
  int N = stoi(s);

  if(N%3==0) {
    cout << "YES" << endl;
    return 0;
  }

  for(const auto& c : s) {
    if(c == '3') {
      cout << "YES" << endl;
      return 0;
    }
  }

  cout << "NO" << endl;
}

