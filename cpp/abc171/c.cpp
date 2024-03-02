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

/*
n桁以下のabc記数法で表せる最大の数
n -> Σ 26^i
ex.
1 -> 26
2 -> 26 + 26^2
3 -> 26 + 26^2 + 26^3
*/
ll digits_max(int n){
  return 26 * (pow(26,n) - 1) / 25;
}

// n番目のアルファベットを返す。Aは0番目
char alpha(int n){
  return 'a' + n;
}

const int BASE = 26;
ll N;

int main(){
  cin >> N;
  int group = 0;
  // Nは第{group}群
  while(digits_max(group) < N) ++group;
  // Nは第{group}群の中で{N-digits_max(group-1)}番目
  N = N - digits_max(group-1)-1;

  for(int d=group-1; d >= 0; --d){
    int top = N/pow(BASE,d);
    cout << alpha(top);
    N -= top*pow(BASE,d);
  }
}


