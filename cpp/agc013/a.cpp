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

int N;
vi A;

int main(){
  cin >> N;
  A = vi_def(N);
  rep(i,N) cin >> A[i];
  
  int i = 0;
  int ans = 0;
  while(true){
    while(i+1 < N && A[i] == A[i+1]) ++i;

    if(i+1 < N && A[i] < A[i+1]){
      while(i+1 < N && A[i] <= A[i+1]) ++i;
    }
    else if(i+1 < N && A[i] > A[i+1]){
      while(i+1 < N && A[i] >= A[i+1]) ++i;
    }
    ++i;
    ++ans;
    if(i >= N) break;
  }

  cout << ans << endl;
}

