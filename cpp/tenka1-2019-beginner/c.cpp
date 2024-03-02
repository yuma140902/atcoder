#include <bits/stdc++.h>
using namespace std;

constexpr char white = '.';
constexpr char black = '#';

int main() {
  int len;
  string str;
  cin >> len >> str;
  
  int sum = 0;
  for(int i=0; i<len-1; ++i) {
    if(str[i] == black && str[i+1] == white) {
      int numWhite = 0;
      for(int j=i+1; j<len; ++j){
        if(str[j] == black) break;
        else ++numWhite;
      }
      int numBlack = 0;
      for(int j=i; 0<=j; --j) {
        if(str[j] == white) break;
        else ++numBlack;
      }
      
      if(numBlack < numWhite){
        for(int j=i; 0<=j; --j){
          if(str[j] == white) break;
          str[j] = white;
        }
      }
      
      sum += min(numWhite, numBlack);
    }
  }
  
  cout << sum << endl;
}

