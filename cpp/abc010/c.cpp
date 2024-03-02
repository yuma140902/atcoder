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

const string YES = "YES";
const string NO = "NO";

void solve(long long tx_a, long long ty_a, long long tx_b, long long ty_b, long long T, long long V, long long n, std::vector<long long> x, std::vector<long long> y){
  constexpr double e = 1e-10;
  rep(i, n){
    double d1 = sqrt(pow(tx_a-x[i],2) + pow(ty_a-y[i],2));
    double d2 = sqrt(pow(x[i]-tx_b,2) + pow(y[i]-ty_b,2));
    if((d1+d2) <= e+V*T){
      cout << YES << endl;
      return;
    }
  }
  cout << NO << endl;
}

int main(){
  long long tx_a;
    scanf("%lld",&tx_a);
    long long ty_a;
    scanf("%lld",&ty_a);
    long long tx_b;
    scanf("%lld",&tx_b);
    long long ty_b;
    scanf("%lld",&ty_b);
    long long T;
    scanf("%lld",&T);
    long long V;
    scanf("%lld",&V);
    long long n;
    scanf("%lld",&n);
    std::vector<long long> x(n);
    std::vector<long long> y(n);
    for(int i = 0 ; i < n ; i++){
        scanf("%lld",&x[i]);
        scanf("%lld",&y[i]);
    }
  solve(tx_a, ty_a, tx_b, ty_b, T, V, n, std::move(x), std::move(y));
}

