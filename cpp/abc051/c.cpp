/*
 * 競技プログラミング用テンプレート
 * by yuma140902
 * 2019/12/07
 * ver 0.0.0
 */
#include <bits/stdc++.h>
using namespace std;
typedef long long int ll;
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

void printMinOutwardPath(int sx, int sy, int tx, int ty);
void printMinReturnPath(int sx, int sy, int tx, int ty);

int sx, sy, tx, ty;
int main(){
  scanf("%d %d %d %d", &sx, &sy, &tx, &ty);
  printMinOutwardPath(sx, sy, tx, ty);
  printMinReturnPath(sx, sy, tx, ty);

  printf("L");
  printMinOutwardPath(sx-1, sy, tx, ty+1);
  printf("D");

  printf("R");
  printMinReturnPath(sx, sy-1, tx+1, ty);
  printf("U");

  printf("\n");
}

// 往路
void printMinOutwardPath(int sx, int sy, int tx, int ty) {
  rep(i, ty - sy){
    printf("U");
  }
  rep(i, tx - sx){
    printf("R");
  }
  
}

// 復路
void printMinReturnPath(int sx, int sy, int tx, int ty) {
  rep(i, ty - sy){
    printf("D");
  }
  rep(i, tx - sx){
    printf("L");
  }
}

