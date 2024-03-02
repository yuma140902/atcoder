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

int main(){
  string S;
  cin >> S;
  for(int i=0; i<S.size(); i+=2){
    if(S[i] != 'h' || S[i+1] != 'i'){
      cout << "No";
      return 0;
    }
  }

  cout << "Yes";
}

