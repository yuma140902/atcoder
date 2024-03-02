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

const char L = 'L';
const char R = 'R';
const char U = 'U';
const char D = 'D';
const int FIND_MAX = 1;
const int FIND_MIN = 2;

string S;
int T;
int main(){
  cin >> S;
  cin >> T;

  int x=0, y=0;
  rep(i, S.size()){
    if(S[i] == L) --x;
    else if(S[i] == R) ++x;
    else if(S[i] == U) ++y;
    else if(S[i] == D) --y;
  }

  int numBlank = 0;
  rep(i, S.size()){
    if(S[i] == '?') ++numBlank;
  }

  int ans = -1;
  if(T == FIND_MAX){
    ans = abs(x) + abs(y) + numBlank;
  }
  else if(T == FIND_MIN){
    int distance = abs(x) + abs(y);
    if((distance - numBlank) % 2 == 0){  // 距離と?の数がともに奇数またはともに偶数のとき
      ans = max(distance - numBlank, 0);
    }
    else{
      ans = max(distance - numBlank, 1);
    }
  }

  cout << ans << endl;
}

