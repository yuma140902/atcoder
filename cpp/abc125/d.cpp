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

int N;
vi A;
int main(){
  scanf("%d", &N);
  A = vi_def(N);
  rep(i, N) scanf("%d", &A[i]);

  int countNegative = 0;
  rep(i, N) if(A[i] < 0) ++countNegative;

  rep(i, N) A[i] = abs(A[i]);

  ll sum = accumulate(A.begin(), A.end(), (ll)0);

  if(countNegative % 2 == 0){
    cout << sum << endl;
  }
  else{
    int min = *min_element(A.begin(), A.end());
    cout << sum-(min*2) << endl;
  }

}

