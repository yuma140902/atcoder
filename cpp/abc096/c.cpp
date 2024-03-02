#include <bits/stdc++.h>
using namespace std;
typedef int64_t ll;
typedef tuple<int, int> ii;
typedef tuple<int, int, int> iii;
const int INF = 900000;
const ll INFL = 9000000;

#define vi vector<int>
#define vii vector<vector<int> >
#define vi_def(n) vector<int>((n))
#define vii_def(n,m) vector<vector<int> >((n), vector<int>((m)))
#define vi_def_dflt(n, def) vector<int>((n), (def))
#define vii_def_dflt(n, m, def) vector<vector<int> >((n), vector<int>((m), (def)))

#define rep(i, n) for(int i=0; i < (n); ++i)
#define per(i, n) for(int i=(n)-1; i >= 0; --i)
#define range(i, begin, end) for(int i=(begin); i < (end); ++i)

#define ALL(vec) vec.begin(), vec.end()

#ifdef DBG_FLAG
#define pdbg(msg) cout << (msg)
#define pdbgl(msg) cout << (msg) << endl
#define dbg if(true)
#else
#define pdbg(msg) ;
#define pdbgl(msg) ;
#define dbg if(false)
#endif

int H, W;
vector<vector<bool>> canvas;
vector<vector<bool>> canBeBlack;

int main(){
  cin >> H >> W;
  canvas = vector<vector<bool>>(H, vector<bool>(W));
  canBeBlack = vector<vector<bool>>(H, vector<bool>(W, false));
  rep(h, H) rep(w, W){
    char c;
    cin >> c;
    canvas[h][w] = (c == '#');
  }

  rep(h, H) rep(w, W){
    if(canvas[h][w]){
      if(h-1 >= 0) canBeBlack[h-1][w] = true;
      if(h+1 < H) canBeBlack[h+1][w] = true;
      if(w-1 >= 0) canBeBlack[h][w-1] = true;
      if(w+1 < W) canBeBlack[h][w+1] = true;
    }
  }

  bool possible = true;
  rep(h, H) rep(w, W){
    if(canvas[h][w] && !canBeBlack[h][w]) {
      possible = false;
      break;
    }
  }

  cout << (possible ? "Yes" : "No") << endl;
}

