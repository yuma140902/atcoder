#include<bits/stdc++.h>
using namespace std;

int main() {
  int N, M;
  cin >> N >> M;
  vector<bool> solved(N, false);
  int wa = 0;
  for(int i=0; i<M; ++i) {
    int n;
    string s;
    cin >> n >> s;
    if(solved[n-1]) continue;
    if(s == "WA") {
      ++wa;
    }
    else {
      solved[n-1] = true;
    }
  }
  
  int ac = 0;
  for(int i=0; i<solved.size(); ++i) if(solved[i]) ++ac;
  
  cout << ac << " " << wa << endl;
}

