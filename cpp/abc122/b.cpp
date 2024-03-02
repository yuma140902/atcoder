#include<bits/stdc++.h>
using namespace std;

int main() {
  string input;
  cin >> input;
  
  int cnt = 0;
  int prevCnt = 0;
  size_t len = input.length();
  for(int i=0; i < len; ++i) {
    if(input[i] == 'A' || input[i] == 'T'
       || input[i] == 'C' || input[i] == 'G') {
      ++cnt;
    }
    else{
      prevCnt = cnt > prevCnt ? cnt : prevCnt;
      cnt = 0;
    }
  }
  
  cout << (cnt > prevCnt ? cnt : prevCnt) << endl;
  
}

