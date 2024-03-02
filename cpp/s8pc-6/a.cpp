#include <bits/stdc++.h>
using namespace std;

int main(){
  int numCities, timeLeep;
  cin >> numCities >> timeLeep;
  
  int timeSum = 0;
  for(int i=0; i<numCities-1; ++i){
    int j;
    cin >> j;
    timeSum += j;
  }
  
  int div = timeSum / timeLeep;
  int mod = timeSum % timeLeep;
  int ans = (mod==0 ? div : div+1);
  
  cout << ans;
}

