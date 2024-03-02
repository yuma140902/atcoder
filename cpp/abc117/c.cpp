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

void printVec(vector<long long> v){
  rep(i, v.size()) cout << v[i] << ", ";
  cout << endl;
}

void printVec(vector<int> v){
  rep(i, v.size()) cout << v[i] << ", ";
  cout << endl;
}

void solve(long long N, long long M, std::vector<long long> X){
  if(N > M) N = M;

  sort(ALL(X));

  vi D(M-1);
  rep(i, M-1) D[i] = X[i+1] - X[i];

  // argsort D
  vi indices(M-1);
  iota(ALL(indices), 0);
  sort(ALL(indices), [&D](size_t i, size_t j) {
    return D[i] < D[j];
  });
  reverse(ALL(indices));

  rep(i, N-1) D[indices[i]] = 0;

  /*int tmpsum = 0;
  int maxsum = 0;
  rep(i, M-1){
    if(D[i] != 0) tmpsum += D[i];
    else{
      maxsum = max(maxsum, tmpsum);
      tmpsum = 0;
    }
  }

  cout << maxsum << endl;
  */
  
  cout << accumulate(ALL(D), 0) << endl;
}

int main(){
  long long N;
    scanf("%lld",&N);
    long long M;
    scanf("%lld",&M);
    std::vector<long long> X(M);
    for(int i = 0 ; i < M ; i++){
        scanf("%lld",&X[i]);
    }
  solve(N, M, std::move(X));
  return 0;
}


