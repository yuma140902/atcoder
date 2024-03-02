#include <bits/stdc++.h>
using namespace std;

int m, n, N;

int main(){
  cin >> m >> n >> N;
  int sold = 0;
  sold = N;  /* 最初のN本を売る */
  while(N >= m){  /* 在庫(N)がリサイクル可能な間繰り返す */
    sold += N/m*n;  /* リサイクルしたものを売る */
    N = N/m*n + N%m;  /* リサイクルして売ったもの(`N/m*n`)を回収、リサイクルしきれなかったもの(`N%m`)と合わせて在庫(N)とする */
  }
  cout << sold << endl;
}

