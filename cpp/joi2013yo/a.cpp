#include <bits/stdc++.h>
using namespace std;

int main(){
  int days, jp, mp, jpm, mpm;
  cin >> days >> jp >> mp >> jpm >> mpm;
  
  int jdays = (jp+jpm-1) / jpm;
  int mdays = (mp+mpm-1) / mpm;
  
  int ans = days - max(jdays, mdays);
  cout << ans << endl;
  
}

