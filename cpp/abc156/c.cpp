/*
 * 競技プログラミング用テンプレート
 * by yuma140902
 * 2020/06/01
 * ver 0.3.1
 */
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
#define dbg if(true)
#else
#define pdbg(msg) ;
#define dbg if(false)
#endif



int N;
vi X;

int calcTotalCost(int p){
  int cost = 0;
  rep(i, N){
    int tmp = X[i] - p;
    cost += tmp*tmp;
  }
  return cost;
}

int main(){
  cin >> N;
  X = vi_def(N);
  rep(i, N){
    cin >> X[i];
  }

  int sum = accumulate(ALL(X), 0);
  float avg = (float)sum / X.size();
  int ideal1 = (int)floorf(avg);
  int ideal2 = (int)ceilf(avg);
  int cost1 = calcTotalCost(ideal1);
  int cost2 = calcTotalCost(ideal2);
  int ans = min(cost1, cost2);

  dbg{
    printf("sum: %d, avg: %f\n", sum, avg);
    printf("ideal: [%d, %d]\n", ideal1, ideal2);
    printf("cost: [%d, %d]\n", cost1, cost2);
    printf("ans: %d\n", ans);
  }

  printf("%d\n", ans);

}

