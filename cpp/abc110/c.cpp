/*
 * 競技プログラミング用テンプレート
 * by yuma140902
 * 2019/12/07
 * ver 0.0.0
 */
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

string S, T;
int main(){
  cin >> S >> T;
  vector<char> R('a'+26, ' ');  // S -> T の置換表
  vector<char> Q('a'+26, ' ');  // T -> S の置換表

  bool ans = true;
  rep(i, S.size()){
    if(R[S[i]] < 'a') {
      R[S[i]] = T[i];
    }
    else if(R[S[i]] >= 'a'){
      if(R[S[i]] != T[i]) {
        ans = false;
        break;
      }
    }
    if(Q[T[i]] < 'a'){
      Q[T[i]] = S[i];
    }
    else if(Q[T[i]] >= 'a'){
      if(Q[T[i]] != S[i]){
        ans = false;
        break;
      }
    }
  }

  cerr << "A: ";
  for(char i='a'; i <= 'z'; ++i){
    cerr << i;
    if(i != 'z')
      cerr << ", ";
    else
      cerr << endl;
  }

  cerr << "R: ";
  for(int i='a'; i < R.size(); ++i){
    cerr << R[i];
    if(i != R.size()-1)
      cerr << ", ";
    else
      cerr << endl;
  }

  cerr << "Q: ";
  for(int i='a'; i < Q.size(); ++i){
    cerr << Q[i];
    if(i != Q.size()-1)
      cerr << ", ";
    else
      cerr << endl;
  }

  cout << (ans ? "Yes" : "No") << endl;
}

