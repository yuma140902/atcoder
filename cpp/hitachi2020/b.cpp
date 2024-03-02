/*
 * 競技プログラミング用テンプレート
 * by yuma140902
 * 2019/12/08
 * ver 0.1.0
 */
#include <bits/stdc++.h>
using namespace std;
typedef int64_t ll;
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

int A, B, M;
vi a;
vi b;
vi x;
vi y;
vi c;
int main(){
  cin >> A >> B >> M;
  a = vi_def(A);
  b = vi_def(B);
  x = vi_def(M);
  y = vi_def(M);
  c = vi_def(M);
  rep(i, A) cin >> a[i];
  rep(i, B) cin >> b[i];
  rep(i, M){
    cin >> x[i] >> y[i] >> c[i];
  }
  vii t = vii_def_dflt(A, B, 0);
  rep(k, M) {
    int i = x[k]-1;
    int j = y[k]-1;
    t[i][j] = std::min(t[i][j], -c[k]);
  }

  rep(i, A) rep(j, B) t[i][j] += a[i] + b[j];

  /*rep(i, A) {
    rep(j, B){
      cerr << t[i][j] << " ";
    }
    cerr << endl;
  }*/

  int min = INF;
  rep(i, A) rep(j, B) if(t[i][j] < min) min = t[i][j];

  cout << min << endl;
}

