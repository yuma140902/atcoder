#include <bits/stdc++.h>
using namespace std;

int main(){
  vector<int> data(5);
  int min = 11;
  int minIdx = -1;
  
  for(int i=0; i<5; ++i) {
    int j;
    cin >> j;
    data[i] = j;
    
    int tmp = j%10;
    if(tmp != 0 && tmp < min) {
      minIdx = i;
      min = tmp;
    }
  }
  
  int sum = 0;
  
  for(int i=0; i<5; ++i) {
    if(i == minIdx){
      sum += data[i];
    }
    else{
      int div = data[i]/10;
      int mod = data[i]%10;
      sum += (mod==0 ? data[i] : div*10+10);
    }
  }
  
  cout << sum << endl;
  
}

