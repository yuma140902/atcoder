#include<bits/stdc++.h>
using namespace std;

int main() {
  int len, numQ;
  cin >> len >> numQ;
  string str;
  cin >> str;
  
  vector<int> numACs(len-1);
  int cnt = 0;
  for(int i=0; i<len-1; ++i) {
    if(str[i] == 'A' && str[i+1] == 'C') {
      numACs[i] = ++cnt;
    }
    else {
      numACs[i] = cnt;
    }
  }
  
  for(int i=0; i<numQ; ++i) {
    int start, end;
    cin >> start >> end;
    int ans = numACs[end-2] - numACs[start-2];
    cout << ans << endl;
  }
  
}

