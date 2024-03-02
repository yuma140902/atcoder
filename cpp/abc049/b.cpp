#include <bits/stdc++.h>
using namespace std;
typedef int64_t ll;
const int INF = 900000;
const ll INFL = 9000000;

#define rep(i, n) for(int i=0; i < (n); ++i)
#define per(i, n) for(int i=(n)-1; i >= 0; --i)
#define range(i, begin, end) for(int i=(begin); i < (end); ++i)

#define ALL(vec) vec.begin(), vec.end()



int main(){
  int H, W;
  cin >> H >> W;
  vector<string> pict(H);
  rep(i, H) cin >> pict[i];

  rep(i, H){
    cout << pict[i] << endl;
    cout << pict[i] << endl;
  }
  return 0;
}

