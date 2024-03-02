#include <bits/stdc++.h>
using namespace std;

int main(){
  vector<int> data(5);
  int k;
  for(int i=0; i<5; ++i) {
    int j;
    cin >> j;
    data[i] = j;
  }
  cin >> k;
  
  for(int i=0; i<5; ++i) {
    for(int j=i+1; j<5; ++j) {
      if(data[j] - data[i] > k) {
        cout << ":(" << endl;
        return 0;
      }
    }
  }
  
  cout << "Yay!" << endl;
  return 0;
}

