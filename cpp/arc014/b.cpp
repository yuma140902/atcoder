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

int main(){
  int N; cin >> N;
  vector<string> w(N); rep(i, N) cin >> w[i];

  unordered_set<string> c1;
  unordered_set<string> c2;

  rep(i, N){
    unordered_set<string> &c = (1%2==0) ? c1 : c2;

    if(c.find(w[i]) != c.end()){
      if(i%2==0) {
        cout << "LOSE" << endl;
        return 0;
      }
      else {
        cout << "WIN" << endl;
        return 0;
      }
    }

    c.emplace(w[i]);

    if(i != 0){
      string prevWord = w[i-1];
      if(prevWord.at(prevWord.size()-1) != w[i].at(0)){
        if(i%2==0) {
          cout << "LOSE" << endl;
          return 0;
        }
        else {
          cout << "WIN" << endl;
          return 0;
        }
      }
    }

  }
  cout << "DRAW" << endl;
}

