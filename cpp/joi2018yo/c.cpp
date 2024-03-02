#include <bits/stdc++.h>
using namespace std;
typedef long long int ll;
const int INF = 90000;
const ll INFL = 900000;

#define vi vector<int>
#define vii vector<vector<int> >
#define vi_def(n) vector<int>((n))
#define vii_def(n,m) vector<vector<int> >((n), vector<int>((m)))
#define vi_def_dflt(n, def) vector<int>((n), (def))
#define vii_def_dflt(n, m, def) vector<vector<int> >((n), vector<int>((m), (def)))

#define rep(i, n) for(int i=0; i < (n); ++i)
#define per(i, n) for(int i=(n)-1; i >= 0; --i)
#define range(i, begin, end) for(int i=(begin); i < (end); ++i)

// 距離を計算
int calcDistance(int roadH, int roadW);

int H, W;
vii A;
int main(){
  cin >> H >> W;
  A = vii_def(H, W);
  rep(i, H){
    rep(j, W){
      int a;
      cin >> a;
      A[i][j] = a;
    }
  }
  
  int ans = 900000;

  rep(h, H){
    rep(w, W){
      int roadH = h;
      int roadW = w;
      int distance = calcDistance(roadH, roadW);
      ans = min(ans, distance);

      cerr << "roadH: " << roadH << endl;
      cerr << "roadW: " << roadW << endl;

      cerr << "distance: " << distance << endl;
      cerr << endl;
    }
  }

  cout << ans << endl;

  /*
  // 各行の最大値
  vi maxH(H, -1);
  rep(h, H){
    rep(w, W){
      int a = A[h][w];
      maxH[h] = max(maxH[h], a);
    }
  }

  // 各行の最大値の最大値
  int maxAllH = -1;
  int maxAllHIdx = -1;
  rep(h, H){
    if(maxAllH < maxH[h]){
      maxAllH = maxH[h];
      maxAllHIdx = h;
    }
  }
  // 各行の最大値が最大である行が、行方向の幹線道路
  int roadH = maxAllHIdx;

  // 各列の最大値*(ただし行方向の幹線道路の通る交差点は除く)
  vi maxW(W, -1);
  rep(w, W){
    rep(h, H){
      if(h == roadH) continue;
      int a = A[h][w];
      maxW[w] = max(maxW[w], a);
    }
  }

  // 各列の最大値*の最大値
  int maxAllW = -1;
  int maxAllWIdx = -1;
  rep(w, W){
    if(maxAllW < maxW[w]){
      maxAllW = maxW[w];
      maxAllWIdx = w;
    }
  }
  // 各列の最大値*が最大である列が、行方向の幹線道路
  int roadW = maxAllWIdx;
  */

  
}

int calcDistance(int roadH, int roadW){
  int distance = 0;
  rep(h, H){
    rep(w, W){
      distance += min(abs(h-roadH), abs(w-roadW)) * A[h][w];
    }
  }
  return distance;
}

