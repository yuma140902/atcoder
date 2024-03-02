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

int f(int x);

int N;
int main(){
  cin >> N;

  double x = (double)N / 1.08;
  int X = floor(x);

  dbg{
    printf("N: %d\n", N);
    printf("x: %f\n", x);
  }

  if(x > 0){
    int tmp;
    while((tmp = f(X)) <= 0){
      if(tmp == 0){
        printf("%d\n", X);
        return 0;
      }
      ++X;
    }
  }
  printf(":(\n");
}

int f(int x){
  int n = floor((double)x * 1.08);
  if(n<N) return -1;
  else if(n==N) return 0;
  else return 1;
}
