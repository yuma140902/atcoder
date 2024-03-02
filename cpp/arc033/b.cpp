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

int Na, Nb;
vi A;
vi B;
int main(){
  scanf("%d %d", &Na, &Nb);
  A = vi_def(Na);
  rep(i, Na) scanf("%d", &A[i]);
  B = vi_def(Nb);
  rep(i, Nb) scanf("%d", &B[i]);

  sort(A.begin(), A.end());
  sort(B.begin(), B.end());
  
  vi intersection;
  set_intersection(A.begin(), A.end(), B.begin(), B.end(), inserter(intersection, intersection.end()));
  int common = intersection.size();

  double ans = (double)common / (double)(A.size() + B.size() - common);

  cout<<fixed<<setprecision(10);
  cout << ans << endl;
}

