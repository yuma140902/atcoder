#include <bits/stdc++.h>
using namespace std;
typedef unsigned long long ll;
double median(vector<ll> v);

int main(){
  int n;
  cin >> n;
  
  vector<ll> a(n);
  vector<ll> b(n);
  ll aSum = 0;
  ll bSum = 0;
  for(int i=0; i<n; ++i) {
    ll j, k;
    cin >> j >> k;
    if(j <= k){
      a[i] = j;
      b[i] = k;
      aSum += j;
      bSum += k;
    }
    else{
      a[i] = k;
      b[i] = j;
      aSum += k;
      bSum += j;
    }
  }
  
  double aIdeal = median(a);
  double bIdeal = median(b);
  double aDiffMin = INFINITY;
  double bDiffMin = INFINITY;
  int beginIdx = -1;
  int endIdx = -1;
  for(int i=0; i<n; ++i) {
    if(abs(a[i] - aIdeal) < aDiffMin) {
      aDiffMin = abs(a[i] - aIdeal);
      beginIdx = i;
    }
    if(abs(b[i] - bIdeal) < bDiffMin) {
      bDiffMin = abs(b[i] - bIdeal);
      endIdx = i;
    }
  }
  
  
  
  ll sum = 0;
  int begin = a[beginIdx];
  int end = b[endIdx];
  ll tmp = abs(end - begin);
  //cout << "begin: " << begin << endl;
  //cout << "end:   " << end << endl;
  for(int i=0; i<n; ++i) {
    ll foo = 0;
    if(a[i] < begin && end < b[i]){
      foo = (begin-a[i])*2 + tmp + (b[i]-end)*2;
      //cout << (begin-a[i])*2 << "+" << tmp << "+" << (b[i]-end)*2 << "  ";
      //cout << "pass a:";
    }
    else if(begin <= a[i] && end < b[i]){
      foo = tmp + (b[i]-end)*2;
      //cout << "pass b:";
    }
    else if(a[i] < begin && b[i] <= end){
      foo = (begin-a[i])*2 + tmp;
      //cout << "pass c:";
    }
    else{
      foo = tmp;
      //cout << "pass d:";
    }
    //cout << "[" << i << "]: " << foo << endl;
    sum += foo;
  }
  
  cout << sum;
}

double median(vector<ll> v) {
  size_t size = v.size();
  ll *t = new ll[size];
  std::copy(v.begin(), v.end(), t);
  std::sort(t, &t[size]);
  double result = size%2 ? t[size/2] : (t[(size/2)-1]+t[size/2])/2;
  delete[] t;
  return result;
}

