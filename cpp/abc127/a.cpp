#include <bits/stdc++.h>
using namespace std;

int main(){
  int A, B;
  cin >> A >> B;
  
  int cost = 0;
  if(13 <= A) {
    cost = B;
  }
  else if(6 <= A && A <= 12) {
    cost = B/2;
  }
  else {
    cost = 0;
  }
  
  cout << cost << endl;
}

