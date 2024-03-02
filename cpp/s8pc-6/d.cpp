#include <bits/stdc++.h>
using namespace std;
typedef long long ll;
typedef double floating;

constexpr ll disabled = -1;
constexpr floating third = 1.0 / 3.0;

floating combine(floating left, floating right);

int main(){
  int numBall;
  cin >> numBall;
  
  vector<ll> pos(numBall);
  vector<floating> rad(numBall);
  for(int i=0; i<numBall; ++i){
    ll posin;
    floating radin;
    cin >> posin >> radin;
    pos[i] = posin;
    rad[i] = radin;
  }
  
  for(int i=0; i<numBall-1; ++i){
    ll dis = abs(pos[i] - pos[i+1]);
    ll min = std::min(pos[i], pos[i+1]);
    int minIdx = (pos[i] > pos[i+1] ? i+1 : i);
    int maxIdx = (pos[i] < pos[i+1] ? i+1 : i);
    
    if(min > dis) { //小さい方がその大きさのままもう一つに到達できるとき
      rad[minIdx] -= dis;
      rad[maxIdx] = combine(rad[i], rad[i+1]);
      pos[minIdx] = disabled;
      rad[minIdx] = disabled;
    }
    else{
      rad[maxIdx] -= dis;
      rad[minIdx] = combine(rad[i], rad[i+1]);
      pos[maxIdx] = disabled;
      rad[maxIdx] = disabled;
    }
  }
  
  for(int i=0; i<numBall; ++i) {
    if(rad[i] != disabled){
      cout << rad[i] << endl;
      return 0;
    }
  }
}

floating combine(floating left, floating right) {
  return pow(pow(left, 3) + pow(right, 3), third);
}

